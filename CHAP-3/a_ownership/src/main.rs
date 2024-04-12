fn main() {
    let s = String::from("Hello"); 
    let b = "String";// String literal because the value of the string is hardcoded
    let a = s.clone();
    println!("{}, {}", a, s);
}
