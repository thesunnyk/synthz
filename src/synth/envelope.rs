
use synth::module;
use std::f32;

#[derive(Debug)]
#[derive(Clone)]
pub struct Envelope {
    rate: f32,
    t: f32,
    t_trig: f32,
    n_trig_1: bool,
}

impl Envelope {
    pub fn new(rate: f32) -> module::MisoModule<Envelope> {
        module::MisoModule::new(Envelope {
            rate,
            t: 0.0,
            n_trig_1: true,
            t_trig: 0.0,
        })
    }

    // TODO ADSR is buggy. Should smoothly transition.
    fn envelope(&mut self, ar: f32, dr: f32, s: f32, rr: f32, trig: f32, sig_n: f32) -> f32 {
        let a = ar * self.rate * 10.0;
        let d = dr * self.rate * 10.0;
        let r = rr * self.rate * 10.0;

        if (self.n_trig_1 && trig < 0.5) || (!self.n_trig_1 && trig > 0.5) {
            self.t_trig = self.t;
        }
        let rt = self.t - self.t_trig;
        let env = if trig > 0.5 {
            let ad = a + d;
            if rt < a {
                rt / a
            } else if rt < ad {
                (1.0 - s) * ((ad - rt) / d) + s
            } else {
                s
            }
        } else {
            if rt < r {
                s * (r - rt) / r
            } else {
                0.0
            }
        };
        self.t = self.t + 1.0;
        self.n_trig_1 = trig > 0.5;

        sig_n * ((2.0 as f32).powf(env) - 1.0)
    }

}

impl module::MisoWorker for Envelope {
    fn get_data(&self) -> Vec<module::DataIn> {
        vec![
            module::DataIn::new(String::from("attack"), 0.1),
            module::DataIn::new(String::from("decay"), 1.0),
            module::DataIn::new(String::from("sustain"), 1.0),
            module::DataIn::new(String::from("release"), 0.1),
            module::DataIn::new(String::from("trigger"), 0.0),
            module::DataIn::new(String::from("signal"), 0.0),
        ]
    }

    fn extract(&mut self, vals: Vec<f32>) -> f32 {
        self.envelope(vals[0], vals[1], vals[2], vals[3], vals[4], vals[5])
    }
}
