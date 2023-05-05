use openov::{pass::Pass, tp::TimingPointResponse};

#[derive(Debug)]
pub struct Timing {
    pub arrival: chrono::NaiveDateTime,
    pub arrival_expected: chrono::NaiveDateTime,
    pub departure: chrono::NaiveDateTime,
}

#[derive(Debug)]
pub struct Departure {
    pub id: String,
    pub timing: Timing,
}

impl From<(&String, &Pass)> for Departure {
    fn from((id, pass): (&String, &Pass)) -> Self {
        Self {
            id: id.to_owned(),
            timing: Timing {
                arrival: pass.target_arrival_time,
                arrival_expected: pass.expected_arrival_time,
                departure: pass.target_departure_time,
            },
        }
    }
}

#[derive(Debug)]
pub struct DepartureInfo {
    pub departures: Vec<Departure>,
}

impl From<TimingPointResponse> for DepartureInfo {
    fn from(value: TimingPointResponse) -> Self {
        Self {
            departures: value
                .tps
                .values()
                .next()
                .unwrap()
                .passes
                .iter()
                .map(Departure::from)
                .collect(),
        }
    }
}
