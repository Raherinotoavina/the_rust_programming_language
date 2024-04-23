/** dbg!() macro take ownership of the current value
 * Write in the stderr => standard error console stream
 * 
 * println!() stdout => standard output console stream
 */

// Using struct
#[derive(Debug)]
struct Rectangle {
    w : f64,
    h : f64
}
fn main () {
    let rectangle = Rectangle {
        w : 20.4,
        h : 20.3
    };
    let area = calculate_area(&rectangle);
    println!("The area of {}x{} rectangle is {}", rectangle.w, rectangle.h, area);
    print_struct(&rectangle);
    dbg!(&rectangle);
}

fn calculate_area (rec:&Rectangle) -> f64 {
    rec.w * rec.h
}

fn print_struct (rec:&Rectangle) {
    println!("The rectangle is {:#?}", rec)
}