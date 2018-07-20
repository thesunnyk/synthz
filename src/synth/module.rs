
use std::collections::HashMap;
use std::iter::Cycle;

pub trait Module {
    // TODO Connector should be part of builder pattern / metadata
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
                *mod_out_offset,
                modules.get(*mod_out_offset).unwrap().connector(c.conn_out.mod_conn.clone()),
                *mod_in_offset,
                modules.get(*mod_in_offset).unwrap().connector(c.conn_in.mod_conn.clone())
            )
        }).collect();

        Rack {
            modules,
            connections
        }
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

pub trait MisoWorker {
    fn get_data(&self) -> Vec<DataIn>;
    fn extract(&mut self, vals: &[f32]) -> f32;
}

pub struct MisoModule<T: MisoWorker> {
    data: Vec<DataIn>,
    worker: T
}

impl <T: MisoWorker> MisoModule<T> {
    pub fn new(worker: T) -> MisoModule<T> {
        MisoModule {
            data: worker.get_data(),
            worker
        }
    }
}

impl <T: MisoWorker> Module for MisoModule<T> {
    fn connector(&self, name: String) -> usize {
        match (self.data.iter().position(|v| v.name == name)) {
            Some(i) => i,
            None => {
                assert!(name == String::from("output"));
                0
            }
        }
    }

    fn feed(&mut self, input: usize, v: Vec<f32>) {
        self.data[input].set(v)
    }

    fn extract(&mut self, output: usize, len: usize) -> Vec<f32> {
        assert_eq!(output, 0);
        assert!(self.data.len() <= 10);
        let mut val = Vec::with_capacity(len);
        let mut vecs: Vec<Vec<f32>> = self.data.iter_mut().map(|d| d.get()).collect();
        let mut cycles = Vec::with_capacity(vecs.len());
        for item in vecs.iter() {
            cycles.push(item.iter().cycle());
        }

        let mut inputs: [f32; 10] = [0.0; 10];
        for i in 0..len {
            for j in 0..cycles.len() {
                inputs[j] = *cycles[j].next().unwrap();
            }
            val.push(self.worker.extract(&inputs));
        }
        val
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
}

impl Attenuverter {
    pub fn new() -> MisoModule<Attenuverter> {
        MisoModule::new(Attenuverter { })
    }

    fn attenuvert(val: f32, input: f32) -> f32 {
        // TODO use 2 ^ val instead?
        (val * 2.0 - 0.5) * input
    }
}

impl MisoWorker for Attenuverter {
    fn get_data(&self) -> Vec<DataIn> {
        vec![
            DataIn::new(String::from("attenuation"), 1.0),
            DataIn::new(String::from("signal"), 0.0),
        ]
    }

    fn extract(&mut self, vals: &[f32]) -> f32 {
        Attenuverter::attenuvert(vals[0], vals[1])
    }
}

