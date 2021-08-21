use wasm_bindgen::prelude::*;

use crate::cpu::Cpu;
use crate::display::Display;
use crate::keypad::Keypad;

let mut CPU: Cpu = Cpu {
    i: 0,
    pc: 0,
    memory: vec![0; 4096],
    v: vec![0; 16],
    keypad: Keypad::new(),
    display: Display::new(),
    stack: vec![0; 16],
    sp: 0,
    dt: 0,
    st: 0
};

/*
#[wasm_bindgen]
pub fn reset() {
CPU.reset();
}
#[wasm_bindgen]
pub fn get_memory() -> &'static [u8; 4096] {
    unsafe {
        &CPU.memory
    }
}
*/

#[wasm_bindgen]
pub fn get_memory() -> JsValue {
    JsValue::from_serde(&CPU.memory).unwrap()
}
/*
#[wasm_bindgen]
pub fn get_display() -> &'static [u8; 2048] {
    &CPU.display.memory
}

#[wasm_bindgen]
pub fn key_down(i: u8) {
    CPU.keypad.key_down(i);
}

#[wasm_bindgenpub fn key_up(i: u8) {
    CPU.keypad.key_up(i);
}#[wasm_bindgen]
pub fn get_registers() -> &'static [u8; 16] {
    &CPU.v

#[wasm_bindgen]
pub fn get_index_register() -> u16 {
    CPU.i
}

#[wasm_bindgen]
pub fn get_program_counter() -> u16 {
    CPU.pc
}

#[wasm_bindgen]
pub fn execute_cycle() {
    CPU.execute_cycle();
}
*/
