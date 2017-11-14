
use std::f32;
use std::cmp;

#[derive(Debug)]
pub struct BiQuadCoeffs {
    a1: f32,
    a2: f32,
    b0: f32,
    b1: f32,
    b2: f32
}

impl BiQuadCoeffs {
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

#[derive(Debug)]
struct AnalogBiQuadCoeffs {
    a0: f32,
    a1: f32,
    a2: f32,
    b0: f32,
    b1: f32,
    b2: f32
}

impl AnalogBiQuadCoeffs {
    pub fn new(b0: f32, b1: f32, b2: f32, a0: f32, a1: f32, a2: f32) -> AnalogBiQuadCoeffs {
        AnalogBiQuadCoeffs {
            b0: b0,
            b1: b1,
            b2: b2,
            a0: a0,
            a1: a1,
            a2: a2
        }
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

fn analog_coeffs(n: u32) -> Vec<AnalogBiQuadCoeffs> {
    let mut coeffs = Vec::<AnalogBiQuadCoeffs>::with_capacity(((n + 1) / 2) as usize);
    for k in 0..n / 2 {
        let frac = (2.0 * (k + 1) as f32 + n as f32 - 1.0) * f32::consts::PI / (2.0 * n as f32);
        let mv = -2.0 * f32::cos(frac) + 1.0;
        coeffs.push(AnalogBiQuadCoeffs::new(0.0, 0.0, 1.0 / f32::sqrt(2.0), 1.0, mv, 1.0));
    }
    if n % 2 > 0 {
        coeffs.push(AnalogBiQuadCoeffs::new(0.0, 0.0, 1.0 / f32::sqrt(2.0), 0.0, 1.0, 1.0));
    }
    coeffs
}

fn digitise_biquad(c: &AnalogBiQuadCoeffs, warp: f32) -> BiQuadCoeffs {
    let warp_sq = f32::powi(warp, 2);
    let a0 = c.a0 * warp_sq + c.a1 * warp + c.a2;
    let b0 = (c.b0 * warp_sq + c.b1 * warp + c.b2) / a0;
    let b1 = (2.0 * c.b2 - 2.0 * c.b0 * warp_sq) / a0;
    let b2 = (c.b0 * warp_sq - c.b1 * warp + c.b2) / a0;
    let a1 = (2.0 * c.a2 - 2.0 * c.a0 * warp_sq) / a0;
    let a2 = (c.a0 * warp_sq - c.a1 * warp + c.a2) / a0;
    BiQuadCoeffs::new(b0, b1, b2, a1, a2)
}

pub fn butterworth_lpf(order: u32, cutoff: f32, sampling_freq: f32) -> Vec<BiQuadCoeffs> {
    // denominator. Numerator = cutoff.
    let coeffs = analog_coeffs(order);

    println!("Synth Coeffs: {:?}", coeffs);

    let warp = 1.0 / f32::tan(cutoff / (2.0 * sampling_freq));
    println!("Synth Warp: {:?}", warp);

    let biquads = coeffs.iter().map(|c| digitise_biquad(c, warp)).collect();
    println!("Synth Biquads: {:?}", biquads);

    biquads
}

// TODO Add Filter and Filter ADSR
// start at cutoff frequency fC,
// A => go to fC + Filter Depth dp
// D => go to fC + dp*Sustain
// S => Stay
// R => back to fC

// TODO Also make the filter do Formants

// TODO Also add reverb

