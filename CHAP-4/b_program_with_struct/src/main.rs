// Using struct
struct Rectangle {
    w : f64,
    h : f64
}
fn main () {
    let rectangle = Rectangle {
        w : 20.4,
        h : 20.3
    };
    let area = rectangle.w * rectangle.h;
    println!("The area of {} * {} rectangle is {}", rectangle.w, rectangle.h, area);
}