
use synth::module;
use std::f32;

#[derive(Debug)]
#[derive(Clone)]
pub struct Envelope {
    rate: f32,
    t: f32,
    t_trig: f32,
    n_trig_1: bool,
    a: module::DataIn,
    d: module::DataIn,
    s: module::DataIn,
    r: module::DataIn,
    signal: module::DataIn,
    trigger: module::DataIn
}

impl Envelope {
    pub fn new(rate: f32) -> Envelope {
        Envelope {
            rate,
            t: 0.0,
            n_trig_1: true,
            t_trig: 0.0,
            a: module::DataIn::new(0.1),
            d: module::DataIn::new(1.0),
            s: module::DataIn::new(1.0),
            r: module::DataIn::new(0.1),
            signal: module::DataIn::new(0.0),
            trigger: module::DataIn::new(0.0)
        }
    }

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
            let et = self.t_trig;
            let er = et + r;
            if rt < er {
                s * (er - rt) / r
            } else {
                0.0
            }
        };
        self.t = self.t + 1.0;
        self.n_trig_1 = trig > 0.5;

        sig_n * ((2.0 as f32).powf(env) - 1.0)
    }

}

impl module::Module for Envelope {
    fn feed(&mut self, input: usize, v: Vec<f32>) {
        match input {
            0 => self.a.set(v),
            1 => self.d.set(v),
            2 => self.s.set(v),
            3 => self.r.set(v),
            4 => self.signal.set(v),
            5 => self.trigger.set(v),
            _ => panic!("Doesn't match any known value")
        }
    }

    fn extract(&mut self, output: usize, len: usize) -> Vec<f32> {
        assert!(output == 0);
        let a = self.a.get();
        let d = self.d.get();
        let s = self.s.get();
        let r = self.r.get();
        let trigger = self.trigger.get();
        let signal = self.signal.get();

        let mut a_it = a.iter().cycle();
        let mut d_it = d.iter().cycle();
        let mut s_it = s.iter().cycle();
        let mut r_it = r.iter().cycle();
        let mut t_it = trigger.iter().cycle();
        let mut sig_it = signal.iter().cycle();

        let mut ret = Vec::<f32>::with_capacity(len);
        for i in 0..len {
            let an: f32 = *a_it.next().expect("Expected more data");
            let dn: f32 = *d_it.next().expect("Expected more data");
            let sn: f32 = *s_it.next().expect("Expected more data");
            let rn: f32 = *r_it.next().expect("Expected more data");
            let sig_n: f32 = *sig_it.next().expect("Expected more data");
            let t_n: f32 = *t_it.next().expect("Expected more data");

            ret.push(self.envelope(an, dn, sn, rn, t_n, sig_n));
        }

        ret
    }
}
