use super::types::Position;
use super::types::Leverage;

pub fn calculate_recovery_rate(loss_rate: f64) -> f64 {
    let recovery_rate = loss_rate / (100.0 - loss_rate);
    recovery_rate
}

pub fn calculate_target_stock_price(
    position: Position,
    leverage: Leverage,
    recovery_rate: f64,
    current_stock_price: f64,
) -> f64 {
    match position {
        Position::Long => current_stock_price * (1.0 + (recovery_rate / leverage.value())),
        Position::Short => current_stock_price * (1.0 - (recovery_rate / leverage.value())),
    }
}