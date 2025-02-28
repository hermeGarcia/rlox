use rlox_ast::StrId;
use std::collections::HashMap;

use crate::value_system::Value;

pub type MemAddr = usize;

const MEMORY_SIZE: usize = 4_000 / std::mem::size_of::<Value>();

#[derive(Default)]
struct Env {
    start: usize,
    inner: HashMap<StrId, MemAddr>,
}

impl Env {
    pub fn get(&self, id: StrId) -> Option<MemAddr> {
        self.inner.get(&id).copied()
    }

    pub fn insert(&mut self, id: StrId, value: MemAddr) {
        self.inner.insert(id, value);
    }
}

pub struct Runtime {
    free_address: usize,
    var_env: Vec<Env>,
    pub memory: Vec<Value>,
}

impl Runtime {
    pub fn new() -> Runtime {
        Runtime {
            free_address: 0,
            memory: vec![Value::Nil; MEMORY_SIZE],
            var_env: vec![Env::default()],
        }
    }

    pub fn address(&self, id: StrId) -> Option<MemAddr> {
        for env in self.var_env.iter().rev() {
            if let Some(value) = env.get(id) {
                return Some(value);
            }
        }

        None
    }

    pub fn insert(&mut self, id: StrId, value: Value) -> MemAddr {
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
        let Some(env) = self.var_env.pop() else {
            panic!("Scoping was not properly managed")
        };

        self.free_address = env.start;
    }
}
