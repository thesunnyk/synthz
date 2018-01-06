
use std::rc::Rc;

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
    modules: Vec<Rc<Module>>
}

impl Rack {
    pub fn new(modules: Vec<Rc<Module>>) -> Rack {
        Rack {
            modules,
            connections: Vec::new()
        }
    }

    pub fn connect(&mut self, m: usize, out: usize, m_i: usize, i: usize) {
        // TODO Insert connection at the appropriate spot.
        self.connections.push(Connection::new(m, out, m_i, i));
    }

    pub fn feed_all(&mut self, len: usize) {
        for c in &self.connections {
            let out = {
                let mut mod_out = Rc::get_mut(&mut self.modules[c.mod_out]).unwrap();
                mod_out.extract(c.output, len)
            };

            let mut mod_in = Rc::get_mut(&mut self.modules[c.mod_in]).unwrap();
            mod_in.feed(c.input, out)
        }
    }
}

#[derive(Debug)]
#[derive(Clone)]
pub struct DataIn {
    v: Option<Vec<f32>>,
    default: f32
}

impl DataIn {
    pub fn new(default: f32) -> DataIn {
        DataIn { v: None, default: default }
    }

    pub fn get(&mut self) -> Vec<f32> {
        let v = self.v.take();
        v.unwrap_or(vec![self.default])
    }

    pub fn set(&mut self, v: Vec<f32>) {
        self.v = Some(v);
    }
}


pub struct BufferModule {
    data: Vec<DataIn>,
}

impl BufferModule {
    pub fn new(data: Vec<DataIn>) -> BufferModule {
        BufferModule { data }
    }
}

impl Module for BufferModule {
    fn feed(&mut self, input: usize, v: Vec<f32>) {
        self.data[input].set(v)
    }

    fn extract(&mut self, output: usize, len: usize) -> Vec<f32> {
        // TODO cycle and create a new vector.
        self.data[output].get()
    }
}

