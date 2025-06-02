use super::constraints::{MASTER_PRICISION_SCALE, PRICE_SCALE, RATE_SCALE};
use super::types::Leverage;
use super::types::Position;
// use super::types::Country;

pub fn calculate_target_stock_price(
    // country:Country,
    position: Position,
    leverage: Leverage,
    required_recovery_rate_bp: i64,
    user_entered_stock_price: f64,
) -> f64 {
    let stock_price_bp: i64 = convert_stock_price_to_bp(user_entered_stock_price);

    let recovery_rate_bp_scaled: i64 = (RATE_SCALE * MASTER_PRICISION_SCALE)
        + ((required_recovery_rate_bp + leverage.value() / 2) * MASTER_PRICISION_SCALE)
            / leverage.value();

    let target_price:f64 = match position {
        Position::Long => {
            let target_price_bp: i128 = if let Some(price) =
                (stock_price_bp as i128).checked_mul(recovery_rate_bp_scaled as i128)
            {
                // price / (MASTER_PRICISION_SCALE * RATE_SCALE)
                price
            } else {
                panic!("Over Flow!!");
            };
            let target_price: i128 =
                target_price_bp / (MASTER_PRICISION_SCALE as i128 * RATE_SCALE as i128);
            // target_price_bp as f64 / PRICE_SCALE as f64
            target_price as f64 / PRICE_SCALE as f64
        }
        Position::Short => 100.0,
    };

    target_price

    // match country {
    //     Country::KR => {
    //         target_price.round() as i64
    //     }
    // }
}

/*
        let recovery_rate_bp_scaled: i64 = (RATE_SCALE * MASTER_PRICISION_SCALE)
       + ((required_recovery_rate_bp + leverage.value() / 2) * MASTER_PRICISION_SCALE) / leverage.value();
*/

pub fn calculate_leveraged_required_recovery_rate() {}

pub fn calculate_required_recovery_rate(user_entered_loss_rate: f64) -> (f64, i64) {
    let loss_rate_bp = convert_loss_rate_to_bp(user_entered_loss_rate); // 7000

    let required_recovery_rate_scaled: i64 =
        (loss_rate_bp * MASTER_PRICISION_SCALE) / (RATE_SCALE - loss_rate_bp);

    let required_recovery_rate_bp: i64 =
        (required_recovery_rate_scaled * RATE_SCALE) / MASTER_PRICISION_SCALE;

    // 출력용 f64 튜플로 반환?
    let required_recovery_rate = convert_recovery_rate_to_percentage(required_recovery_rate_bp);

    (required_recovery_rate, required_recovery_rate_bp) // 내부 계산용 -> calculate_target_price에서 사용
}

pub fn convert_recovery_rate_to_percentage(recovery_rate_bp: i64) -> f64 {
    let required_recovery_rate = recovery_rate_bp as f64 / PRICE_SCALE as f64;
    required_recovery_rate
}

pub fn convert_loss_rate_to_bp(user_entered_loss_rate: f64) -> i64 {
    let converted_loss_rate_for_percentage: f64 = user_entered_loss_rate / 100.0;

    let loss_rate_bp: i64 = (converted_loss_rate_for_percentage * RATE_SCALE as f64).round() as i64;

    loss_rate_bp
}

pub fn convert_stock_price_to_bp(user_entered_price: f64) -> i64 {
    let stock_price_bp: i64 = (user_entered_price * PRICE_SCALE as f64).round() as i64;

    stock_price_bp
}
