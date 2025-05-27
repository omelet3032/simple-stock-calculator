pub enum Position {
    Long,
    Short,
}
#[derive(Copy, Clone)]
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

pub enum Sign {
    Doller,
    Won,
}

pub enum Guide {
    StartGuide,
    EnteredValue,
    Warning,
    ResultGuide(f64, f64, Leverage, f64, f64),
    Exit,
}

pub enum Menu {
    SelectPosition,
    SelectLeverage,
    EnterLossRate,
    EnterStockPrice,
}

pub enum Invaild {
    InvaildNumber,
    InvaildRange,
    InvaildChoice,
    InvaildYn,
}
pub enum Message {
    GuideMessage(Guide),
    MenuMessage(Menu),
    InvaildMessage(Invaild),
}
