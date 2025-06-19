pub enum Country {
    KR, // South Korea
    US, // Unite states of america
}
pub enum Position {
    Long,
    Short,
}
// #[derive(Copy, Clone)]
pub enum Leverage {
    Daily2x,
    Daily3x,
}

impl Leverage {
    pub fn value(&self) -> i64 {
        match self {
            Leverage::Daily2x => 2,
            Leverage::Daily3x => 3,
        }
    }
}

pub struct StockInfo {
    pub country:Country,
    pub position:Position,
    pub leverage:Leverage,
    pub loss_rate:f64,
    pub current_underlying_stock_price:f64,
    
    pub required_recovery_rate:f64,
    pub target_underlying_stock_price:f64,
}

pub enum CurrencySign {
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
    SelectCountry,
    SelectPosition,
    SelectLeverage,
    EnterLossRate,
    EnterStockPrice,
}

pub enum Invaild {
    InvaildInt,
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
