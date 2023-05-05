use serde::Deserialize;

use crate::{Accessibility, Coord};

#[derive(Deserialize, Debug)]
pub struct Stop {
    #[serde(rename = "Latitude")]
    pub lat: Coord,
    #[serde(rename = "Longitude")]
    pub long: Coord,
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
