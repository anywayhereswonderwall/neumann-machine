mod utils;

use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};


#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct Computer {
    memory: HashMap<String, i32>
}

#[wasm_bindgen]
impl Computer {
    // Constructor
    pub fn new() -> Self {
        Self {
            memory: HashMap::new()
        }
    }

    pub fn memory_add(&mut self, s: String, v: i32) {
        self.memory.insert(s, v);
    }

    pub fn memory_read(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.memory).unwrap()
    }
}