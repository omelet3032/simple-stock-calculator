use crate::types::Guide::*;
use crate::types::Invaild::*;
use crate::types::StockInfo;
// use crate::types::UserInputPrice;
use crate::{
    constraints::{MAX_LOSS_RATE, MAX_STOCK_PRICE, MIN_LOSS_RATE, MIN_STOCK_PRICE},
    types::{Country, CurrencySign, Message},
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

pub fn parse_input_price(prompt: Message, country: &Country) -> f64 {
    loop {
        println!("{}", prompt);

        let input = user_input();

        let price_parsed_f64 = match country {
            Country::KR => match input.parse::<i64>() {
                Ok(value) => value as f64,
                Err(_) => {
                    println!("{}", Message::InvaildMessage(InvaildInt));
                    continue;
                }
            },
            Country::US => match input.parse::<f64>() {
                Ok(value) => value,
                Err(_) => {
                    println!("{}", Message::InvaildMessage(InvaildNumber));
                    continue;
                }
            },
        };

        let is_in_vaild_range =
            price_parsed_f64 > MIN_STOCK_PRICE && price_parsed_f64 < MAX_STOCK_PRICE;

        let currency_sign = match country {
            Country::KR => CurrencySign::Won,
            Country::US => CurrencySign::Doller,
        };

        if is_in_vaild_range {
            println!(
                "{}: {}\n",
                Message::GuideMessage(EnteredValue),
                currency_sign.format_value(price_parsed_f64),
            );
            return price_parsed_f64;
        } else {
            println!("{}", Message::InvaildMessage(InvaildRange));
            println!(
                "({} ~ {})",
                currency_sign.format_value(MIN_STOCK_PRICE),
                currency_sign.format_value(MAX_STOCK_PRICE)
            );
            println!(
                "{}: {}\n",
                Message::GuideMessage(EnteredValue),
                price_parsed_f64
            );
            continue;
        }

        // let final_price = match country {
        //     Country::KR => {
        //         if price_parsed_f64 > MIN_STOCK_PRICE && price_parsed_f64 < MAX_STOCK_PRICE {
        //             println!(
        //                 "{}: {}\n",
        //                 Message::GuideMessage(EnteredValue),
        //                 CurrencySign::Won.format_value(price_parsed_f64),
        //             );
        //             price_parsed_f64
        //         } el println!("{}", Message::InvaildMessage(InvaildRange));
        //             println!(
        //                 "({} ~ {})",
        //                 CurrencySign::Won.format_value(MIN_STOCK_PRICE),
        //                 CurrencySign::Won.format_value(MAX_STOCK_PRICE)
        //             );
        //             println!(
        //                 "{}: {}\n",
        //                 Message::GuideMessage(EnteredValue),
        //                 price_parsed_f64
        //             );
        //             continue;se {
        //             println!("{}", Message::InvaildMessage(InvaildRange));
        //             println!(
        //                 "({} ~ {})",
        //                 CurrencySign::Won.format_value(MIN_STOCK_PRICE),
        //                 CurrencySign::Won.format_value(MAX_STOCK_PRICE)
        //             );
        //             println!(
        //                 "{}: {}\n",
        //                 Message::GuideMessage(EnteredValue),
        //                 price_parsed_f64
        //             );
        //             continue;
        //         }
        //     }
        //     Country::US => {
        //         if price_parsed_f64 > MIN_STOCK_PRICE && price_parsed_f64 < MAX_STOCK_PRICE {
        //             println!(
        //                 "{}: {}\n",
        //                 Message::GuideMessage(EnteredValue),
        //                 CurrencySign::Doller.format_value(price_parsed_f64),
        //             );
        //             price_parsed_f64
        //         } else {
        //             println!("{}", Message::InvaildMessage(InvaildRange));
        //             println!(
        //                 "({} ~ {})",
        //                 CurrencySign::Doller.format_value(MIN_STOCK_PRICE),
        //                 CurrencySign::Doller.format_value(MAX_STOCK_PRICE)
        //             );
        //             println!(
        //                 "{}: {}\n",
        //                 Message::GuideMessage(EnteredValue),
        //                 price_parsed_f64
        //             );
        //             continue;
        //         }
        //     }
        // };

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
