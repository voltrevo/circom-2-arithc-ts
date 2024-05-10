use vfs::{FileSystem, VfsError};
use circom_2_arithc::{circom::input::Input, program::build_circuit};
use vfs::MemoryFS;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn compile(main: &str) -> String {
    match compile_impl(main) {
        Ok(result) => result,
        Err(e) => e.to_string(),
    }
}

fn compile_impl(main: &str) -> Result<String, VfsError> {
    let fs = MemoryFS::new();
    // fs.create_dir("/src")?;
    // fs.create_file("/src/main.circom")?.write_all(main.as_bytes())?;

    return Ok("test".into());

    // match build_circuit(&fs, &Input::new("/src/main.circom", "/output", None)) {
    //     Ok(circuit) => serde_json::to_string(&circuit).unwrap(),
    //     Err(e) => e.to_string(),
    // }
}
