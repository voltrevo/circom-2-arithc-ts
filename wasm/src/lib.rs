use std::collections::HashMap;

use circom_2_arithc::{
    circom::input::Input,
    circuit::ArithmeticCircuit,
    program::build_circuit,
};
use circom_virtual_fs::{FileSystem, MemoryFs};
use wasm_bindgen::{prelude::*, JsError};
use web_sys::js_sys::{Array, Object, Reflect, Uint32Array};

#[wasm_bindgen]
pub fn compile(files: JsValue) -> Result<String, JsError> {
    Ok(serde_json::to_string(&compile_impl(files)?)?)
}

#[wasm_bindgen]
pub fn eval_(circuit_json: JsValue, inputs: JsValue) -> Result<Vec<u32>, JsError> {
    let circuit: ArithmeticCircuit = serde_json::from_str(&circuit_json.as_string().unwrap_or_default())
        .map_err(|_| JsError::new("Invalid circuit JSON"))?;

    let inputs = inputs.dyn_into::<Uint32Array>()
        .map_err(|_| JsError::new("Expected a Uint32Array"))?
        .to_vec();

    let sim_circuit = circuit.build_sim_circuit()?;
    let outputs = sim_circuit.execute(&inputs)?;

    Ok(outputs)
}

fn compile_impl(files: JsValue) -> Result<ArithmeticCircuit, JsError> {
    let mut fs = MemoryFs::new("/src".into());

    for (name, content) in convert_jsvalue_to_hashmap(files)? {
        fs.write(&name.into(), content.as_bytes())?;
    }

    let circuit = build_circuit(&mut fs, &Input::new("/src/main.circom", "/output", None))?;

    Ok(circuit)
}

pub fn convert_jsvalue_to_hashmap(value: JsValue) -> Result<HashMap<String, String>, JsError> {
    let object = value
        .dyn_into::<Object>()
        .map_err(|_| JsError::new("Input is not a valid object"))?;

    let keys = Object::keys(&object);
    let mut map = HashMap::new();

    for key in Array::from(&keys).iter() {
        let key_str = key.as_string().ok_or(JsError::new("Key is not a string"))?;
        let value = Reflect::get(&object, &key).map_err(|e| {
            JsError::new(&format!(
                "Error accessing property: {}",
                e.as_string().unwrap_or_default()
            ))
        })?;
        let value_str = value
            .as_string()
            .ok_or(JsError::new("Value is not a string"))?;

        map.insert(key_str, value_str);
    }

    Ok(map)
}

pub fn convert_jsvalue_to_inputs(value: JsValue) -> Result<Vec<u32>, JsError> {
    let array = value
        .dyn_into::<Array>()
        .map_err(|_| JsError::new("Input is not a valid array"))?;

    let mut inputs = Vec::new();

    for item in array.iter() {
        let item = item.as_f64().ok_or(JsError::new("Item is not a number"))?;
        inputs.push(item as u32);
    }

    Ok(inputs)
}
