use rust_decimal::Decimal;
use rust_decimal::prelude::*;

use super::constraints::{MASTER_PRECISION_SCALE, PRICE_SCALE, RATE_SCALE};
use super::types::{StockInfo, Country, Leverage, Position};

pub fn generate_user_stock_info(
    country: Country,
    position: Position,
    leverage: Leverage,
    user_entered_loss_rate: f64,
    user_entered_stock_price: f64,
) -> StockInfo {
    
    let loss_rate_bp: i64 = convert_loss_rate_to_bp(user_entered_loss_rate);

    let stock_price_bp: i64 = convert_stock_price_to_bp(user_entered_stock_price);

    let required_recovery_rate_with_master_precision_scale =
        calculate_required_recovery_rate_with_master_precision_scale(loss_rate_bp);

    let leveraged_required_recovery_rate_with_master_precision_scale =
        calculate_leveraged_required_recovery_rate_with_master_precision_scale(
            required_recovery_rate_with_master_precision_scale,
            &leverage,
        );

    let target_underlying_stock_price: f64 = calculate_target_underlying_stock_price(
        &position,
        stock_price_bp,
        leveraged_required_recovery_rate_with_master_precision_scale,
    );

    let target_underlying_stock_price_for_country = match country {
        Country::KR => target_underlying_stock_price.round(),
        Country::US => target_underlying_stock_price,
    };

    let user_stock_info = StockInfo {
        country: country,
        position: position,
        leverage: leverage,
        loss_rate: user_entered_loss_rate,
        current_underlying_stock_price: user_entered_stock_price,

        required_recovery_rate: convert_rate_to_percentage(
            required_recovery_rate_with_master_precision_scale,
        ),
        leveraged_required_recovery_rate: convert_rate_to_percentage(
            leveraged_required_recovery_rate_with_master_precision_scale,
        ),
        target_underlying_stock_price: target_underlying_stock_price_for_country,
    };

    user_stock_info
}

pub fn calculate_target_underlying_stock_price(
    position: &Position,
    stock_price_bp: i64,
    leveraged_required_recovery_rate_bp_with_master_precision_scale: i64,
) -> f64 {
    let multiplicand = stock_price_bp * MASTER_PRECISION_SCALE; 

    let multiplier: i128 = (1 * MASTER_PRECISION_SCALE as i128)
        + (leveraged_required_recovery_rate_bp_with_master_precision_scale as i128); 

    let target_underlying_stock_price: f64 = match position {
        Position::Long => {
            let target_underlying_stock_price_bp_scaled: i128 =
                if let Some(price) = (multiplicand as i128).checked_mul(multiplier as i128) {
                    price
                } else {
                    panic!("Over Flow!!");
                };

            let target_underlying_stock_price =
                unscale_target_underlying_stock_price(target_underlying_stock_price_bp_scaled);

            (target_underlying_stock_price * 100.0).round() / 100.0
        }
        Position::Short => 100.0,
    };

    target_underlying_stock_price

}

pub fn unscale_target_underlying_stock_price(target_underlying_stock_price_bp_scaled: i128) -> f64 {
    let target_underlying_stock_price_bp_unscaled_by_master_precision_scale =
        target_underlying_stock_price_bp_scaled as f64
            / (MASTER_PRECISION_SCALE as f64 * MASTER_PRECISION_SCALE as f64);

    let unscaled_target_underlying_stock_price =
        target_underlying_stock_price_bp_unscaled_by_master_precision_scale / PRICE_SCALE as f64;

    unscaled_target_underlying_stock_price
}

pub fn calculate_leveraged_required_recovery_rate_with_master_precision_scale(
    required_recovery_rate_with_master_precision_scale: i64,
    leverage: &Leverage,
) -> i64 {
    let leveraged_recovery_rate_bp_with_master_precision_scale: i64 =
        (required_recovery_rate_with_master_precision_scale + (leverage.value() / 2))
            / leverage.value();

    leveraged_recovery_rate_bp_with_master_precision_scale
}

pub fn calculate_required_recovery_rate_with_master_precision_scale(loss_rate_bp: i64) -> i64 {
    let required_recovery_rate_bp_with_master_precision_scale: i64 =
        (loss_rate_bp * MASTER_PRECISION_SCALE) / (RATE_SCALE - loss_rate_bp);

    required_recovery_rate_bp_with_master_precision_scale
}

pub fn convert_rate_to_percentage(rate_bp_with_master_precision_scaled: i64) -> f64 {
    let required_rate_bp_unscaled =
        rate_bp_with_master_precision_scaled as f64 / MASTER_PRECISION_SCALE as f64;
    let required_rate_bp = (required_rate_bp_unscaled * RATE_SCALE as f64).round();
    let required_rate = required_rate_bp / 100.0;
    required_rate
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
