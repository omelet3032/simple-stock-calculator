use std::fmt;
use std::io;

use constraints::*;
use msg::*;


fn get_input<T>
(prompt: &str,
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

mod msg {
    pub const INVAILD_NUMBER_MSG:&str = "정수를 입력해주세요";
    pub const INVAILD_RANGE_MSG:&str = "유효한 범위를 입력해주세요";
}
mod constraints {
    pub const MIN_LOSS_RATE: f64 = 0.0;
    pub const MAX_LOSS_RATE: f64 = 100.0;
    pub const MIN_STOCK_PRICE: f64 = 0.0;
    pub const MAX_STOCK_PRICE: f64 = 10_000_000.0;
}

impl fmt::Display for Leverage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Leverage::Daily2x => write!(f, "2X"),
            Leverage::Daily3x => write!(f, "3X"),
        }
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Position::Long => write!(f, "Long"),
            Position::Short => write!(f, "Short"),
        }
    }
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
enum Position {
    Long,
    Short,
}

enum Leverage {
    Daily2x,
    Daily3x,
}

impl Leverage {
    fn value(&self) -> f64 {
        match self {
            Leverage::Daily2x => 2.0,
            Leverage::Daily3x => 3.0,
        }
    }
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

// fn select_position() -> Position {
//     loop {
//         let mut input1 = String::new(); // 롱 숏 선택 input

//         // 2. 롱 or 숏 선택
//         println!("1. 포지션을 선택해주세요.");
//         println!("");
//         println!("1) Long, 2) Short");

//         io::stdin().read_line(&mut input1).expect("입력 실패");

//         match input1.trim() {
//             "1" => return Position::Long,
//             "2" => return Position::Short,
//             _ => {
//                 println!("1과 2중 하나를 입력해주세요.");
//                 continue;
//             }
//         }
//     }
// }

fn select_leverage() -> Leverage {
    loop {
        // 3. 배율 선택
        println!("2. 레버리지 배율을 선택해주세요.");
        println!("");
        println!("1) x2, 2) x3");

        let mut input2 = String::new(); // 배율 선택
        io::stdin().read_line(&mut input2).expect("입력 실패");

        // position은 문자열 처리, leverage는 정수 처리 (공부용)
        match input2.trim().parse::<i32>() {
            Ok(1) => {
                // println!("Leverage: {}를 선택하셨습니다.", Leverage::Daily2x);
                return Leverage::Daily2x;
            }
            Ok(2) => {
                println!("Leverage: {}를 선택하셨습니다.", Leverage::Daily3x);
                return Leverage::Daily3x;
            }
            _ => {
                println!("1 또는 2를 입력해주세요.");
                continue;
            }
        };
    }
}

fn enter_loss_rate() -> f64 {
    loop {
        println!("3. 손실율을 입력해주세요");
        println!("");
        let mut input3 = String::new(); // 손실율 input

        io::stdin().read_line(&mut input3).expect("입력 실패");

        match input3.trim().parse::<f64>() {
            Ok(loss_rate) if loss_rate > MIN_LOSS_RATE && loss_rate < MAX_LOSS_RATE => {
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
            Ok(stock_price) if stock_price > MIN_STOCK_PRICE && stock_price < MAX_STOCK_PRICE => {
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
