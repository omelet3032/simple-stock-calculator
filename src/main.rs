use simple_stock_calculator::{calculator::*, display::*, input::*};

fn main() {
    print_app_guide();

    let position = select_position();
    let leverage = select_leverage();

    let loss_rate = enter_loss_rate();
    let current_stock_price = enter_stock_price();

    let required_recovery_rate = calculate_recovery_rate(loss_rate, leverage);

    let target_stock_price = calculate_target_stock_price(
        position,
        required_recovery_rate.with_leverage,
        current_stock_price,
    );

    print_result(
        required_recovery_rate.recovery_rate,
        required_recovery_rate.with_leverage,
        target_stock_price,
    );
}
