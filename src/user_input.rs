pub mod input;
use input::*;

// use crate::types::UserInputPrice;

use super::types::{Country, Leverage, Menu::*, Message, Position, Guide::Exit};

pub fn select_country() -> Country {
    get_input_select(Message::MenuMessage(SelectCountry), |c| match c {
        "1" => Some(Country::KR),
        "2" => Some(Country::US),
        _ => None,
    })
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

pub fn enter_stock_price(country:&Country) -> f64 {
    let user_input_price = parse_input_price(Message::MenuMessage(EnterStockPrice), country);
    user_input_price
}

pub fn select_exit() -> bool {
    get_input_exit(Message::GuideMessage(Exit))
}

