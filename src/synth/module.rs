
pub trait Module {
    fn initialise(&mut self, pos: usize);
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

pub struct Connector {
    mod_in: usize,
    offset: usize
}

pub struct Rack {
    connections: Vec<Connection>,
    modules: Vec<Box<Module>>
}

// TODO Higher level connection concept

// TODO Module with standard connectors.

impl Rack {
    pub fn new(modules: Vec<Box<Module>>) -> Rack {
        moules.iter().enumerate().for_each(|i, val| val.initialise(i));
        Rack {
            modules,
            connections: Vec::new()
        }
    }

    pub fn connect(&mut self, output: Connector, input: Connector) {
        self.connect_direct(output.mod_in, output.offset, input.mod_in, input.offset)
    }

    pub fn connect_direct(&mut self, m: usize, out: usize, m_i: usize, i: usize) {
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
    pos: usize,
}

impl BufferModule {
    pub fn new(data: Vec<DataIn>) -> BufferModule {
        BufferModule { data, pos: 0 }
    }

    pub fn connector(&self, item: usize) -> Connector {
        assert!(item < self.data.len());

        Connector {
            mod_in: self.pos,
            offset: item
        }
    }
}

impl Module for BufferModule {
    fn initialise(&mut self, pos: usize) {
        self.pos = pos;
    }

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
    pos: usize
}

pub enum AttenuverterInput { ATTENUATION, SIGNAL }

impl Attenuverter {
    pub fn new() -> Attenuverter {
        Attenuverter {
            attenuation: DataIn::new(1.0),
            signal: DataIn::new(0.0),
            pos: 0
        }
    }

    fn attenuvert(val: f32, input: f32) -> f32 {
        // TODO use 2 ^ val instead?
        (val * 2.0 - 0.5) * input
    }

    fn connector_in(&self, item: AttenuverterInput) -> Connector {
        Connector {
            mod_in: self.pos,
            offset: item as usize
        }
    }

    fn connector_out(&self) -> Connector {
        Connector {
            mod_in: self.pos,
            offset: 0
        }
    }
}

impl Module for Attenuverter {
    fn initialise(&mut self, pos: usize) {
        self.pos = pos;
    }

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

