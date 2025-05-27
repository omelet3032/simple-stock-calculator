use simple_stock_calculator::{calculator::*, display::*, user_input::*};

fn main() {
    loop {
        print_start();

        let position = select_position();
        let leverage = select_leverage();

        let loss_rate = enter_loss_rate();
        let current_stock_price = enter_stock_price();

        let recovery_rate = calculate_recovery_rate(loss_rate);

        let target_stock_price =
            calculate_target_stock_price(position, leverage, recovery_rate, current_stock_price);

        print_result(
            loss_rate,
            current_stock_price,
            leverage,
            recovery_rate,
            target_stock_price,
        );

        if select_exit() { break } else { continue }
    }
}
