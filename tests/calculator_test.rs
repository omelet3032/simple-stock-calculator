use simple_stock_calculator::calculator::*;
use simple_stock_calculator::types::*;

/* pub fn calculate_recovery_rate(loss_rate: f64) -> f64 {
    let recovery_rate = loss_rate / (100.0 - loss_rate);
    recovery_rate
} */

#[test]
fn test_calculate_recovery_rate() {
    let result = calculate_recovery_rate(20.0);
    let expected: f64 = 20.0 / 80.0;
    let epsilon = 1e-6;
    assert!((result-expected).abs() < epsilon);
}

/* pub fn calculate_target_stock_price(
    position: Position,
    leverage: Leverage,
    recovery_rate: f64,
    current_stock_price: f64,
) -> f64 {
    match position {
        Position::Long => current_stock_price * (1.0 + (recovery_rate / leverage.value())),
        Position::Short => current_stock_price * (1.0 - (recovery_rate / leverage.value())),
    }
} */
#[test]
fn test_calculate_target_stock_price() {

    let position = Position::Long;
    let leverage = Leverage::Daily2x;
    let recovery_rate = calculate_recovery_rate(20.0);
    let current_stock_price = 250.00;

    let result = calculate_target_stock_price(position, leverage, recovery_rate, current_stock_price);
    let expected = current_stock_price * (1.0 + (recovery_rate / leverage.value()));
    let epsilon = 1e-6;
    assert!((result-expected).abs() < epsilon);
}
