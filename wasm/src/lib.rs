use circom_2_arithc::{circom::input::Input, program::{build_circuit, ProgramError}};
use circom_virtual_fs::{FileSystem, MemoryFs};
use wasm_bindgen::prelude::*;
use web_sys::js_sys::Error;

#[wasm_bindgen]
pub fn compile(main: &str) -> Result<String, JsValue> {
    compile_impl(main).map_err(to_js_err)
}

fn to_js_err<T: std::fmt::Display>(e: T) -> JsValue {
    Error::new(&e.to_string()).into()
}

fn compile_impl(main: &str) -> Result<String, ProgramError> {
    let mut fs = MemoryFs::new("/home".into());
    fs.write(&"/src/main.circom".into(), main.as_bytes())?;

    let circuit = build_circuit(
        &mut fs,
        &Input::new("/src/main.circom", "/output", None),
    )?;

    Ok(serde_json::to_string(&circuit)?)
}
