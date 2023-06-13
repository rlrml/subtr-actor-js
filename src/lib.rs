use std::path::Path;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn get_json(filepath: &str) -> Result<String, JsValue> {
    let path = Path::new(filepath);
    let data = std::fs::read(path).map_err(|err| err.to_string())?;

    let replay = boxcars::ParserBuilder::new(&data)
        .must_parse_network_data()
        .on_error_check_crc()
        .parse()
        .map_err(|err| err.to_string())?;

    let data = subtr_actor::ReplayDataCollector::new()
        .get_replay_data(&replay)
        .map_err(|_e| "fuck".to_string())?;

    Ok(serde_json::to_string(&data).map_err(|err| err.to_string())?)
}
