use simple_stock_calculator::calculator::*;
use simple_stock_calculator::types::{Position, Leverage};

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
fn test_calculate_required_recovery_rate() {

    let (result1, result2) = calculate_required_recovery_rate(70.00);

    let expected1 = 233.33;
    let expected2 = 23333;

    println!("result1 : {}, result2 : {}", result1, result2); // 2 / 233
    assert_eq!(result1, expected1); // 2.0 / 233.33
    assert_eq!(result2, expected2); // 233 / 23333
}
/* pub fn calculate_target_stock_price(
    position: Position,
    leverage: Leverage,
    required_recovery_rate_bp: i64,
    user_entered_stock_price: f64,
) -> f64 {
    let stock_price_bp: i64 = convert_stock_price_to_bp(user_entered_stock_price);

    let recovery_rate_bp_scaled: i64 = (RATE_SCALE * MASTER_PRICISION_SCALE)
        + (required_recovery_rate_bp * MASTER_PRICISION_SCALE) / leverage.value();

    match position {
        Position::Long => {
            let target_price_bp: i64 =
                if let Some(price) = stock_price_bp.checked_mul(recovery_rate_bp_scaled) {
                    price / (MASTER_PRICISION_SCALE * RATE_SCALE)
                } else {
                    panic!("Over Flow!!");
                };

            target_price_bp as f64 / PRICE_SCALE as f64 
        }
        Position::Short => {
            100.0
        }
    }
} */
#[test]
fn test_calculate_target_stock_price() {

    let result:f64 = calculate_target_stock_price(Position::Long, Leverage::Daily2x, 23333, 250.00);

    let expected:f64 = 541.67;

    assert_eq!(result, expected);


}


