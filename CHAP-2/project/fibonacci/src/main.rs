use std::io;
fn main() {
    let mut nth = String::new();
    println!("Entrer le nombre nth fibonnaci que vous voulez trouver :");
    io::stdin().read_line(&mut nth).expect("Failed to read line");
    let nth:i32 = nth.trim().parse().expect("Impossible to convert");

    let res:i128 = if nth == 0 {0} else if nth == 1 {1} else {fibonacci(nth)};
    println!("Le fibonnacci number est : {}", res);
}

fn fibonacci (n:i32) -> i128 {
    let (mut first, mut second, mut r) = (0,1,0);
    for _ in 0..n - 1 {
        r = second + first;
        (first, second) = (second, r);
    }
    r
}