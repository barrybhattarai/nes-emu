pub mod point;

use point::Point;
fn main() {
    let p = Point { x: 3.0, y: 4.0 };
    let p2 = Point{ x: 2.0, y: 2.0 };
    let result = p - p2;
    println!("{:?}", result);
}



