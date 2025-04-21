use std::io;

fn main() {
    loop {
        let mut input1 = String::new(); // 롱 숏 선택 input
        let mut input4 = String::new(); // 본주 input

        // 1. 앱 실행
        println!("레버리지 손실 회복 계산기");
        println!("---------------------");
        
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

        break;
    }

    loop {
        // 3. 배율 선택
        println!("레버리지 배율을 선택해주세요.");
        println!("1. 1x");
        println!("2. 2x");
        println!("3. 3x");


        let mut input2 = String::new(); // 배율 선택
        io::stdin().read_line(&mut input2).expect("입력 실패");

        let leverage = match input2.trim().parse::<i32>() {
            Ok(n) if n == 1 || n == 2 || n == 3 => {
                if n == 1 {
                    println!("1x를 선택하셨습니다");
                }
                else if n == 2 {
                    println!("2x를 선택하셨습니다.");
                }
                else if n == 3 {
                    println!("3x를 선택하셨습니다");
                }
                n
            },
            Ok(_) => {
                println!("1,2,3중 하나를 입력해주세요");
                continue;
            },
            Err(_) => {
                println!("정수를 입력해주세요.");
                continue;
            }
        };


        break;
    }

    loop {
        
        println!("손실율을 입력해주세요");
        let mut input3 = String::new(); // 손실율 input

        io::stdin().read_line(&mut input3).expect("입력 실패");

        let loss_rate = match input3.trim().parse::<i32>() {
            Ok(n) if n > 0 && n < 100 => {
                println!("{}%", n);
                n
            },
            Ok(_) => {
                println!("1~99사이의 정수를 입력해주세요");
                continue;
            }
            Err(_) => {
                println!("정수를 입력해주세요");
                continue;
            }
        };

        break;
    }
}

fn test() {
    loop {
        let mut input1 = String::new(); // 롱 숏 선택 input
        let mut input2 = String::new(); // 손실율 input
        let mut input3 = String::new(); // 본주 input

        // 1. 안내
        println!("레버리지 계산기 시작ㅓ");

        // 2. 롱 or 숏 선택
        println!("1. 롱");
        println!("2. 숏");

        io::stdin().read_line(&mut input1).expect("입력 실패");

        // let num: i32 = match input.trim().parse::<i32>() {
        let position_choice = match input1.trim().parse::<i32>() {
            Ok(n) => {
                if n == 1 {
                    println!("롱을 선택하셨습니다.");
                } else if n == 2 {
                    println!("숏을 선택하셨습니다.");
                } else {
                    println!("입력이 잘못되었습니다.");
                    continue;
                }
                n
            }
            Err(_) => {
                println!("입력이 잘못되었습니다");
                continue;
            }
        };
        // println!("입력 값 : {}", position_choice);
        println!("손실율을 입력하세요.");

        // 입력값을 2개를 받는다. 둘 다 정수
        // ex. 1 -> 10

        // 3. 손실율
        io::stdin().read_line(&mut input2).expect("입력 실패");

        let loss_rate = match input2.trim().parse::<i32>() {
            Ok(n) => {
                // println!("입력 값 : {}%", n);
                // 손실율은 100%를 넘어갈 수 없다.
                // 소수점일 수도 있다.
                // break;
                // return;
                n
            }
            Err(_) => {
                println!("잘못된 입력 : {}", input2.trim());
                continue;
            }
        };
        println!("{}%", loss_rate);

        // 4. 본주 가격 입력
        println!("본주의 가격을 입력하시오.");
        io::stdin().read_line(&mut input3).expect("입력 실패");

        let stock_price = match input3.trim().parse::<i32>() {
            Ok(n) => n,
            Err(_) => {
                println!("잘못된 입력 : {}", input3.trim());
                continue;
            }
        };
        println!("${}", stock_price);
        break;
    }
}
