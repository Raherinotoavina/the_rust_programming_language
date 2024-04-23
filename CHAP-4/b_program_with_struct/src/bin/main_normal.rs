// Calculate area of a rectangle
fn main() {
    let width = 32.0;
    let height = 20.0;
    let area = calculate_area(width, height);

    println!("The area of {} * {} rectangle is {}", width, height, area);
}

fn calculate_area (w:f32, h:f32) -> f32 {
    w * h
}