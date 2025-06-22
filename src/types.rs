pub enum Country {
    KR, 
    US, 
}
pub enum Position {
    Long,
    Short,
}
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

pub enum CurrencySign {
    Dollar,
    Won,
}

pub enum Message {
    GuideMessage(Guide),
    MenuMessage(Menu),
    InvalidMessage(Invalid),
}

pub enum Guide {
    StartGuide,
    EnteredValue,
    Warning,
    UserStockInfo(Country, f64, Leverage, String),
    ResultGuide(f64, f64, String),
    Exit,
}

pub enum Menu {
    SelectCountry,
    SelectPosition,
    SelectLeverage,
    EnterLossRate,
    EnterStockPrice,
}

pub enum Invalid {
    InvalidInt,
    InvalidNumber,
    InvalidRange,
    InvalidChoice,
    InvalidYn,
}

pub struct StockInfo {
    pub country: Country,
    pub position: Position,
    pub leverage: Leverage,
    pub loss_rate: f64,
    pub current_underlying_stock_price: f64,

    pub required_recovery_rate: f64,
    pub leveraged_required_recovery_rate: f64,
    pub target_underlying_stock_price: f64,
}
