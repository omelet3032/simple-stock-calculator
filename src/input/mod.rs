mod input;
use input::*;

use super::constraints::*;
use super::types::{Leverage, Message, Position, Sign};

pub fn select_position() -> Position {
    get_input_select(Message::SelectPosition, |p| match p {
        "1" => Some(Position::Long),
        "2" => Some(Position::Short),
        _ => None,
    })
}

pub fn select_leverage() -> Leverage {
    get_input_select(Message::SelectLeverage, |s| match s {
        "1" => Some(Leverage::Daily2x),
        "2" => Some(Leverage::Daily3x),
        _ => None,
    })
}

pub fn enter_loss_rate() -> f64 {
    get_input_rate(
        Message::EnterLossRate,
        Sign::Percentage,
        MIN_LOSS_RATE,
        MAX_LOSS_RATE,
    )
}

pub fn enter_stock_price() -> f64 {
    get_input_rate(
        Message::EnterStockPrice,
        Sign::Doller,
        MIN_STOCK_PRICE,
        MAX_STOCK_PRICE,
    )
}
