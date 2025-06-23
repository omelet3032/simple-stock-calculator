use std::fmt::{self};

use crate::types::Country;

use super::types::{CurrencySign, Leverage, Position, StockInfo};
use super::types::{Guide::*, Invalid::*, Menu::*, Message};

pub fn print_start() {
    println!("{}", Message::GuideMessage(StartGuide));
    println!("{}", Message::GuideMessage(Warning));
}

pub fn print_result(user_stock_info: StockInfo) {
    let underlying_stock_price = match user_stock_info.country {
        Country::KR => CurrencySign::Won,
        Country::US => CurrencySign::Dollar,
    };

    println!(
        "{}",
        Message::GuideMessage(UserStockInfo(
            user_stock_info.country,
            user_stock_info.loss_rate,
            user_stock_info.leverage,
            underlying_stock_price
                .with_currency_sign(user_stock_info.current_underlying_stock_price),
        ))
    );
    println!();
    println!(
        "{}",
        Message::GuideMessage(ResultGuide(
            user_stock_info.required_recovery_rate,
            user_stock_info.leveraged_required_recovery_rate,
            underlying_stock_price
                .with_currency_sign(user_stock_info.target_underlying_stock_price)
        ))
    );
}

impl fmt::Display for Country {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Country::KR => write!(f, "대한민국"),
            Country::US => write!(f, "미국"),
        }
    }
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

impl CurrencySign {
    pub fn with_currency_sign(&self, value: f64) -> String {
        match self {
            CurrencySign::Won => format!("{}원", value),
            CurrencySign::Dollar => format!("${}", value),
        }
    }
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Message::GuideMessage(StartGuide) => write!(
                f,
                "---------------------\n\
            *** 주식 레버리지 회복율 계산기 ***\n\n\
            레버리지를 탔는데, 속절없이 주가가 떨어졌던 경험이 있으신가요?\n\
            '본주 가격이 얼마가 되어야 본전이지?'라며 골머리를 앓고 계신다면\n\
            이 앱을 사용해보세요.\n\
            ---------------------\n"
            ),
            Message::GuideMessage(Exit) => write!(f, "종료하시겠습니까? (Y/n)"),
            Message::GuideMessage(Warning) => write!(
                f,
                "---------------------\n\
            WARNING\n\
            1. 한 번의 하락없이 계속 상승한다는 가정하에 계산되었으며, \
            주식 시장 상황에 따른 괴리율로 인해 실제 주가는 산출된 기존 값을 훨씬 상회할 수 있음을 알려드립니다.\n\n\
            2. 레버리지는 위험합니다. 자칫하면 패가망신할 수도 있으니 주의하세요.\n\
            ---------------------\n"
            ),
            Message::GuideMessage(EnteredValue) => write!(f, "입력한 값"),

            Message::GuideMessage(UserStockInfo(
                country,
                loss_rate,
                leverage,
                current_underlying_stock_price,
            )) => write!(
                f,
                "-- 주식 정보 --\n\
            국가 : {}\n\
            손실율 : {:.2}%\n\
            레버리지 배율 : {}\n\
            현재 ETF 추종 주가 : {}",
                country, loss_rate, leverage, current_underlying_stock_price
            ),  

            Message::GuideMessage(ResultGuide(
                required_recovery_rate,
                leveraged_required_recovery_rate,
                target_underlying_stock_price,
            )) => write!(
                f,
                "-- 계산 결과 --\n\
            필요 회복율 : {}%\n\
            필요 회복율(레버리지 배율 적용) : {}%\n\
            원금 회복 목표 주가 : {}",
                required_recovery_rate,
                leveraged_required_recovery_rate,
                target_underlying_stock_price
            ),

            Message::MenuMessage(SelectCountry) => {
                write!(f, "1. 국가를 선택해주세요.\n\n1) KR, 2) US")
            }
            Message::MenuMessage(SelectPosition) => {
                write!(f, "2. 포지션을 선택해주세요.\n\n1) Long, 2) Short")
            }
            Message::MenuMessage(SelectLeverage) => {
                write!(f, "3. 배율을 선택해주세요.\n\n1) 2X 2) 3X")
            }
            Message::MenuMessage(EnterLossRate) => write!(f, "4. 손실율을 입력해주세요."),
            Message::MenuMessage(EnterStockPrice) => write!(f, "5. 본주 가격을 입력해주세요."),

            Message::InvalidMessage(InvalidInt) => write!(f, "정수를 입력해주세요"),
            Message::InvalidMessage(InvalidNumber) => write!(f, "숫자를 입력해주세요."),
            Message::InvalidMessage(InvalidRange) => write!(f, "유효한 범위내에서 입력해주세요."),
            Message::InvalidMessage(InvalidChoice) => write!(f, "보기중 하나를 선택해주세요."),
            Message::InvalidMessage(InvalidYn) => {
                write!(f, "Y 또는 n을 입력해주세요.(대소문자 유의)")
            }
        }
    }
}
