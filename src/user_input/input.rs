use crate::types::Guide::*;
use crate::types::Invaild::*;
use crate::{
    constraints::{MAX_LOSS_RATE, MAX_STOCK_PRICE, MIN_LOSS_RATE, MIN_STOCK_PRICE},
    types::{Message, Sign, Country},
};


pub fn get_input_select<T: std::fmt::Display>(prompt: Message, parser: fn(&str) -> Option<T>) -> T {
    loop {
        println!("{}", prompt);

        let input = user_input();

        if let Some(value) = parser(&input) {
            println!("{}: {}\n", Message::GuideMessage(EnteredValue), value);
            return value;
        } else {
            println!("{}", Message::InvaildMessage(InvaildChoice));
            println!("{}: {}\n", Message::GuideMessage(EnteredValue), input);
        }
    }
}

pub fn get_input_rate(prompt: Message) -> f64 {
    loop {
        println!("{}", prompt);

        let input = user_input();

        match input.parse::<f64>() {
            Ok(value) => {
                if value > MIN_LOSS_RATE && value < MAX_LOSS_RATE {
                    println!("{}: {}%\n", Message::GuideMessage(EnteredValue), value);
                    return value;
                } else {
                    println!(
                        "{} ({}% ~ {}%)",
                        Message::InvaildMessage(InvaildRange),
                        MIN_LOSS_RATE,
                        MAX_LOSS_RATE
                    );
                    println!("{}: {}\n", Message::GuideMessage(EnteredValue), value);
                }
            }
            Err(_) => println!("{}", Message::InvaildMessage(InvaildNumber)),
        };
    }
}

/* 
    select_conuntry 값을 받아 출력 기호를 바꿔야 한다.
    화폐 기호 바뀔 시 신경써야 할 것
    1. 소수점 자리수 
        원 단위는 소수점x
        달러 단위는 소수점 필요
    
    사용자가 원 단위 입력시 1,000식으로 콤마를 입력할 수도 있겠네.. 아닌가?
*/
pub fn get_input_price(prompt: Message, country: Country) -> f64 {
    loop {
        println!("{}", prompt);

        let input = user_input();

        let sign:Sign = match country {
            Country::KR => Sign::Won,
            Country::US => Sign::Doller,
        };
        
        match input.parse::<f64>() {
            Ok(value) => {
                if value > MIN_STOCK_PRICE && value < MAX_STOCK_PRICE {
                    println!(
                        "{}: {}\n",
                        Message::GuideMessage(EnteredValue),
                        sign.format_value(value)
                    );
                    return value;
                } else {
                    println!(
                        "{} ({} ~ {})",
                        Message::InvaildMessage(InvaildRange),
                        sign.format_value(MIN_STOCK_PRICE),
                        sign.format_value(MAX_STOCK_PRICE)
                        // MIN_STOCK_PRICE,
                        // MAX_STOCK_PRICE,
                    );
                    println!("{}: {}\n", Message::GuideMessage(EnteredValue), value);
                }
            }
            Err(_) => println!("{}", Message::InvaildMessage(InvaildNumber)),
        };
    }
}

pub fn get_input_exit(promt:Message) -> bool {
    loop {
        println!("{}", promt);
        let input = user_input();

        if input.eq("Y") {
            return true;
        } else if input.eq("n") {
            return false;
        } else {
            println!("{}", Message::InvaildMessage(InvaildYn));
        }
    }

}

fn user_input() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("입력 실패");
    return input.trim().to_string();
}
