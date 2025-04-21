use std::io;

fn main() {
    loop {
        let mut input = String::new();
        let mut input2 = String::new();
        // 1. 안내
        println!("레버리지 계산기");

        // 2. 롱 or 숏 선택
        println!("1. 롱");
        println!("2. 숏");

        io::stdin().read_line(&mut input2);

        // let num: i32 = match input.trim().parse::<i32>() {
        match input2.trim().parse::<i32>() {
            Ok(n) => {
                if n == 1 {
                    println!("롱을 선택하셨습니다.");
                } 
                else if n == 2 {
                    println!("숏을 선택하셨습니다.");
                } else {
                    println!("입력이 잘못되었습니다.");
                    continue;
                }
                n
            }
            Err(_) => {
                println!("입력이 잘못되었습니다");
                return;
            }
        };
        // println!("입력 값 : {num}");
        println!("손실율을 입력하세요.");

        // 입력값을 2개를 받는다. 둘 다 정수
        // ex. 1 -> 10

        // 3.
        io::stdin().read_line(&mut input).expect("입력 실패");

        match input.trim().parse::<i32>() {
            Ok(n) => {
                println!("입력 값 : {}%", n);
                // 손실율은 100%를 넘어갈 수 없다.
                // 소수점일 수도 있다.
                break;
            }
            Err(_) => {
                println!("잘못된 입력 : {}", input.trim());
                continue;
            }
        };
    }
}
