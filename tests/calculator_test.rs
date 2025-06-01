use simple_stock_calculator::calculator::*;

const EPSILON: f64 = 1e-6;

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
fn test_calculate_required_recovery_rate() {

    let (result1, result2) = calculate_required_recovery_rate(70.00);

    let expected1 = 233.33;
    let expected2 = 23333;

    println!("result1 : {}, result2 : {}", result1, result2); // 2 / 233
    assert_eq!(result1, expected1); // 2.0 / 233.33
    assert_eq!(result2, expected2); // 233 / 23333
}


