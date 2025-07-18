use std::collections::HashMap;

use crate::native_functions;
use crate::value_system::Value;

pub type MemAddr = usize;

const MEMORY_SIZE: usize = 4_000 / std::mem::size_of::<Value>();

#[derive(Default)]
struct Env<'a> {
    start: usize,
    inner: HashMap<&'a str, MemAddr>,
}

impl<'a> Env<'a> {
    pub fn get(&self, id: &str) -> Option<MemAddr> {
        self.inner.get(id).copied()
    }

    pub fn insert(&mut self, id: &'a str, value: MemAddr) {
        self.inner.insert(id, value);
    }
}

pub struct Runtime<'a> {
    free_address: usize,
    var_env: Vec<Env<'a>>,
    pub memory: Vec<Value>,
}

impl Default for Runtime<'_> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> Runtime<'a> {
    pub fn new() -> Runtime<'a> {
        let mut runtime = Runtime {
            free_address: 0,
            memory: vec![Value::Nil; MEMORY_SIZE],
            var_env: vec![Env::default()],
        };

        for native_fn in native_functions::REGISTRY {
            runtime.insert(native_fn.name, Value::Fn(*native_fn));
        }

        runtime
    }

    pub fn address(&self, id: &str) -> Option<MemAddr> {
        for env in self.var_env.iter().rev() {
            if let Some(value) = env.get(id) {
                return Some(value);
            }
        }

        None
    }

    pub fn insert(&mut self, id: &'a str, value: Value) -> MemAddr {
        if self.free_address == self.memory.len() {
            self.memory.extend((0..MEMORY_SIZE).map(|_| Value::Nil));
        }

        let address = self.free_address;
        let current_env = self.var_env.len() - 1;

        self.memory[address] = value;
        self.var_env[current_env].insert(id, address);
        self.free_address += 1;

        address
    }

    pub fn deref(&self, address: MemAddr) -> &Value {
        &self.memory[address]
    }

    pub fn enter_block(&mut self) {
        self.var_env.push(Env {
            start: self.free_address,
            ..Default::default()
        });
    }

    pub fn leave_block(&mut self) {
        // Unwrapping should be safe since a valid lox
        // program must have balanced braces.
        let env = self.var_env.pop().unwrap();

        self.free_address = env.start;
    }
}
