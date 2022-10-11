mod utils;

use std::collections::HashMap;
use std::ptr::null;
use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};


#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct Computer {
    memory: HashMap<String, i32>,
    instructions: HashMap<i32, (String, i32)>,
    program_counter: i32,
    program_counter_init: i32,
    current_instruction: (String, String),
    accumulator: i32
}

#[wasm_bindgen]
impl Computer {
    // Constructor
    pub fn new() -> Self {
        let mut memory_default = HashMap::new();
        memory_default.insert("x".to_string(), 27);
        Self {
            memory: memory_default,
            instructions: HashMap::new(),
            program_counter: 100,
            program_counter_init: 99,
            current_instruction: ("LOAD".to_string(), "x".to_string()),
            accumulator: 0
        }
    }



    pub fn memory_add(&mut self, ptr: String, v: i32) {
        self.memory.insert(ptr, v);
    }

    pub fn memory_read(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.memory).unwrap()
    }


    pub fn instructions_add(&mut self, instruction: String, ptr: i32) {
        self.program_counter_init += 1;
        let v = (instruction, ptr);
        self.instructions.insert(self.program_counter_init, v);
    }

    pub fn instructions_read(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.instructions).unwrap()
    }

    pub fn load(&mut self) {
        self.accumulator = self.memory[&self.current_instruction.1];
    }

    pub fn accumulator(&self) -> i32 {
        self.accumulator
    }

    pub fn input(&self, s: String) -> JsValue {
        let res: Vec<String> = s.split_whitespace().map(|s| s.to_string()).collect();
        serde_wasm_bindgen::to_value(&res).unwrap()
    }
}