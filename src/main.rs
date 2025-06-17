use simple_stock_calculator::{calculator::*, display::*, types::StockInfo, user_input::*};

fn main() {
    loop {
        print_start();

        let country = select_country();

        let position = select_position();
        let leverage = select_leverage();

        let loss_rate = enter_loss_rate();

        let current_stock_price = enter_stock_price(&country);
       
        // let user_stock_info:StockInfo = StockInfo {
        //     country: country,
        //     price: current_stock_price,
        // };

        let target_stock_price =
            calculate_result(country, position, leverage, loss_rate, current_stock_price);

        println!("{}", target_stock_price);
        
/*         print_result(
            loss_rate, //f64
            current_stock_price, //f64
            leverage, // Leverage
            required_recovery_rate, // f64
            target_stock_price, // i64
        ); */

        /* 
            main.rs 앱 실행부 세분화하자
            print_result 파라미터는 튜플로 전달?
         */

        if select_exit() { break } else { continue }
    }
}
