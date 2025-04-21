use std::io;
fn main () {
    let mut input = String::new();

    println!("손실율을 입력하세요");
    io::stdin().read_line(&mut input).expect("입력 실패");

    println!("입력한 이름 : {}", input.trim());
}

