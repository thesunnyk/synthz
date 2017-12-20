

pub trait Module {
    fn feed(&mut self, input: usize, v: Vec<f32>);
    fn extract(&mut self, output: usize, len: usize) -> Vec<f32>;
}

struct Connection {
    mod_in: usize,
    input: usize,
    mod_out: usize,
    output: usize
}

impl Connection {
    fn new(mod_in: usize, input: usize, mod_out: usize, output: usize) -> Connection {
        Connection { mod_in, input, mod_out, output }
    }
}

pub struct Rack {
    connections: Vec<Connection>,
    modules: Vec<Box<Module>>
}

impl Rack {
    pub fn connect(&mut self, m: usize, out: usize, m_i: usize, i: usize) {
        // TODO Insert connection at the appropriate spot.
        self.connections.push(Connection::new(m, out, m_i, i));
    }

    pub fn feed_all(&mut self, len: usize) {
        for c in &self.connections {
            let out = {
                let mut mod_out = self.modules[c.mod_out].as_mut();
                mod_out.extract(c.output, len)
            };

            let mut mod_in = self.modules[c.mod_in].as_mut();
            mod_in.feed(c.input, out)
        }
    }
}

