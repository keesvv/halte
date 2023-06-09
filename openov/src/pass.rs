use serde::Deserialize;

use crate::{Coord, TransportType};

#[derive(Deserialize, Debug)]
pub struct Pass {
    #[serde(rename = "IsTimingStop")]
    pub is_timing_stop: bool,
    #[serde(rename = "DestinationName50")]
    pub destination_name: String,
    #[serde(rename = "DataOwnerCode")]
    pub data_owner_code: String,
    #[serde(rename = "OperatorCode")]
    pub operator_code: String,
    #[serde(rename = "FortifyOrderNumber")]
    pub fortify_order_no: u64,
    #[serde(rename = "LinePublicNumber")]
    pub line_public_no: String,
    #[serde(rename = "TransportType")]
    pub transport_type: TransportType,
    #[serde(rename = "Latitude")]
    pub lat: Coord,
    #[serde(rename = "Longitude")]
    pub long: Coord,
    #[serde(rename = "TargetArrivalTime")]
    pub target_arrival_time: chrono::NaiveDateTime,
    #[serde(rename = "TargetDepartureTime")]
    pub target_departure_time: chrono::NaiveDateTime,
    #[serde(rename = "ExpectedArrivalTime")]
    pub expected_arrival_time: chrono::NaiveDateTime,
}
