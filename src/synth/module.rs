
use std::collections::HashMap;

pub trait Module {
    fn connector(&self, name: String) -> usize;
    fn feed(&mut self, input: usize, v: Vec<f32>);
    fn extract(&mut self, output: usize, len: usize) -> Vec<f32>;
}

pub struct ConnectorInfo {
    mod_name: String,
    mod_conn: String
}

impl ConnectorInfo {
    pub fn new(name: &'static str, conn: &'static str) -> ConnectorInfo {
        ConnectorInfo {
            mod_name: String::from(name),
            mod_conn: String::from(conn)
        }
    }
}

pub struct ConnectionInfo {
    conn_in: ConnectorInfo,
    conn_out: ConnectorInfo
}

impl ConnectionInfo {
    pub fn new(conn_out: ConnectorInfo, conn_in: ConnectorInfo) -> ConnectionInfo {
        ConnectionInfo {
            conn_in,
            conn_out
        }
    }
}

pub struct ModuleInfo {
    name: String,
    module: Box<Module>
}

impl ModuleInfo {
    pub fn new(name: &'static str, module: Box<Module>) -> ModuleInfo {
        ModuleInfo { name: String::from(name), module }
    }
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
    pub fn new(mut module_info: Vec<ModuleInfo>, connection_info: Vec<ConnectionInfo>) -> Rack {
        let mut mod_names = HashMap::new();

        module_info.iter().enumerate().for_each(|(i, v)| { mod_names.insert(v.name.clone(), i); });
        let drain_range = 0..module_info.len();
        let modules: Vec<Box<Module>> = module_info.drain(drain_range).map(|val| val.module).collect();

        let connections: Vec<Connection> = connection_info.iter().map(|c| {
            let mod_in_offset = mod_names.get(&c.conn_in.mod_name).unwrap();
            let mod_out_offset = mod_names.get(&c.conn_out.mod_name).unwrap();
            Connection::new(
                *mod_in_offset,
                modules.get(*mod_in_offset).unwrap().connector(c.conn_in.mod_conn.clone()),
                *mod_out_offset,
                modules.get(*mod_out_offset).unwrap().connector(c.conn_out.mod_conn.clone())
            )
        }).collect();

        Rack {
            modules,
            connections
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
    default: f32,
    name: String
}

impl DataIn {
    pub fn new(name: String, default: f32) -> DataIn {
        DataIn { name, v: None, default }
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
    data: Vec<DataIn>
}

impl BufferModule {
    pub fn new(data: Vec<DataIn>) -> BufferModule {
        BufferModule { data }
    }
}

impl Module for BufferModule {
    fn connector(&self, item: String) -> usize {
        self.data.iter().position(|v| v.name == item).unwrap()
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
    signal: DataIn
}

impl Attenuverter {
    pub fn new() -> Attenuverter {
        Attenuverter {
            attenuation: DataIn::new(String::from("attenuation"), 1.0),
            signal: DataIn::new(String::from("signal"), 0.0),
        }
    }

    fn attenuvert(val: f32, input: f32) -> f32 {
        // TODO use 2 ^ val instead?
        (val * 2.0 - 0.5) * input
    }
}

impl Module for Attenuverter {
    fn connector(&self, item: String) -> usize {
        match item.as_str() {
            "attenuation" => 0,
            "signal" => 1,
            "output" => 0,
            _ => panic!("Invalid input")
        }
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

