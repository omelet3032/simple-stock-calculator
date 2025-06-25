use std::error::Error;
use std::fmt;

use super::calculator::generate_user_stock_info;
use super::display::{print_result, print_start};
use super::types::{Country, Leverage, Position};
use super::user_input::*;

enum ErrorType {
    AppNameBlank,
    ExecuteError,
    OverFlow,
}

impl fmt::Display for ErrorType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ErrorType::AppNameBlank => write!(f, "앱 이름이 없습니다."),
            ErrorType::ExecuteError => write!(f, "실행 오류."),
            ErrorType::OverFlow => write!(f, "over flow"),
        }
    }
}

// impl Error for AppError {


// }

// 앱의 핵심 로직을 담는 구조체 (옵션)
pub struct App {
    // 앱 상태나 설정 등
    name: String,
}

impl App {
    pub fn new(name: &str) -> Result<Self, ErrorType> {

            if name.is_empty() {
                return Err(AppError("앱 이름은 비어 있을 수 없습니다.".to_string()));
                // return ErrorType::AppNameBlank;
            }
    
            Ok(App {
                name: name.to_string(),
            })
            
    }

    pub fn execute_logic(&self) -> Result<(), ErrorType> {
        println!("{} 앱이 핵심 로직을 실행합니다.", self.name);
        loop {
            print_start();
            let country: Country = select_country();
            let position: Position = select_position();
            let leverage: Leverage = select_leverage();
            let loss_rate: f64 = enter_loss_rate();
            let current_underlying_stock_price: f64 = enter_stock_price(&country);

            let user_stock_info = generate_user_stock_info(
                country,
                position,
                leverage,
                loss_rate,
                current_underlying_stock_price,
            );

            print_result(user_stock_info);
            if select_exit() { break } else { continue }
        }

        Ok(())
    }
}

// 앱의 진입점 역할을 하는 함수
pub fn run() -> Result<(), Box<dyn Error>> {
    println!("애플리케이션이 시작됩니다.");

    // let my_app = App::new("simple-stock-calculator")?;
    let my_app = App::new("simple-stock-calculator");
    // my_app.execute_logic()?;
    my_app.execute_logic();

    println!("애플리케이션이 성공적으로 종료되었습니다.");
    Ok(())
}
