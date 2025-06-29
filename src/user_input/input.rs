use std::io;

use crate::application::ErrorType;
use crate::types::Guide::EnteredValue;
use crate::types::Invalid::{InvalidChoice, InvalidInt, InvalidNumber, InvalidRange, InvalidYn};
use crate::{
    constraints::{MAX_LOSS_RATE, MAX_STOCK_PRICE, MIN_LOSS_RATE, MIN_STOCK_PRICE},
    types::{Country, CurrencySign, Message},
};

pub fn get_input_select<T: std::fmt::Display>(
    prompt: Message,
    parser: fn(&str) -> Result<T, ErrorType>,
) -> Result<T, ErrorType> {
    println!("{}", prompt);

    let input = user_input()?;

    let value = parser(&input)?;
    println!("{}: {}\n", Message::GuideMessage(EnteredValue), value);
    Ok(value)
}

pub fn get_input_rate(prompt: Message) -> Result<f64, ErrorType> {
    // loop {
        println!("{}", prompt);

        let input = user_input()?;

        match input.parse::<f64>() {
            Ok(value) => {
                if value > MIN_LOSS_RATE && value < MAX_LOSS_RATE {
                    println!("{}: {}%\n", Message::GuideMessage(EnteredValue), value);
                    value
                } else {
                    println!(
                        "{} ({}% ~ {}%)",
                        Message::InvalidMessage(InvalidRange),
                        MIN_LOSS_RATE,
                        MAX_LOSS_RATE
                    );
                    println!("{}: {}\n", Message::GuideMessage(EnteredValue), value);
                    // ErrorType::InvalidInputFormat("유효한 범위가 아닙니다.".to_string())
                    value
                    // continue
                }
            }
            // Err(_) => println!("{}", Message::InvalidMessage(InvalidNumber)),
            Err(_) => ErrorType::InvalidInputFormat("유효한 숫자가 아닙니다.".to_string())
        };
    // }
}

pub fn get_input_price(prompt: Message, country: &Country) -> f64 {
    loop {
        println!("{}", prompt);

        let input = user_input();

        let price_parsed_f64 = match country {
            Country::KR => match input.parse::<i64>() {
                Ok(value) => value as f64,
                Err(_) => {
                    println!("{}", Message::InvalidMessage(InvalidInt));
                    continue;
                }
            },
            Country::US => match input.parse::<f64>() {
                Ok(value) => value,
                Err(_) => {
                    println!("{}", Message::InvalidMessage(InvalidNumber));
                    continue;
                }
            },
        };

        let is_in_vaild_range =
            price_parsed_f64 > MIN_STOCK_PRICE && price_parsed_f64 < MAX_STOCK_PRICE;

        let price = match country {
            Country::KR => CurrencySign::Won,
            Country::US => CurrencySign::Dollar,
        };

        if is_in_vaild_range {
            println!(
                "{}: {}\n",
                Message::GuideMessage(EnteredValue),
                price.with_currency_sign(price_parsed_f64),
            );
            return price_parsed_f64;
        } else {
            println!("{}", Message::InvalidMessage(InvalidRange));
            println!(
                "({} ~ {})",
                price.with_currency_sign(MIN_STOCK_PRICE),
                price.with_currency_sign(MAX_STOCK_PRICE)
            );
            println!(
                "{}: {}\n",
                Message::GuideMessage(EnteredValue),
                price.with_currency_sign(price_parsed_f64),
            );
            continue;
        }
    }
}

pub fn get_input_exit(prompt: Message) -> bool {
    loop {
        println!("{}", prompt);
        let input = user_input();

        if input.eq("Y") {
            return true;
        } else if input.eq("n") {
            return false;
        } else {
            println!("{}", Message::InvalidMessage(InvalidYn));
        }
    }
}

fn user_input() -> Result<String, io::Error> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}
