use simple_stock_calculator::calculator::*;
use simple_stock_calculator::types::{Country, Leverage, Position, UserInputPrice};

const EPSILON: f64 = 1e-6;

// 
/* 
    test 케이스 정리
    1. 한국 주식 1,000,000원 이상일 경우 overflow
    2. 버크셔 주식 overflow 확인

*/

#[test]
fn test_convert_recovery_rate_to_percentage() {
    let result = convert_recovery_rate_to_percentage(23333);
    let expected:f64 = 233.33;
    println!("result : {}", result);
    assert_eq!(result, expected);
}

#[test]
fn test_convert_loss_rate_to_bp() {
    let result = convert_loss_rate_to_bp(70.00);
    let expected: i64 = 7000;
    println!("result : {}", result);
    assert_eq!(result, expected);
}

#[test]
fn test_convert_stock_price_to_bp() {
    let result: i64 = convert_stock_price_to_bp(250.00);
    let expected: i64 = 25000;
    println!("result : {}", result);
    assert_eq!(result, expected);
}
#[test]
fn test_calculate_result() {

    let result:f64 = calculate_result(Country::KR, Position::Long, Leverage::Daily2x, 70.0, UserInputPrice::Integer(250));
    let expected:f64 = 541.0;

    assert_eq!(result, expected); 
}

/* #[test]
fn test_calculate_required_recovery_rate() {

    let (result1, result2) = calculate_required_recovery_rate(99.00);

    let expected1 = 233.33;
    let expected2 = 23333;

    println!("result1 : {}, result2 : {}", result1, result2); // 2 / 233
    assert_eq!(result1, expected1); // 2.0 / 233.33
    assert_eq!(result2, expected2); // 233 / 23333
}

#[should_panic(expected = "Over Flow in calculate_target_stock_price: intermediate product exceeds i128 max!")] // gemini
fn test_calculate_target_stock_price() {

    let result1:f64 = calculate_target_stock_price(Position::Long, Leverage::Daily2x, 23333, 250.00);

    let expected:f64 = 541.67;

    let very_large_stock_price = 1_000_000_000_000_000_000_000_000_000_000.0; // 10^30
    let very_large_recovery_rate_bp:i64 = 9_000_000_000_000_000_000;

    assert_eq!(result1, expected);

    // let result2 = calculate_target_stock_price(Position::Long, Leverage::Daily2x, 990000, 1000000.0);
    let result2 = calculate_target_stock_price(Position::Long, Leverage::Daily2x,very_large_recovery_rate_bp,very_large_stock_price);

    assert!(result2 > EPSILON);

}
 */

