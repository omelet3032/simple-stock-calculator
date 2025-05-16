fn main() {

}


// fn get_input_f64<T>(
//     prompt: &str,
//     parser: fn(f64) -> Option<T>,
//     validator: fn(&T) -> bool,
//     error_msg: &str,
// ) -> T {
//     loop {
//         println!("{}", prompt);

//         let mut input = String::new();
//         io::stdin().read_line(&mut input).expect("입력 실패");
//         // let trimmed = input.trim().parse::<f64>();

//         // 추상화 버전
//         if let Ok(num) = input.trim().parse::<f64>() {
//             if let Some(value) = parser(num) {
//                 if validator(&value) {
//                     return value;
//                 }
//             }
//         }

//         println!("{}", error_msg);
//     }
// }

// fn enter_loss_rate_input() -> f64 {
//     get_input_f64(
//         "3. 손실율을 입력해주세요",
//         |n|  {
//             if n > constraints::MIN_LOSS_RATE && n < constraints::MAX_LOSS_RATE {
//                 Some(n)
//             } else {

//                 None
//             }
//         },
//         |_| true,
//         "다시 입력해주세요",
//     )
// }

// fn get_input_select<T>(prompt: &str, parser: fn(&str) -> Option<T>, error_msg: &str) -> Option<T> {
//         println!("{}", prompt);

//         let mut input = String::new();

//         io::stdin().read_line(&mut input).expect("입력 실패");
//         let trimmed = input.trim();

//         if let Some(value) = parser(trimmed) {
//             Some(value)
//         } else {
//             println!("{}", error_msg);
//             None
//         }
// }

// 익명 함수(클로저)를 일반 함수로 변환 (공부용)
// fn parse_position(input: &str) -> Option<Position> {
//     match input {
//         "1" => Some(Position::Long),
//         "2" => Some(Position::Short),
//         _ => None,
//     }
// }


// fn get_input<T>(
//     prompt: &str,
//     parser: fn(&str) -> Option<T>,
//     validator: fn(&T) -> bool,
//     error_msg: &str,
// ) -> T {
//     loop {
//         println!("{}", prompt);

//         let mut input = String::new();
//         io::stdin().read_line(&mut input).expect("입력 실패");
//         let trimmed = input.trim();

//         // 추상화 버전
//         // if let Some(value) = parser(trimmed) {
//         //     if validator(&value) {
//         //         return value;
//         //     }
//         // }

//         match parser(trimmed) {
//             Some(v) => {
//                 let value = v;
//                 if validator(&value) {
//                     return value;
//                 }
//                 // return value;
//             }
//             None => {
//                 println!("{}", error_msg);
//             }
//         }
//     }
// }