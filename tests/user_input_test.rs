use simple_stock_calculator::types::{Country, Menu};
use simple_stock_calculator::user_input::*;
use simple_stock_calculator::user_input::input::get_input_select;
use simple_stock_calculator::types::Message::{self, MenuMessage};

/* 
사용자가 입력한 숫자를 받아 select_country의 display가 출력이 되는지 확인한다.
*/
// pub fn get_input_select<T: std::fmt::Display>(prompt: Message, parser: fn(&str) -> Option<T>) -> T {
//     loop {
//         println!("{}", prompt);

//         let input = user_input();

//         if let Some(value) = parser(&input) {
//             println!("{}: {}\n", Message::GuideMessage(EnteredValue), value);
//             return value;
//         } else {
//             println!("{}", Message::InvaildMessage(InvaildChoice));
//             println!("{}: {}\n", Message::GuideMessage(EnteredValue), input);
//         }pub fn select_country() -> Country {
/*     get_input_select(Message::MenuMessage(SelectCountry), |c| match c {
        "1" => Some(Country::KR),
        "2" => Some(Country::US),
        _ => None,
    })
}
untry {
    get_input_select(Message::MenuMessage(SelectCountry), |c| match c {
        "1" => Some(Country::KR),
        "2" => Some(Country::US),
        _ => None,
    })
} */

// #[test]
// fn test_select_country() {

//     let mock_input_1 = || "1".to_string();

//     assert_eq!(get_input_select(MenuMessage(Menu::SelectCountry), mock_input_1), "1. 국가를 선택해주세요. 1) KR, 2)US");
// }