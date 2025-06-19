use simple_stock_calculator::{
    calculator::*,
    display::*,
    types::{Country, StockInfo},
    user_input::*,
};

fn main() {
    loop {
        print_start();

        let country = select_country();

        let position = select_position();
        let leverage = select_leverage();

        let loss_rate = enter_loss_rate();

        let current_stock_price = enter_stock_price(&country);

        // 함수명 추후 수정
        let user_stock_info =
            calculate_user_stock_info(country, position, leverage, loss_rate, current_stock_price);

        print_result(user_stock_info);

        /*
           main.rs 앱 실행부 세분화하자
        */

        if select_exit() { break } else { continue }
    }
}
