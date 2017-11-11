
use std::f32;

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

// TODO Add Filter and Filter ADSR
// start at cutoff frequency fC,
// A => go to fC + Filter Depth dp
// D => go to fC + dp*Sustain
// S => Stay
// R => back to fC

// TODO Also make the filter do Formants

// TODO Also add reverb

