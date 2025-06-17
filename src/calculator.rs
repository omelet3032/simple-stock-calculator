use crate::types::UserInputPrice;

use super::constraints::{MASTER_PRICISION_SCALE, PRICE_SCALE, RATE_SCALE};
use super::types::Country;
use super::types::Leverage;
use super::types::Position;

pub fn calculate_result(
    country: Country,
    position: Position,
    leverage: Leverage,
    user_entered_loss_rate: f64,
    user_entered_stock_price: UserInputPrice,
) -> f64 {
    let loss_rate_bp: i64 = convert_loss_rate_to_bp(user_entered_loss_rate);

    let stock_price: i64 = match user_entered_stock_price {
        UserInputPrice::Integer(value) => value,
        UserInputPrice::Float(value) => convert_stock_price_to_bp(value),
    };

    let required_recovery_rate_bp = calculate_required_recovery_rate_bp(loss_rate_bp);
    let scaled_required_recovery_rate_bp =
        scale_leveraged_required_recovery_rate_bp(required_recovery_rate_bp, leverage);

    let target_stock_price: f64 =
        calculate_target_stock_price(position, stock_price, scaled_required_recovery_rate_bp);

    let final_price = match country {
        Country::KR => target_stock_price,
        Country::US => target_stock_price as f64 / PRICE_SCALE as f64,
    };

    return final_price;
    // target_stock_price
}

pub fn calculate_target_stock_price(
    position: Position,
    stock_price_bp: i64,
    scaled_required_recovery_rate_bp: i64,
) -> f64 {
    // 5
    let target_price: f64 = match position {
        Position::Long => {
            let target_price_bp: i128 = if let Some(price) =
                (stock_price_bp as i128).checked_mul(scaled_required_recovery_rate_bp as i128)
            {
                price
            } else {
                panic!("Over Flow!!");
            };
            let target_price: i128 =
                target_price_bp / (MASTER_PRICISION_SCALE as i128 * RATE_SCALE as i128);
            
            target_price as f64
            // target_price as f64 / PRICE_SCALE as f64
        }
        Position::Short => 100.0,
    };

    target_price
}

pub fn scale_leveraged_required_recovery_rate_bp(
    required_recovery_rate_bp: i64,
    leverage: Leverage,
) -> i64 {
    // 4
    let recovery_rate_bp_scaled: i64 = (RATE_SCALE * MASTER_PRICISION_SCALE)
        + ((required_recovery_rate_bp + leverage.value() / 2) * MASTER_PRICISION_SCALE)
            / leverage.value();

    recovery_rate_bp_scaled
}

// 3
pub fn calculate_required_recovery_rate_bp(loss_rate_bp: i64) -> i64 {
    let required_recovery_rate_scaled: i64 =
        (loss_rate_bp * MASTER_PRICISION_SCALE) / (RATE_SCALE - loss_rate_bp);

    let required_recovery_rate_bp: i64 =
        (required_recovery_rate_scaled * RATE_SCALE) / MASTER_PRICISION_SCALE;

    required_recovery_rate_bp // 내부 계산용 -> calculate_target_price에서 사용
}

// 마지막
pub fn convert_recovery_rate_to_percentage(recovery_rate_bp: i64) -> f64 {
    let required_recovery_rate = recovery_rate_bp as f64 / PRICE_SCALE as f64;
    required_recovery_rate
}

// 1
pub fn convert_loss_rate_to_bp(user_entered_loss_rate: f64) -> i64 {
    let converted_loss_rate_for_percentage: f64 = user_entered_loss_rate / 100.0;

    let loss_rate_bp: i64 = (converted_loss_rate_for_percentage * RATE_SCALE as f64).round() as i64;

    loss_rate_bp
}

// 2

pub fn convert_stock_price_to_bp(user_entered_price: f64) -> i64 {
    let stock_price_bp: i64 = (user_entered_price * PRICE_SCALE as f64).round() as i64;

    stock_price_bp
}
