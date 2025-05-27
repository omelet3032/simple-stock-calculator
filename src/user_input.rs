mod input;
use input::*;

use super::types::{Leverage, Menu::*, Message, Position, Sign, Guide::Exit};

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

pub fn enter_stock_price() -> f64 {
    get_input_price(Message::MenuMessage(EnterStockPrice), Sign::Doller)
}

pub fn select_exit() -> bool {
    get_input_exit(Message::GuideMessage(Exit))
}

