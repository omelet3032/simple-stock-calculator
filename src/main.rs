use simple_stock_calculator::{calculator::*, display::*, input::*};

fn main() {

    print_app_guide();

    let position = select_position();
    let leverage = select_leverage();

    let loss_rate = enter_loss_rate();
    let current_stock_price = enter_stock_price();

    let required_recovery_rate = calculate_recovery_rate(loss_rate);

    let target_stock_price = calculate_target_stock_price(
        position,
        leverage,
        required_recovery_rate,
        current_stock_price,
    );

    print_result(loss_rate, current_stock_price,
        leverage,
        required_recovery_rate, 
        target_stock_price,
    );
}
