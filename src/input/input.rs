use crate::types::{Message, Sign};

pub fn get_input_select<T: std::fmt::Display>(prompt: Message, parser: fn(&str) -> Option<T>) -> T {
    loop {
        println!("{}", prompt);

        let input = user_input();

        if let Some(value) = parser(&input) {
            println!("{}: {}\n", Message::EnteredValue, value);
            return value;
        } else {
            println!("{}", Message::InvaildChoice);
            println!("{}: {}\n", Message::EnteredValue, input);
        }
    }
}

pub fn get_input_rate(prompt: Message, sign: Sign, min_num: f64, max_num: f64) -> f64 {
    loop {
        println!("{}", prompt);

        let input = user_input();

        match input.parse::<f64>() {
            Ok(value) => {
                if value > min_num && value < max_num {
                    println!("{}: {}\n", Message::EnteredValue, sign.format_value(value));
                    return value;
                } else {
                    println!(
                        "{} ({} ~ {})",
                        Message::InvaildRange,
                        sign.format_value(min_num),
                        sign.format_value(max_num)
                    );
                    println!("{}: {}\n", Message::EnteredValue, value);
                }
            }
            Err(_) => println!("{}", Message::InvaildNumber),
        };
    }
}

fn user_input() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("입력 실패");
    return input.trim().to_string();
}
