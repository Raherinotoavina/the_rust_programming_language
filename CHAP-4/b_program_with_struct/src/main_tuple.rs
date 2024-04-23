// Using tuple
fn main () {
    let (w, h) = (20.0, 40.0);
    let area = calculate_area(w,h);
    println!("The area of {} * {} rectangle is {}", w, h, area);
}

fn calculate_area (w:f64, h:f64) -> f64 {
    w*h
}