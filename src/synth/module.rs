
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
    fn new(mod_out: usize, output: usize, mod_in: usize, input: usize) -> Connection {
        Connection { mod_in, input, mod_out, output }
    }
}

pub struct Rack {
    connections: Vec<Connection>,
    modules: Vec<Box<Module>>
}

impl Rack {
    pub fn new(modules: Vec<Box<Module>>) -> Rack {
        Rack {
            modules,
            connections: Vec::new()
        }
    }

    pub fn connect(&mut self, m: usize, out: usize, m_i: usize, i: usize) {
        // TODO Insert connection at the appropriate spot.
        self.connections.push(Connection::new(m, out, m_i, i));
    }

    pub fn get<'a>(&'a mut self, m: usize) -> &'a mut Module {
        self.modules[m].as_mut()
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

#[derive(Debug)]
#[derive(Clone)]
pub struct DataIn {
    v: Option<Vec<f32>>,
    default: f32
}

impl DataIn {
    pub fn new(default: f32) -> DataIn {
        DataIn { v: None, default }
    }

    pub fn get(&mut self) -> Vec<f32> {
        let v = self.v.take();
        v.unwrap_or(vec![self.default])
    }

    pub fn set(&mut self, v: Vec<f32>) {
        self.v = Some(v);
    }
}

#[derive(Debug)]
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
        let val = self.data[output].get();
        self.data[output].set(val.clone());
        val
    }
}

pub struct Attenuverter {
    attenuation: DataIn,
    signal: DataIn,
}

impl Attenuverter {
    pub fn new() -> Attenuverter {
        Attenuverter { attenuation: DataIn::new(1.0), signal: DataIn::new(0.0) }
    }

    fn attenuvert(val: f32, input: f32) -> f32 {
        // TODO use 2 ^ val instead?
        (val * 2.0 - 0.5) * input
    }

}

impl Module for Attenuverter {
    fn feed(&mut self, input: usize, v: Vec<f32>) {
        match input {
            0 => self.attenuation.set(v),
            1 => self.signal.set(v),
            _ => panic!("Invalid input")
        }
    }

    fn extract(&mut self, output: usize, len: usize) -> Vec<f32> {
        assert_eq!(output, 0);
        let mut val = Vec::with_capacity(len);
        let att = self.attenuation.get();
        let s = self.signal.get();

        let mut ait = att.iter().cycle();
        let mut sit = s.iter().cycle();
        for i in 0..len {
            let a_val = *ait.next().expect("Expected Attenuation");
            let s_val = *sit.next().expect("Expected Signal");
            val.push(Attenuverter::attenuvert(a_val, s_val));
        }
        val
    }
}

