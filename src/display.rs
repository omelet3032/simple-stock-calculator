use std::fmt;

use super::types::{Leverage, Message, Position, Sign};

pub fn print_app_guide() {
    println!("{}", Message::StartGuide);
}

pub fn print_result(base_rate: f64, adjusted_rate: f64, target_stock_price: f64) {
    println!("{}", Message::ResultGuide);
    println!("{}", Message::BaseResult(base_rate));
    println!("{}", Message::AdjustedResult(adjusted_rate));
    println!("{}", Message::PriceResult(target_stock_price));
}

impl fmt::Display for Leverage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Leverage::Daily2x => write!(f, "2X"),
            Leverage::Daily3x => write!(f, "3X"),
        }
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Position::Long => write!(f, "Long"),
            Position::Short => write!(f, "Short"),
        }
    }
}

impl Sign {
    pub fn format_value(&self, value: f64) -> String {
        match self {
            Sign::Doller => format!("${}", value),
            Sign::Percentage => format!("{}%", value),
        }
    }
}
impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Message::StartGuide => write!(
                f,
            "---------------------\n\n\
            *** 주식 레버리지 회복율 계산기 ***\n\n\
            레버리지를 탔는데, 속절없이 주가가 떨어졌던 경험이 있으신가요?\n\
            '본주 가격이 얼마가 되어야 본전이지?'라며 골머리를 앓고 계신다면\n\
            이 앱을 사용해보세요.\n\n\
            -- WARNING --\n\
            1. 한 번의 하락없이 계속 상승한다는 가정하에 계산되었으며, \
            주식 시장 상황에 따른 괴리율로 인해 실제 주가는 산출된 기존 값을 훨씬 상회할 수 있음을 알려드립니다.\n\n\
            2. 레버리지는 위험합니다. 자칫하면 패가망신할 수도 있으니 주의하세요.\n\
            ---------------------\n"
            ),

            Message::SelectPosition => write!(f, "1. 포지션을 선택해주세요.\n\n1) Long, 2) Short"),
            Message::SelectLeverage => write!(f, "2. 배율을 선택해주세요.\n\n1) 2X 2) 3X"),
            Message::EnterLossRate => write!(f, "3. 손실율을 입력해주세요."),
            Message::EnterStockPrice => write!(f, "4. 본주 가격을 입력해주세요."),

            Message::InvaildNumber => write!(f, "숫자를 입력해주세요."),
            Message::InvaildRange => write!(f, "유효한 범위내에서 입력해주세요."), // 퍼센테이지, 가격 따로 함수 만들기
            Message::InvaildChoice => write!(f, "보기중 하나를 선택해주세요."),
            Message::EnteredValue => write!(f, "입력한 값"),

            Message::ResultGuide => write!(f, "계산 결과\n"),
            Message::BaseResult(base_rate) => write!(
                f,
                "-> 1. 필요 회복율 : {}%",
                format!("{:.2}", base_rate * 100.0)
            ),
            Message::AdjustedResult(adjusted_rate) => write!(
                f,
                "-> 2. 레버리지 배율이 적용된 필요 회복율 : {}%",
                format!("{:.2}", adjusted_rate * 100.0)
            ),
            Message::PriceResult(target_stock_price) => write!(
                f,
                "-> 3. 목표 주가 : ${}",
                format!("{:.2}", target_stock_price)
            ),
        }
    }
}
