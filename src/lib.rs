use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn get_json(data: &[u8]) -> Result<String, JsValue> {
    let replay = boxcars::ParserBuilder::new(&data)
        .must_parse_network_data()
        .on_error_check_crc()
        .parse()
        .map_err(|err| err.to_string())?;

    let data = subtr_actor::ReplayDataCollector::new()
        .get_replay_data(&replay)
        .unwrap();

    Ok(serde_json::to_string(&data).map_err(|err| err.to_string())?)
}
