use circom_2_arithc::program::build_circuit_pure;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn compile(main: &str) -> String {
    match build_circuit_pure(main, |_| "".into()) {
        Ok(circuit) => serde_json::to_string(&circuit).unwrap(),
        Err(e) => e.to_string(),
    }
}
