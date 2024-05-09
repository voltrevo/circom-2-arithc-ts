use circom_2_arithc::{circuit::ArithmeticCircuit, program::{build_circuit_pure, ProgramError}};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn add(a: u32, b: u32) -> u32 {
    a + b
}

#[wasm_bindgen]
pub fn compile(main: &str) -> u32 {
    match build_circuit_pure(main, |_| "".into()) {
        Ok(_circuit) => 0,
        Err(_e) => 1,
    }
}

#[wasm_bindgen]
pub fn test_string_output() -> String {
    "test string".into()
}
