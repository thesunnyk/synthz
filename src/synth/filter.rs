
use std::f32;
use std::cmp;

pub struct BiQuadCoeffs {
    a1: f32,
    a2: f32,
    b0: f32,
    b1: f32,
    b2: f32
}

impl BiQuadCoeffs {
    pub fn normalise(b0: f32, b1: f32, b2: f32, a0: f32, a1: f32, a2: f32) -> BiQuadCoeffs {
        BiQuadCoeffs::new(b0 / a0, b1 / a0, b2 / a0, a1 / a0, a2 / a0)
    }

    pub fn new(b0: f32, b1: f32, b2: f32, a1: f32, a2: f32) -> BiQuadCoeffs {
        BiQuadCoeffs {
            b0: b0,
            b1: b1,
            b2: b2,
            a1: a1,
            a2: a2
        }
    }
}

pub struct BiQuad {
    coeffs: BiQuadCoeffs,
    wn1: f32,
    wn2: f32
}

impl BiQuad {
    pub fn new(coeffs: BiQuadCoeffs) -> BiQuad {
        BiQuad {
            coeffs: coeffs,
            wn1: 0.0,
            wn2: 0.0
        }
    }

    pub fn filter(&mut self, x: f32) -> f32 {
        let wn = x - self.coeffs.a1 * self.wn1 - self.coeffs.a2 * self.wn2;
        let y = self.coeffs.b0 * wn + self.coeffs.b1 * self.wn1 + self.coeffs.b2 * self.wn2;
        // shift delays
        self.wn2 = self.wn1;
        self.wn1 = wn;
        y
    }
}

pub fn lpf(freq: f32, rate: f32) -> BiQuadCoeffs {
    let eat = f32::exp(-(freq * 2.0 * f32::consts::PI)/rate);
    BiQuadCoeffs::new(1.0 - eat, 0.0, 0.0, -eat, 0.0)
}

pub struct Filter {
    quads: Vec<BiQuad>
}

impl Filter {
    pub fn fromCfg(quads: Vec<BiQuadCoeffs>) -> Filter {
        let mut xv = Vec::with_capacity(quads.len());
        for i in quads {
            xv.push(BiQuad::new(i));
        }
        Filter {
            quads: xv
        }
    }

    pub fn filter(&mut self, x: f32) -> f32 {
        self.quads.iter_mut().fold(x, |yn, quad| quad.filter(yn))
    }
}

fn analog_coeffs(n: u32) -> Vec<Vec<f32>> {
    let mut coeffs = Vec::<Vec<f32>>::with_capacity(((n + 1) / 2) as usize);
    for k in 1..n / 2 {
        let frac = (2.0 * k as f32 + n as f32 - 1.0 * f32::consts::PI) / (2.0 * n as f32);
        let mv = -2.0 * f32::cos(frac) + 1.0;
        coeffs.push(vec![1.0, mv, 1.0]);
    }
    if n % 2 > 0 {
        coeffs.push(vec![1.0, 1.0, 0.0]);
    }
    coeffs
}

fn multiply_all_coeffs(c: Vec<Vec<f32>>) -> Vec<f32> {
    c.iter().fold(vec![1.0], |v1, v2| multiply_coeffs(&v1, v2))
}

fn multiply_coeffs(v1: &Vec<f32>, v2: &Vec<f32>) -> Vec<f32> {
    let retVal_size = v1.len() + v2.len() - 1;
    let mut retVal = Vec::<f32>::with_capacity(retVal_size);
    for a in 0..retVal_size {
        retVal.push(0.0);
    }
    for a in 0..v1.len() {
        for b in 0..v2.len() {
            retVal[a + b] = retVal[a + b] + v1[a] * v2[b];
        }
    }
    retVal
}

fn pow(v: Vec<f32>, n: u32) -> Vec<f32> {
    if n == 0 {
        vec![1.0]
    } else {
        let mut bd = v.clone();
        for i in 0..n {
            bd = multiply_coeffs(&bd, &v)
        }
        bd
    }
}

fn sum_vals(v: Vec<Vec<f32>>) -> Vec<f32> {
    v.iter().fold(vec![1.0], |v1, v2| v1.iter().zip(v2.iter()).map(|(x, y)| x + y).collect())
}

fn bilinear_transform(n: Vec<f32>, d: Vec<f32>, alpha: f32) -> (Vec<f32>, Vec<f32>) {
    let max_pow = cmp::max(n.len() - 1, d.len() - 1) as u32;

    let mut co_n = Vec::<Vec<f32>>::with_capacity(n.len());
    let mut offset = 0;
    for kn in n {
        let zco = multiply_coeffs(&pow(vec![1.0, 1.0], max_pow - offset), &pow(vec![-1.0, 1.0], offset));
        co_n.push(multiply_coeffs(&vec![kn * f32::powi(alpha, offset as i32)], &zco));
        offset = offset + 1;
    }
    let res_n = sum_vals(co_n);

    let mut co_d = Vec::<Vec<f32>>::with_capacity(d.len());
    offset = 0;
    for kd in d {
        let zco = multiply_coeffs(&pow(vec![1.0, 1.0], max_pow - offset), &pow(vec![-1.0, 1.0], offset));
        co_d.push(multiply_coeffs(&vec![kd * f32::powi(alpha, offset as i32)], &zco));
        offset = offset + 1;
    }
    let res_d = sum_vals(co_d);

    (res_n, res_d)
}

fn biquads_for(n: Vec<f32>, d: Vec<f32>) -> Vec<BiQuadCoeffs> {
    n.chunks(3).zip(d.chunks(3)).map(|(n, d)| BiQuadCoeffs::normalise(
            *d.get(0).unwrap_or(&0.0),
            *d.get(1).unwrap_or(&0.0),
            *d.get(2).unwrap_or(&0.0),
            *n.get(0).unwrap_or(&0.0),
            *n.get(1).unwrap_or(&0.0),
            *n.get(2).unwrap_or(&0.0),
        )).collect()
}

pub fn butterworth_lpf(order: u32, cutoff: f32, sampling_freq: u32) -> Vec<BiQuadCoeffs> {
    // denominator. Numerator = cutoff.
    let coeffs = multiply_all_coeffs(analog_coeffs(order));

    let alpha = cutoff / f32::tan(cutoff / (2 * sampling_freq) as f32);

    let transfer_fn = bilinear_transform(vec![cutoff], coeffs, alpha);

    biquads_for(transfer_fn.0, transfer_fn.1)
}

// TODO Add Filter and Filter ADSR
// start at cutoff frequency fC,
// A => go to fC + Filter Depth dp
// D => go to fC + dp*Sustain
// S => Stay
// R => back to fC

// TODO Also make the filter do Formants

// TODO Also add reverb

