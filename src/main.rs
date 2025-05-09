use std::io;

// use msg::*;
use types::Leverage;
use types::Position;

mod constraints;
mod display;
mod types;

// mod msg {
//     pub const INVAILD_NUMBER_MSG: &str = "정수를 입력해주세요";
//     pub const INVAILD_RANGE_MSG: &str = "유효한 범위를 입력해주세요";
// }

fn get_input<T>(
    prompt: &str,
    parser: fn(&str) -> Option<T>,
    validator: fn(&T) -> bool,
    error_msg: &str,
) -> T {
    loop {
        println!("{}", prompt);

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("입력 실패");
        let trimmed = input.trim();

        if let Some(value) = parser(trimmed) {
            if validator(&value) {
                return value;
            }
        }

        println!("{}", error_msg);
    }
}

fn select_position() -> Position {
    get_input(
        "1. 포지션을 선택해주세요.\n1) Long, 2) Short",
        |s| match s {
            "1" => Some(Position::Long),
            "2" => Some(Position::Short),
            _ => None,
        },
        |_| true,
        "-> 1 또는 2를 입력해주세요.",
    )
}

fn select_leverage() -> Leverage {
    get_input (
        "2. 배율을 선택해주세요. \n1) 2X, 2) 3x",
        |s| match s {
            "1" => Some(Leverage::Daily2x),
            "2" => Some(Leverage::Daily3x),
            _ => None,
        },
        |_| true,
        "-> 1 또는 2를 입력해주세요.",
    )
}
fn main() {
    print_app_guide();

    let position = select_position();
    println!("{}을 선택하셨습니다.", position);
    let leverage = select_leverage();
    println!("{}을 선택하셨습니다.", leverage);
    let loss_rate = enter_loss_rate();
    let current_stock_price = enter_stock_price();

    let required_recovery_rate = calculate_recovery_rate(loss_rate, leverage);
    let target_stock_price =
        calculate_target_stock_price(position, required_recovery_rate, current_stock_price);

    print_calculation(required_recovery_rate, target_stock_price);
}

fn print_app_guide() {
    println!("---------------------");
    println!("레버리지에 따른 손실 회복 계산기");
    println!(
        "한 번의 하락없이 계속 상승한다는 가정하에 계산되었으며, 주식 시장 상황에 따른 괴리율로 인해 실제 주가는 산출된 기존 값을 훨씬 상회할 수 있음을 알려드립니다."
    );
    println!("---------------------");
}

fn print_calculation(required_recovery_rate: f64, target_stock_price: f64) {
    println!("계산 결과");
    println!("");
    println!(
        "-> 1. 레버리지 배율이 적용된 필요 회복율 : {}%",
        format!("{:.2}", required_recovery_rate * 100.0)
    );

    println!(
        "-> 2. 목표 본주 가격 : ${}",
        format!("{:.2}", target_stock_price)
    );
}

fn enter_loss_rate() -> f64 {
    loop {
        println!("3. 손실율을 입력해주세요");
        println!("");
        let mut input3 = String::new(); // 손실율 input

        io::stdin().read_line(&mut input3).expect("입력 실패");

        match input3.trim().parse::<f64>() {
            Ok(loss_rate)
                if loss_rate > constraints::MIN_LOSS_RATE
                    && loss_rate < constraints::MAX_LOSS_RATE =>
            {
                println!("-> 입력된 값 : {}%", loss_rate);
                return loss_rate;
            }
            Ok(_) => {
                println!("-> 1~99 사이의 손실율을 입력해주세요");
                continue;
            }
            Err(_) => {
                println!("-> 정수를 입력해주세요");
                continue;
            }
        };
    }
}

fn enter_stock_price() -> f64 {
    loop {
        println!("4. 본주의 가격을 입력하세요");
        println!("");

        let mut input4 = String::new(); // 본주 input

        io::stdin().read_line(&mut input4).expect("입력 실패");

        match input4.trim().parse::<f64>() {
            Ok(stock_price)
                if stock_price > constraints::MIN_STOCK_PRICE
                    && stock_price < constraints::MAX_STOCK_PRICE =>
            {
                println!("-> 입력된 값 : ${}", stock_price);
                return stock_price;
            }
            Ok(_) => {
                println!("-> 1~10000000 사이 가격을 입력해주세요");
                continue;
            }
            Err(_) => {
                println!("-> 정수를 입력해주세요.");
                continue;
            }
        };
    }
}

fn calculate_recovery_rate(loss_rate: f64, leverage: Leverage) -> f64 {
    (loss_rate / (100.0 - loss_rate)) / leverage.value()
}

fn calculate_target_stock_price(
    position: Position,
    required_recovery_rate_with_leverage: f64,
    current_stock_price: f64,
) -> f64 {
    match position {
        Position::Long => current_stock_price * (1.0 + required_recovery_rate_with_leverage),
        Position::Short => current_stock_price * (1.0 - required_recovery_rate_with_leverage),
    }
}
