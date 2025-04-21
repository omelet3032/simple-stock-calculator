use std::io;

fn main() {
    loop {
        let mut input1 = String::new(); // 롱 숏 선택 input
        let mut input2 = String::new(); // 배율 선택
        let mut input3 = String::new(); // 손실율 input
        let mut input4 = String::new(); // 본주 input

        // 1. 앱 실행
        println!("레버리지 손실 회복 계산기");

        // 2. 롱 or 숏 선택
        println!("1. 롱");
        println!("2. 숏");

        io::stdin().read_line(&mut input1).expect("입력 실패");
    }
}

fn deepseek1() {
    loop {
        let mut input1 = String::new(); // 롱 숏 선택 input
        let mut input2 = String::new(); // 배율 선택
        let mut input3 = String::new(); // 손실율 input
        let mut input4 = String::new(); // 본주 input

        // 1. 앱 실행
        println!("레버리지 손실 회복 계산기");

        // 2. 롱 or 숏 선택
        println!("1. 롱");
        println!("2. 숏");

        io::stdin().read_line(&mut input1).expect("입력 실패");

        // 1,롱 2 숏 외 나머지는 err로 보내기
        let position_choice = match input1.trim().parse::<i32>() {
            Ok(n) => {
                if n == 1 {
                    println!("롱");
                } else if n == 2 {
                    println!("숏");
                } else {
                    continue;
                }
                n
            }
            Err(_) => {
                println!("입력이 잘못되었습니다");
                continue;
            }
        };
    }
}

fn deepseek2() {
    loop {
        let mut input1 = String::new(); // 롱 숏 선택 input
        let mut input2 = String::new(); // 배율 선택
        let mut input3 = String::new(); // 손실율 input
        let mut input4 = String::new(); // 본주 input

        // 1. 앱 실행
        println!("레버리지 손실 회복 계산기");

        // 2. 롱 or 숏 선택
        println!("1. 롱");
        println!("2. 숏");

        io::stdin().read_line(&mut input1).expect("입력 실패");

        // 1,롱 2 숏 외 나머지는 err로 보내기
        let position_choice = match input1.trim().parse::<i32>() {
            Ok(n) if n == 1 || n == 2 => {
                if n == 1 {
                    println!("롱");
                } else if n == 2 {
                    println!("숏");
                }
                n
            }
            Ok(_) => {
                println!("1과 2중 하나를 입력하세요");
                continue;
            }
            Err(_) => {
                println!("정수를 입력하세요");
                continue;
            }
        };
    }
}

fn gpt1() {
    loop {
        let mut input1 = String::new();

        io::stdin().read_line(&mut input1).expect("에러");

        let position_choice;

        if let Ok(1) = input1.trim().parse::<i32>() {
            position_choice = 1;
        } else if let Ok(2) = input1.trim().parse::<i32>() {
            position_choice = 2;
        } else {
            println!("정수를 입력하세요.");
            continue;
        }
    }
}

fn gpt2() {
    loop {
        let mut input1 = String::new();

        io::stdin().read_line(&mut input1).expect("입력 실패");

        let parsed = input1.trim().parse::<i32>();

        let position_choice = if let Ok(n) = parsed {
            match n {
                1 => {
                    println!("롱");
                    1
                }
                2 => {
                    println!("숏");
                    2
                }
                _ => {
                    println!("1 또는 2");
                    continue;
                }
            }
        } else {
            println!("정수만 입력해주세요");
            continue;
        };
    }
}
