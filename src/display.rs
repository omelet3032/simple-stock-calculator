use std::fmt::{self};

use crate::types::Country;

use super::types::{Guide::*, Invaild::*, Menu::*, Message};
use super::types::{Leverage, Position, Sign};
// use super::calculator::c

pub fn print_start() {
    println!("{}", Message::GuideMessage(StartGuide));
    println!("{}", Message::GuideMessage(Warning));
}

pub fn print_result(
    loss_rate: f64,
    current_stock_price: f64,
    leverage: Leverage,
    recovery_rate: f64,
    target_stock_price: f64,
) {
    println!(
        "{}",
        Message::GuideMessage(ResultGuide(
            loss_rate,
            recovery_rate,
            leverage,
            current_stock_price,
            target_stock_price
        ))
    );
}

impl fmt::Display for Country {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Country::KR => write!(f, "South Korea"),
            Country::US => write!(f, "USA"),
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

impl Sign {
    pub fn format_value(&self, value: f64) -> String {
        match self {
            Sign::Doller => format!("${}", value),
            Sign::Won => format!("{}원", value),
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

            Message::MenuMessage(SelectCountry) => {
                write!(f, "1. 국가를 선택해주세요.\n\n1) KR, 2)US")
            }
            Message::MenuMessage(SelectPosition) => {
                write!(f, "2. 포지션을 선택해주세요.\n\n1) Long, 2) Short")
            }
            Message::MenuMessage(SelectLeverage) => {
                write!(f, "3. 배율을 선택해주세요.\n\n1) 2X 2) 3X")
            }
            Message::MenuMessage(EnterLossRate) => write!(f, "4. 손실율을 입력해주세요."),
            Message::MenuMessage(EnterStockPrice) => write!(f, "5. 본주 가격을 입력해주세요."),

            Message::InvaildMessage(InvaildNumber) => write!(f, "숫자를 입력해주세요."),
            Message::InvaildMessage(InvaildRange) => write!(f, "유효한 범위내에서 입력해주세요."), // 퍼센테이지, 가격 따로 함수 만들기
            Message::InvaildMessage(InvaildChoice) => write!(f, "보기중 하나를 선택해주세요."),
            Message::InvaildMessage(InvaildYn) => {
                write!(f, "Y 또는 n을 입력해주세요.(대소문자 유의)")
            }
            Message::GuideMessage(EnteredValue) => write!(f, "입력한 값"),

            Message::GuideMessage(ResultGuide(
                loss_rate,
                recovery_rate,
                leverage,
                current_price,
                target_price,
            )) => write!(
                f,
                "계산 결과\n\n\
            현재 입력하신 손실율은 '{}%' 입니다.\n\
            '{:.2}%' 상승시 매수하신 Leverage ETF의 손실 복구가 가능합니다.\n\
            '{}' 배율을 적용하여, ETF 추종 주가가 '{:.2}%' 상승시 원금 회복이 가능합니다.\n\
            현재 주가는 '{:.2}'이므로 목표 주가는 '{:.2}'입니다.
",
                loss_rate,
                recovery_rate,
                leverage,
                (recovery_rate / leverage.value() as f64),
                current_price,
                target_price
            ),
        }
    }
}
