use super::types::Position;
use super::types::Leverage;

pub fn calculate_recovery_rate(loss_rate: f64) -> i32 {
    /* 
        사용자가 입력한 f64 손실율을 * 100.0을 통해 정수로 변환한다.
     */
    let loss_rate_bp = (loss_rate * 100.0) as i32;

    let recovery_rate:f64 = loss_rate_bp / (10000 - loss_rate_bp);
    recovery_rate * 100
}

pub fn calculate_target_stock_price(
    position: Position,
    leverage: Leverage,
    recovery_rate: f64,
    current_stock_price: f64,
) -> i32 {
    match position {
        Position::Long => current_stock_price * (1.0 + (recovery_rate / leverage.value())),
        Position::Short => current_stock_price * (1.0 - (recovery_rate / leverage.value())),
    }
}