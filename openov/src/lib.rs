use std::collections::HashMap;

use serde::Deserialize;

pub type TimingPoints = HashMap<String, TimingPoint>;

pub type Passes = HashMap<String, Pass>;

#[derive(Deserialize, Debug)]
pub struct TimingPointResponse {
    #[serde(flatten)]
    pub tps: TimingPoints,
}

#[derive(Deserialize, Debug)]
pub struct TimingPoint {
    #[serde(rename = "Stop")]
    pub stop: Stop,
    #[serde(rename = "Passes")]
    pub passes: Passes,
}

#[derive(Deserialize, Debug)]
pub enum Accessibility {
    #[serde(rename = "ACCESSIBLE")]
    Accessible,
}

#[derive(Deserialize, Debug)]
pub struct Stop {
    #[serde(rename = "Latitude")]
    pub lat: f32,
    #[serde(rename = "Longitude")]
    pub long: f32,
    #[serde(rename = "TimingPointTown")]
    pub town: String,
    #[serde(rename = "TimingPointName")]
    pub tpc_name: String,
    #[serde(rename = "TimingPointCode")]
    pub tpc_code: String,
    #[serde(rename = "StopAreaCode")]
    pub stop_area_code: String,
    #[serde(rename = "TimingPointWheelChairAccessible")]
    pub tp_wheelchair: Accessibility,
    #[serde(rename = "TimingPointVisualAccessible")]
    pub tp_visual: Accessibility,
}

#[derive(Deserialize, Debug)]
pub struct Pass {
    #[serde(rename = "DestinationName50")]
    pub destination_name: String,
}
