use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata {
    font_name: String,
    font_version: String,
    engraving_defaults: HashMap<String, f32>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BBox {
    b_box_n_e: (f32, f32),
    b_box_s_w: (f32, f32),
}
