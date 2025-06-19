use crate::types::StockInfo;

use super::constraints::{MASTER_PRICISION_SCALE, PRICE_SCALE, RATE_SCALE};
use super::types::Country;
use super::types::Leverage;
use super::types::Position;

pub fn calculate_user_stock_info(
    country: Country,
    position: Position,
    leverage: Leverage,
    user_entered_loss_rate: f64,
    user_entered_stock_price: f64,
) -> StockInfo {
    let loss_rate_bp: i64 = convert_loss_rate_to_bp(user_entered_loss_rate);

    let stock_price_bp: i64 = convert_stock_price_to_bp(user_entered_stock_price);

    let required_recovery_rate_bp = calculate_required_recovery_rate_bp(loss_rate_bp);

    let leveraged_required_recovery_rate_bp =
        calculate_leveraged_required_recovery_rate_bp(required_recovery_rate_bp, &leverage);

    let target_underlying_stock_price: f64 = calculate_target_underlying_stock_price(
        &position,
        stock_price_bp,
        leveraged_required_recovery_rate_bp,
    );

    let target_underlying_stock_price_for_country = match country {
        Country::KR => target_underlying_stock_price.round(),
        Country::US => target_underlying_stock_price,
    };

    let user_stock_info = StockInfo {
        country:country,
        position:position,
        leverage:leverage,
        loss_rate:user_entered_loss_rate,
        current_underlying_stock_price:user_entered_stock_price,

        required_recovery_rate:convert_recovery_rate_to_percentage(required_recovery_rate_bp),
        target_underlying_stock_price:target_underlying_stock_price_for_country

    };

    user_stock_info
}

pub fn calculate_target_underlying_stock_price(
    position: &Position,
    stock_price_bp: i64,
    leveraged_required_recovery_rate_bp: i64,
) -> f64 {
    let multiplicand = stock_price_bp;

    let multiplier: i128 = ((RATE_SCALE * MASTER_PRICISION_SCALE) as i128)
        + ((leveraged_required_recovery_rate_bp * MASTER_PRICISION_SCALE) as i128);

    let target_underlying_stock_price: f64 = match position {
        Position::Long => {
            let target_underlying_stock_price_bp_scaled: i128 =
                if let Some(price) = (multiplicand as i128).checked_mul(multiplier as i128) {
                    price
                } else {
                    panic!("Over Flow!!");
                };
            let target_underlying_stock_price_bp_unscaled_master_scale: i128 =
                target_underlying_stock_price_bp_scaled / MASTER_PRICISION_SCALE as i128;

            let target_underlying_stock_price_bp_unscaled_rate_scale: i128 =
                target_underlying_stock_price_bp_unscaled_master_scale / RATE_SCALE as i128;

            let final_price =
                target_underlying_stock_price_bp_unscaled_rate_scale as f64 / PRICE_SCALE as f64;

            final_price
        }
        Position::Short => 100.0,
    };

    target_underlying_stock_price
}

pub fn scale_leveraged_required_recovery_rate_bp(leveraged_recovery_rate_bp: i64) -> i64 {
    let scaled_leveraged_required_recovery_rate_bp = (RATE_SCALE * MASTER_PRICISION_SCALE)
        + (leveraged_recovery_rate_bp * MASTER_PRICISION_SCALE);

    scaled_leveraged_required_recovery_rate_bp
}

pub fn calculate_leveraged_required_recovery_rate_bp(
    required_recovery_rate_bp: i64,
    leverage: &Leverage,
) -> i64 {
    let leveraged_recovery_rate_bp: i64 = ((required_recovery_rate_bp + (leverage.value() / 2))
        * MASTER_PRICISION_SCALE)
        / leverage.value();

    leveraged_recovery_rate_bp / MASTER_PRICISION_SCALE
}

pub fn calculate_required_recovery_rate_bp(loss_rate_bp: i64) -> i64 {
    let required_recovery_rate_scaled: i64 =
        (loss_rate_bp * MASTER_PRICISION_SCALE) / (RATE_SCALE - loss_rate_bp);

    let required_recovery_rate_bp: i64 =
        (required_recovery_rate_scaled * RATE_SCALE) / MASTER_PRICISION_SCALE;

    required_recovery_rate_bp
}

pub fn convert_recovery_rate_to_percentage(recovery_rate_bp: i64) -> f64 {
    let required_recovery_rate = recovery_rate_bp as f64 / PRICE_SCALE as f64;
    required_recovery_rate
}

pub fn convert_stock_price_to_bp(user_entered_price: f64) -> i64 {
    let stock_price_bp = (user_entered_price * PRICE_SCALE as f64).round() as i64;

    stock_price_bp
}

pub fn convert_loss_rate_to_bp(user_entered_loss_rate: f64) -> i64 {
    let converted_loss_rate_for_percentage: f64 = user_entered_loss_rate / 100.0;

    let loss_rate_bp: i64 = (converted_loss_rate_for_percentage * RATE_SCALE as f64).round() as i64;

    loss_rate_bp
}
