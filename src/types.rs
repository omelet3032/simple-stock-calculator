pub enum Position {
    Long,
    Short,
}

pub enum Leverage {
    Daily2x,
    Daily3x,
}

impl Leverage {
    pub fn value(&self) -> f64 {
        match self {
            Leverage::Daily2x => 2.0,
            Leverage::Daily3x => 3.0,
        }
    }
}
