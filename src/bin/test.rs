fn main() {


    calculate_long();
}

fn calculate_long() {
    let loss_rate = 0.7;
    let current_stock_price = 250.0;

    let leverage = 2.0;

    let required_recovery_rate = loss_rate / (1.0 - loss_rate);
    
    let final_recovery_rate = required_recovery_rate / leverage;

    let required_stock_price = current_stock_price * (1.0 + final_recovery_rate);
    
    println!("target_percentage : {}", required_recovery_rate);
    println!("current_price : {}", current_stock_price);
    println!("목표 가격 : {}", required_stock_price);
}


fn calculate_short() {
    let loss_rate = 0.3;
    let current_stock_price = 250.0;

    let leverage = 2.0;

    let target_percentage = loss_rate / (1.0 - loss_rate);
    
    let final_percentage = target_percentage / leverage;

    let required_price:f64 = current_stock_price * (1.0 - final_percentage);
    
    println!("target_percentage : {}", target_percentage);
    println!("current_price : {}", current_stock_price);
    println!("목표 가격 : {}", required_price);
}