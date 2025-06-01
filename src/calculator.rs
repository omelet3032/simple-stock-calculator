use super::constraints::{MASTER_PRICISION_SCALE, PRICE_SCALE, RATE_SCALE};
use super::types::Leverage;
use super::types::Position;

pub fn calculate_target_stock_price(
    position: Position,
    leverage: Leverage,
    required_recovery_rate_bp: i64,
    user_entered_stock_price: f64,
) -> f64 {
    let stock_price_bp: i64 = convert_stock_price_to_bp(user_entered_stock_price);

    let recovery_rate_bp_scaled: i64 = (RATE_SCALE * MASTER_PRICISION_SCALE)
        // + ((required_recovery_rate_bp * MASTER_PRICISION_SCALE + (leverage.value() / 2)) / leverage.value());
        + (required_recovery_rate_bp * MASTER_PRICISION_SCALE) / leverage.value();
        // + (((required_recovery_rate_bp * MASTER_PRICISION_SCALE) + MASTER_PRICISION_SCALE) / leverage.value());

    // 정밀도 더 향상하자 recovery_rate_bp : 11666 -> 11667

    // let test = (required_recovery_rate_bp * MASTER_PRICISION_SCALE + (leverage.value() / 2)) / leverage.value();
    // println!("test : {}", test);    

    match position {
        Position::Long => {
            let target_price_bp: i64 =
                if let Some(price) = stock_price_bp.checked_mul(recovery_rate_bp_scaled) {
                    // let price_scaled = price * PRICE_SCALE;
                    // let decoupled_price_scale = MASTER_PRICISION_SCALE / PRICE_SCALE;
                    // price * decoupled_price_scale
                    // price / (MASTER_PRICISION_SCALE * PRICE_SCALE * RATE_SCALE)
                    // price / (MASTER_PRICISION_SCALE * PRICE_SCALE)
                    // price / MASTER_PRICISION_SCALE
                    price / (MASTER_PRICISION_SCALE * RATE_SCALE)
                } else {
                    panic!("Over Flow!!");
                };

            // target_price_bp as f64
            // target_price_bp as f64 / RATE_SCALE as f64 // 54166.25
            target_price_bp as f64 / PRICE_SCALE as f64 // 54166.25
        }
        Position::Short => {
            // stock_price_bp * (RATE_SCALE - (required_recovery_rate_bp + 1) / leverage.value())
            100.0
        }
    }
}

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
