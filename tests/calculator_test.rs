use simple_stock_calculator::calculator::*;
use simple_stock_calculator::types::*;

use proptest::prelude::*;

const EPSILON:f64 = 1e-6;

#[test]
fn test_calculate_recovery_rate() {
    let result = calculate_recovery_rate(20.0);
    let expected: f64 = 20.0 / 80.0;
    assert!((result-expected).abs() < EPSILON);
}

#[test]
fn test_calculate_target_stock_price() {

    let position = Position::Long;
    let leverage = Leverage::Daily2x;
    let recovery_rate = 20.0;
    let current_stock_price = 0.1123;

    let result_long = calculate_target_stock_price(position, leverage, recovery_rate, current_stock_price);
    let expected = 0.1123 * (1.0 + (20.0 / 2.0));

    // assert!((result-expected).abs() < EPSILON);
    assert_eq!(result_long, expected);

   /* 
    엣지 케이스 테스트
   */ 
    let result_edge = calculate_target_stock_price(Position::Long, Leverage::Daily2x, 0.0, 100.0);
    assert_eq!(result_edge, 100.0);
}
