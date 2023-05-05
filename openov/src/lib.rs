pub struct TimingPointCode {}

pub enum Accessibility {
    Accessible,
}

pub struct Position(f32, f32);

pub struct Stop {
    pub position: Position,
    pub town: String,
    pub tpc_name: String,
    pub tpc_code: u32,
    pub stop_area_code: u16,
    pub tp_wheelchair: Accessibility,
    pub tp_visual: Accessibility,
}
