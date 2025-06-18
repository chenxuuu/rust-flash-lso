use flash_lso::{extra::flex, read::Reader, types::Lso, write::Writer};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn json2sol(input: String) -> Result<Vec<u8>, JsValue> {
    let mut lso: Lso = match serde_json::from_str(&input) {
        Ok(lso) => lso,
        Err(e) => return Err(JsValue::from_str(&format!("Failed to parse JSON: {}", e))),
    };
    let mut buffer = Vec::new();
    let mut s = Writer::default();
    flex::write::register_encoders(&mut s.amf3_encoder);
    match s.write_full(&mut buffer, &mut lso) {
        Ok(_) => {}
        Err(e) => return Err(JsValue::from_str(&format!("Failed to write LSO: {}", e))),
    }
    Ok(buffer)
}

#[wasm_bindgen]
pub fn sol2json(input: Vec<u8>) -> Result<String, JsValue> {
    let mut d = Reader::default();
    flex::read::register_decoders(&mut d.amf3_decoder);
    let lso = match d.parse(input.as_slice()) {
        Ok(lso) => lso,
        Err(e) => return Err(JsValue::from_str(&format!("Failed to parse LSO: {}", e))),
    };
    match serde_json::to_string(&lso) {
        Ok(json) => Ok(json),
        Err(e) => Err(JsValue::from_str(&format!("Failed to convert LSO to JSON: {}", e))),
    }
}