pub mod input;
use input::*;

use super::application::ErrorType;
use super::types::{Country, Guide::Exit, Leverage, Menu::*, Message, Position};

pub fn select_country() -> Result<Country, ErrorType> {
    let parse_user_country_input = |s: &str| -> Result<Country, ErrorType> {
        match s.trim() {
            "1" => Ok(Country::KR),
            "2" => Ok(Country::US),
            _ => Err(ErrorType::InvalidInputFormat(
                "유효하지 않은 국가 선택입니다. 1 또는 2를 입력해주세요.".to_string(),
            )),
        }
    };

    get_input_select(
        Message::MenuMessage(SelectCountry),
        parse_user_country_input,
    )
}


pub fn select_position() -> Position {
    get_input_select(Message::MenuMessage(SelectPosition), |p| match p {
        "1" => Some(Position::Long),
        "2" => Some(Position::Short),
        _ => None,
    })
}

pub fn select_leverage() -> Leverage {
    get_input_select(Message::MenuMessage(SelectLeverage), |s| match s {
        "1" => Some(Leverage::Daily2x),
        "2" => Some(Leverage::Daily3x),
        _ => None,
    })
}

pub fn enter_loss_rate() -> f64 {
    get_input_rate(Message::MenuMessage(EnterLossRate))
}

pub fn enter_stock_price(country: &Country) -> f64 {
    get_input_price(Message::MenuMessage(EnterStockPrice), country)
}

pub fn select_exit() -> bool {
    get_input_exit(Message::GuideMessage(Exit))
}
