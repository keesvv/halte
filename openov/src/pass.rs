use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Pass {
    #[serde(rename = "DestinationName50")]
    pub destination_name: String,
}
