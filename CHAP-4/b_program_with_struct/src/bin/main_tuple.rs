// Using tuple
fn main () {
    let dimension = (20.0, 40.0);
    let area = calculate_area(dimension);
    println!("The area of {} * {} rectangle is {}", dimension.0, dimension.1, area);
}

fn calculate_area (dimension:(f64, f64)) -> f64 {
    dimension.0 * dimension.1
}