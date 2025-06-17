use crate::types::Guide::*;
use crate::types::Invaild::*;
use crate::types::UserInputPrice;
use crate::{
    constraints::{MAX_LOSS_RATE, MAX_STOCK_PRICE, MIN_LOSS_RATE, MIN_STOCK_PRICE},
    types::{Country, CurrencySign, Message, StockInfo},
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

pub fn get_input_country(prompt: Message, country: &Country) -> UserInputPrice {
    loop {
        println!("{}", prompt);

        let input = user_input();

        match country {
            Country::KR => match input.parse::<i64>() {
                Ok(value) => {
                    println!("\n");
                    return UserInputPrice::Integer(value);
                }
                Err(_) => {
                    println!("에러");
                    continue;
                }
            },
            Country::US => match input.parse::<f64>() {
                Ok(value) => {
                    println!("\n");
                    return UserInputPrice::Float(value);
                }
                Err(_) => {
                    println!("에러1");
                    continue;
                }
            },
        };
    }
}

pub fn get_input_price(price: UserInputPrice) -> UserInputPrice {
    loop {
        
        match price {
            UserInputPrice::Integer(value) => {
                if (value as f64) > MIN_STOCK_PRICE && (value as f64) < MAX_STOCK_PRICE {
                    println!(
                        "{}: {}\n",
                        Message::GuideMessage(EnteredValue),
                        CurrencySign::Won.format_value(value as f64)
                    );
                    return UserInputPrice::Integer(value);
                } else {
                    println!(
                        "{} ({} ~ {})",
                        Message::InvaildMessage(InvaildRange),
                        CurrencySign::Won.format_value(MIN_STOCK_PRICE),
                        CurrencySign::Won.format_value(MAX_STOCK_PRICE)
                    );
                    println!("{}: {}\n", Message::GuideMessage(EnteredValue), value);
                }
            }
            UserInputPrice::Float(value) => {
                if (value as f64) > MIN_STOCK_PRICE && (value as f64) < MAX_STOCK_PRICE {
                    println!(
                        "{}: {}\n",
                        Message::GuideMessage(EnteredValue),
                        CurrencySign::Doller.format_value(value as f64)
                    );
                    return UserInputPrice::Float(value);
                } else {
                    println!(
                        "{} ({} ~ {})",
                        Message::InvaildMessage(InvaildRange),
                        CurrencySign::Doller.format_value(MIN_STOCK_PRICE),
                        CurrencySign::Doller.format_value(MAX_STOCK_PRICE)
                    );
                    println!("{}: {}\n", Message::GuideMessage(EnteredValue), value);
                }
            }
        };
    }
}

pub fn get_input_exit(promt: Message) -> bool {
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
