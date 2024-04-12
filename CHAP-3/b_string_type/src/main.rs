fn main() {
    let mut s = String::from("Hello");
    s.push_str("World");
    println!("{}",s);

    let mut a = 15;
    let b = a;
    a = 20;
    println!("a : {}, b : {}", a,b);

    // Clone a string with clone() methodes
    let b = String::from("Hello");
    let c = b.clone();

    println!("Voici le clone : {}", c);

    // Ownership function
    let a = String::from("Hello");
    ownership_func(a.clone());
    println!("{}",a);
}

fn ownership_func (a:String) {
    println!("a est ici : {}", a)
}