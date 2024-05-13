use circom_2_arithc::{circom::input::Input, program::build_circuit};
use circom_virtual_fs::{FileSystem, FsResult, MemoryFs};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn compile(main: &str) -> String {
    match compile_impl(main) {
        Ok(result) => result,
        Err(e) => e.to_string(),
    }
}

fn compile_impl(main: &str) -> FsResult<String> {
    let mut fs = MemoryFs::new("/home".into());
    fs.write(&"/src/main.circom".into(), main.as_bytes())?;

    Ok(match build_circuit(&mut fs, &Input::new("/src/main.circom", "/output", None)) {
        Ok(circuit) => serde_json::to_string(&circuit).unwrap(),
        Err(e) => e.to_string(),
    })
}
