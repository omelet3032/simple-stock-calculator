use std::io;

fn main() {
    // 1. 앱 실행
    println!("레버리지 손실 회복 계산기");
    println!("---------------------");

    let position = select_position();
    let leverage = select_leverage();
    let loss_rate = select_loss_rate();
    let current_stock_price = enter_stock_price();

    let required_recovery_rate = calculate_recovery_rate(loss_rate, leverage);
    // let objected_stock_price = calculate_objected_stock_price(position, recovery_rate, current_stock_price);
    let objected_stock_price = calculate_objected_stock_price(position, required_recovery_rate, current_stock_price);

    println!("필요 손실 회복율 : {}%", required_recovery_rate);
    println!("목표 주가 : ${}", objected_stock_price);


    // if position == 1 {
    //     let required_stock_price = calculate_long(loss_rate, leverage, current_stock_price);
    //     println!("목표 주식 가격 : {}", required_stock_price);
    // } else if position == 2 {
    //     let required_stock_price = calculate_short(loss_rate, leverage, current_stock_price);
    //     println!("목표 주식 가격 : {}", required_stock_price);
    // }

    // 여기에 결과가 나와야 함
    // let object_price
}

fn select_position() -> i32 {
    loop {
        let mut input1 = String::new(); // 롱 숏 선택 input

        // 2. 롱 or 숏 선택
        println!("포지션을 선택해주세요.");
        println!("1. 롱");
        println!("2. 숏");

        io::stdin().read_line(&mut input1).expect("입력 실패");

        let position_choice = match input1.trim().parse::<i32>() {
            Ok(n) if n == 1 || n == 2 => {
                if n == 1 {
                    println!("롱을 선택하셨습니다.");
                } else if n == 2 {
                    println!("숏을 선택하셨습니다.");
                }
                n
            }
            Ok(_) => {
                println!("1과 2 둘 중 하나를 입력해주세요.");
                continue;
            }
            Err(_) => {
                continue;
            }
        };

        break position_choice;
    }
}

fn select_leverage() -> f64 {
    loop {
        // 3. 배율 선택
        println!("레버리지 배율을 선택해주세요.");
        println!("1. 2x");
        println!("2. 3x");

        let mut input2 = String::new(); // 배율 선택
        io::stdin().read_line(&mut input2).expect("입력 실패");

        let leverage = match input2.trim().parse::<i32>() {
            Ok(1) => {
                println!("2x를 선택하셨습니다.");
                2.0
            }
            Ok(2) => {
                println!("3x를 선택하셨습니다.");
                3.0
            }
            Ok(_) => {
                println!("1,2중 하나를 입력해주세요");
                continue;
            }
            Err(_) => {
                println!("정수를 입력해주세요.");
                continue;
            }
        };

        break leverage;
    }
}

fn select_loss_rate() -> f64 {
    loop {
        println!("손실율을 입력해주세요");
        let mut input3 = String::new(); // 손실율 input

        io::stdin().read_line(&mut input3).expect("입력 실패");

        let loss_rate = match input3.trim().parse::<f64>() {
            Ok(n) if n > 0.0 && n < 100.0 => {
                println!("{}%", n);
                n
            }
            Ok(_) => {
                println!("1~99사이의 손실율을 입력해주세요");
                continue;
            }
            Err(_) => {
                println!("정수를 입력해주세요");
                continue;
            }
        };
        // 4. 레버리지 계산
        break loss_rate;
    }
}

fn enter_stock_price() -> f64 {
    loop {
        println!("본주의 가격을 입력하세요");

        let mut input4 = String::new(); // 본주 input

        io::stdin().read_line(&mut input4).expect("입력 실패");

        let stock_price = match input4.trim().parse::<f64>() {
            Ok(n) if n > 0.0 && n < 10000000.0 => {
                println!("${}", n);
                n
            }
            Ok(_) => {
                println!("1~10000000 사이 가격을 입력해주세요");
                continue;
            }
            Err(_) => {
                println!("정수를 입력해주세요.");
                continue;
            }
        };

        break stock_price;
    }
}

fn calculate_recovery_rate(loss_rate: f64, leverage: f64) -> f64 {
    let required_recovery_rate = loss_rate / (100.0 - loss_rate);

    let required_recovery_rate_with_leverage = required_recovery_rate / leverage;

    // println!("필요 회복율 : {}", required_recovery_rate_with_leverage);
    // return required_recovery_rate_with_leverage;
    required_recovery_rate_with_leverage
}

fn calculate_objected_stock_price(
    position: i32,
    required_recovery_rate_with_leverage: f64,
    current_stock_price: f64,
) -> f64 {
    
    let objected_stock_price = match position {
        1 => current_stock_price * (1.0 + required_recovery_rate_with_leverage),
        2 => current_stock_price * (1.0 - required_recovery_rate_with_leverage),
        // _ => panic!()
        _ => unreachable!()
    };
    // println!("목표 주가 : ${}", objected_stock_price);
    objected_stock_price
    // let required_stock_price: f64;
    
    // if position == 1 {

    //     let required_stock_price = current_stock_price * (1.0 + required_recovery_rate_with_leverage);
    //     return required_stock_price;
    // } else if position == 2 {
        
    //     let required_stock_price = current_stock_price * (1.0 - required_recovery_rate_with_leverage);
    //     return required_stock_price;
    // }
    // let required_stock_price = current_stock_price * (1.0 + required_recovery_rate_with_leverage);
    
    // println!("목표 가격 : {}", required_stock_price);

    // return required_stock_price;
}

// fn calculate_long(loss_rate: f64, leverage: f64, current_stock_price: f64) -> f64 {
//     let required_recovery_rate = loss_rate / (100.0 - loss_rate);

//     let required_recovery_rate_with_leverage = required_recovery_rate / leverage;
// }

fn calculate_short(loss_rate: f64, leverage: f64, current_stock_price: f64) -> f64 {
    let required_recovery_rate = loss_rate / (100.0 - loss_rate);

    let required_recovery_rate_with_leverage = required_recovery_rate / leverage;

    let required_stock_price = current_stock_price * (1.0 - required_recovery_rate_with_leverage);

    println!("필요 회복율 : {}", required_recovery_rate_with_leverage);
    println!("목표 가격 : {}", required_stock_price);

    return required_stock_price;
}
