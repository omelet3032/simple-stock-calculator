use simple_stock_calculator::{calculator::*, display::*, user_input::*};

fn main() {
    loop {
        print_start();

        let position = select_position();
        let leverage = select_leverage();

        let loss_rate = enter_loss_rate();

        let current_stock_price = enter_stock_price();
        //  생각해보면 회복율 계산을 외부로 노출할 필요 없다.
        //  내부 계산용으로 쓰고 출력.. 아닌가
        // 노출할 필요가 있기도 하고..
        // let required_recovery_rate = calculate_required_recovery_rate(loss_rate);
        let (required_recovery_rate, required_recovery_rate_bp) = calculate_required_recovery_rate(loss_rate);

        /* 
            목표 주가 계산 함수는 사용자가 입력한 손실율과 주가를 받는게 자연스럽다,
            회복율 계산 로직을 클라이언트가 할 필요는 없으니 노출할 필요가 없다.
         */
        let target_stock_price =
            calculate_target_stock_price(position, leverage, required_recovery_rate_bp, current_stock_price);

        print_result(
            loss_rate, //f64
            current_stock_price, //f64
            leverage, // Leverage
            required_recovery_rate, // f64
            target_stock_price, // i64
        );

        /* 
            main.rs 앱 실행부 세분화하자
            print_result 파라미터는 튜플로 전달?
         */

        if select_exit() { break } else { continue }
    }
}
