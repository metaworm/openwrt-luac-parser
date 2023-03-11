use wasm_bindgen::prelude::wasm_bindgen;

#[cfg(target_family = "wasm")]
#[wasm_bindgen]
pub fn parse(luac: &[u8]) -> Result<Vec<u8>, String> {
    openwrt_luac_parser::parse_(luac)
        .map_err(|e| e.to_string())?
        .to_msgpack()
        .map_err(|e| e.to_string())
}

fn main() {}
