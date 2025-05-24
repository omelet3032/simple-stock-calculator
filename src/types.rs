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
    Percentage,
}


pub enum Message {
    StartGuide,

    InvaildNumber,
    InvaildRange,
    InvaildChoice,

    EnteredValue,

    SelectPosition,
    SelectLeverage,

    EnterLossRate,
    EnterStockPrice,

    ResultGuide,

    InputRate(f64),
    InputPrice(f64),
    InputLeverage(Leverage),

    BaseResult(f64),
    AdjustedResult(f64),
    PriceResult(f64),
}

