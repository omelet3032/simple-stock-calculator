use simple_stock_calculator::calculator::*;
use simple_stock_calculator::types::{Country, Leverage, Position, StockInfo};
// use simple_stock_calculator::constraints::MASTER_PRECISION_SCALE;
// const EPSILON: f64 = 1e-6;

//
/*
    test 케이스 정리
    1. 한국 주식 1,000,000원 이상일 경우 overflow
    2. 버크셔 주식 overflow 확인

*/

#[test]
fn test_convert_recovery_rate_to_percentage() {
    let result = convert_rate_to_percentage(2333333333333);
    let expected: f64 = 233.33;
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
    let result1: i64 = convert_stock_price_to_bp(250.0);
    let expected1: i64 = 25000;
    println!("result1 : {}", result1);
    assert_eq!(result1, expected1);
}

#[test]
fn test_calculate_result() {
    let user_stock_info: StockInfo =
        generate_user_stock_info(Country::US, Position::Long, Leverage::Daily2x, 70.0, 250.0);


    let result = user_stock_info.target_underlying_stock_price;
    let expected: f64 = 541.67;

    assert_eq!(result, expected);
}

#[test]
fn test_calculate_required_recovery_rate_bp() {
    let result = calculate_required_recovery_rate_with_master_precision_scale(7000);

    let expected: i64 = 2333333333333;

    println!("result : {}", result);
    assert_eq!(result, expected); // 2.0 / 233.33
}

#[test]
fn test_scale_leveraged_required_recovery_rate_bp() {
    let result = calculate_leveraged_required_recovery_rate_with_master_precision_scale(2333333333333, &Leverage::Daily2x);

    let expected = 11667000000000000;

    assert_eq!(result, expected);
}

// #[should_panic(
//     expected = "Over Flow in calculate_target_stock_price: intermediate product exceeds i128 max!"
// )] // gemini
#[test]
fn test_calculate_target_stock_price() {
    let result1: f64 = calculate_target_underlying_stock_price(&Position::Long, 25000, 11667);
    // 21667000000000000
    let expected: f64 = 541.67;

    assert_eq!(result1, expected);

    /* let very_large_stock_price:i64 = 1_000_000_000_000_000_000; // 10^30
       let very_large_recovery_rate_bp:i64 = 9_000_000_000_000_000_000;

       let result2 = calculate_target_stock_price(Position::Long,very_large_stock_price,very_large_recovery_rate_bp);

       assert!(result2 > EPSILON);
    */
}
