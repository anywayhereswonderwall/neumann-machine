mod utils;

use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};


#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct Computer {
    memory: HashMap<String, i32>,
    instructions: HashMap<i32, (String, String)>,
    program_counter: i32,
    program_counter_init: i32,
    current_instruction: (String, String),
    memory_address_register: i32,
    memory_data_register: (String, String),
    accumulator: i32,
    alu: (i32, i32)
}

#[wasm_bindgen]
impl Computer {

    pub fn new() -> Self {
        Self {
            memory: HashMap::new(),
            instructions: HashMap::new(),
            program_counter: 100,
            program_counter_init: 99,
            current_instruction: ("".to_string(), "".to_string()),
            memory_address_register: 0,
            memory_data_register: ("".to_string(), "".to_string()),
            accumulator: 0,
            alu: (0, 0)
        }
    }



    pub fn memory_add(&mut self, ptr: String, v: i32) {
        self.memory.insert(ptr, v);
    }

    pub fn memory_read(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.memory).unwrap()
    }

    pub fn instructions_add(&mut self, instruction: String, ptr: String) {
        self.program_counter_init += 1;
        let v = (instruction, ptr);
        self.instructions.insert(self.program_counter_init, v);
    }

    pub fn instructions_read(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.instructions).unwrap()
    }

    pub fn counter_get(&self) -> i32 {
        self.program_counter
    }

    pub fn mar_get(&self) -> i32 {
        self.memory_address_register
    }

    pub fn mdr_get(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.memory_data_register).unwrap()
    }

    pub fn cir_get(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.current_instruction).unwrap()
    }

    pub fn accumulator_get(&self) -> i32 {
        self.accumulator
    }

    pub fn alu_get(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.alu).unwrap()
    }

    pub fn input(&mut self, s: String)  {
        let res: Vec<String> = s.split_whitespace().map(|s| s.to_string()).collect();
        if res.contains(&"=".to_string()) && res.len() == 3 {
            let ptr = res[0].clone();
            let val = res[2].parse::<i32>().unwrap();
            self.memory_add(ptr, val);
        } else {
            let instruction = res[0].clone();
            let ptr = res[1].clone();
            self.instructions_add(instruction, ptr);
        }
    }

    pub fn step(&mut self) {
        self.memory_address_register = self.program_counter;
        self.memory_data_register = self.instructions[&self.program_counter].clone();
        self.current_instruction = self.memory_data_register.clone();
        self.program_counter += 1;
        match *&self.current_instruction.0.as_str() {
            "LOAD" => {
                self.alu = (0, 0);
                self.accumulator = self.memory[&self.current_instruction.1];
            },
            "STORE" => {
                self.alu = (0, 0);
                self.memory_add(self.current_instruction.1.clone(), self.accumulator);
            },
            "ADD" => {
                self.alu = (self.accumulator, self.memory[&self.current_instruction.1]);
                self.accumulator += self.memory[&self.current_instruction.1];
            },
            _ => {}
        }
    }

}