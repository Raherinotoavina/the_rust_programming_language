fn main() {
    let mut x = 5;
    println!("Le nombre est : {x}");
    x = 6;
    println!("Le nouveau nombre est : {x}");
    const MY_CONSTANTE:i32 = 20;
    println!("Le constante est : {MY_CONSTANTE}");
    let x = x + 5;
    {
        let x = x + 6;
        println!("Shadow variable in this scoop : {x}");
    }
    println!("Shadow variable in the main scoop : {x}");
}
