use simple_stock_calculator::calculator::*;
use simple_stock_calculator::types::{Country, Leverage, Position, UserInputPrice};

// const EPSILON: f64 = 1e-6;

//
/*
    test 케이스 정리
    1. 한국 주식 1,000,000원 이상일 경우 overflow
    2. 버크셔 주식 overflow 확인

*/

#[test]
fn test_convert_recovery_rate_to_percentage() {
    let result = convert_recovery_rate_to_percentage(23333);
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
    let result1: i64 = convert_stock_price_to_bp(UserInputPrice::Integer(250));
    let expected1: i64 = 250;
    println!("result1 : {}", result1);
    assert_eq!(result1, expected1);

    let result2:i64 = convert_stock_price_to_bp(UserInputPrice::Float(250.0));
    let expected2:i64 = 25000;

    println!("result2 : {}", result2);
    assert_eq!(result2, expected2);

}

#[test]
fn test_calculate_result_kr() {
    let price: UserInputPrice = calculate_result(
        Country::KR,
        Position::Long,
        Leverage::Daily2x,
        70.0,
        UserInputPrice::Integer(250),
    );
    let expected: i64 = 542;

    let result = if let UserInputPrice::Integer(value) = price {
        println!("value : {}", value);
        value
    } else {
        1
    };

    assert_eq!(result, expected);
}
#[test]
fn test_calculate_result_us() {
    let price: UserInputPrice = calculate_result(
        Country::US,
        Position::Long,
        Leverage::Daily2x,
        70.0,
        UserInputPrice::Float(250.0),
    );
    let expected: f64 = 541.67;

    let result = if let UserInputPrice::Float(value) = price {
        println!("value : {}", value);
        value
    } else {
        1.0
    };

    assert_eq!(result, expected);
}
#[test]
fn test_calculate_required_recovery_rate_bp() {
    let result = calculate_required_recovery_rate_bp(7000);

    let expected: i64 = 23333;

    println!("result : {}", result);
    assert_eq!(result, expected); // 2.0 / 233.33
}

#[test]
fn test_scale_leveraged_required_recovery_rate_bp() {

    let result = scale_leveraged_required_recovery_rate_bp(23333, Leverage::Daily2x);

    let expected = 11667;

    assert_eq!(result, expected);
}

#[should_panic(
    expected = "Over Flow in calculate_target_stock_price: intermediate product exceeds i128 max!"
)] // gemini
#[test]
fn test_calculate_target_stock_price() {
    let result1: f64 = calculate_target_stock_price(Position::Long, 250,  21667000000000000);
    // 21667000000000000
    let expected: f64 = 541.67;

    assert_eq!(result1, expected);

    /* let very_large_stock_price:i64 = 1_000_000_000_000_000_000; // 10^30
       let very_large_recovery_rate_bp:i64 = 9_000_000_000_000_000_000;

       let result2 = calculate_target_stock_price(Position::Long,very_large_stock_price,very_large_recovery_rate_bp);

       assert!(result2 > EPSILON);
    */
}
