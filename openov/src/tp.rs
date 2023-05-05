use std::collections::HashMap;

use serde::Deserialize;

use crate::{pass::Pass, stop::Stop};

#[derive(Deserialize, Debug)]
pub struct TimingPoint {
    #[serde(rename = "Stop")]
    pub stop: Stop,
    #[serde(rename = "Passes")]
    pub passes: HashMap<String, Pass>,
}

#[derive(Deserialize, Debug)]
pub struct TimingPointResponse {
    #[serde(flatten)]
    pub tps: HashMap<String, TimingPoint>,
}
