use simple_stock_calculator::{calculator::*, display::*, user_input::*};

fn main() {
    loop {
        print_start();

        let country = select_country();

        let position = select_position();
        let leverage = select_leverage();

        let loss_rate = enter_loss_rate();

        let current_stock_price = enter_stock_price(&country);
        //  생각해보면 회복율 계산을 외부로 노출할 필요 없다.
        //  내부 계산용으로 쓰고 출력.. 아닌가
        // 노출할 필요가 있기도 하고..
        // let required_recovery_rate = calculate_required_recovery_rate(loss_rate);
        let (required_recovery_rate, required_recovery_rate_bp) = calculate_required_recovery_rate(loss_rate);

        /* 
            목표 주가 계산 함수는 사용자가 입력한 손실율과 주가를 받는게 자연스럽다,
            회복율 계산 로직을 클라이언트가 할 필요는 없으니 노출할 필요가 없다.
         */
        /* 
            회복욜 계산 함수를 main.rs에서 제거하고
            target_stock_price 함수의 파라미터를 loss_rate로 바꾼다.
            calculator.rs에서 target_stock_price의 로직을 바꾼다.
                target~price 함수 내부에서 회복율을 구하고 외부에는 회복율 로직을 볼 수 없게 한다.
            target_price 함수도 내부로 가리고 main.rs의 함수 명칭은 calculate_result 이런식으로 추상화한다.
            calculate_result 함수의 반환 값은 회복율, 레버리지 적용 회복율, 목표 주가로 한다.
         */
        let target_stock_price =
            calculate_target_stock_price(position, leverage, required_recovery_rate_bp, current_stock_price);

        print_result(
            country,
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
