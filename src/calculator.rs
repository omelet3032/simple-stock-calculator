use super::types::RecoveryRate;
use super::types::Position;
use super::types::Leverage;

pub fn calculate_recovery_rate(loss_rate: f64, leverage: Leverage) -> RecoveryRate {
    let recovery_rate = (loss_rate / (100.0 - loss_rate));
    let with_leverage = recovery_rate / leverage.value();

    RecoveryRate {
        recovery_rate,
        with_leverage,
    }
}

pub fn calculate_target_stock_price(
    position: Position,
    recovery_rate_with_leverage: f64,
    current_stock_price: f64,
) -> f64 {
    match position {
        Position::Long => current_stock_price * (1.0 + recovery_rate_with_leverage),
        Position::Short => current_stock_price * (1.0 - recovery_rate_with_leverage),
    }
}