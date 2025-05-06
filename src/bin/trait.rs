use std::fmt::Display;

struct Point {
    x: i32,
    y: i32,
}
    
impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

fn main() {
    let p = Point { x: 10, y: 20 };
    println!("Point: {}", p);
}
