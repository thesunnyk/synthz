

trait Module {
    fn input(i: f32) -> &mut Input;
    fn output(i: f32) -> &mut Output;
}

trait Input {
    fn feed(&mut self, v: &Vec<f32>);
}

trait Output {
    fn connect(&mut self, i: &mut Input);
    fn feed(&mut self);
}

struct Rack {
    modules: Vec<Box<Module>>
}

impl Rack {
    fn connect(&mut self, m: usize, out: usize, m_i: usize, i: usize) {
        self.modules[m].output(out).connect(self.modules[m_i].input(i));
    }

    fn feed_all() {
        // TODO Feed in order of connections
        modules.iter_mut().for_each(|x| x.feed());
    }
}

