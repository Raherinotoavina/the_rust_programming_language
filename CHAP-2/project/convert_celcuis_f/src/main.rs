use std::io;

fn main() {
    let mut s = String::new();
    let mut temp = String::new();
    
    loop {
        println!("Choisir le type de temperature que vous voulez convertir A:celcius et B:fahrenheit");
        io::stdin().read_line(&mut s).expect("Failed to read line");

        s = s.trim().to_uppercase().to_string();

        if s != "A" && s != "B" {
            println!("################### Recommencer votre saisi ######################");
            continue
        }
    
        println!("Saisir le temperature :");
        io::stdin().read_line(&mut temp).expect("Failed to read line");
        
        if s == "A" || s == "B" {break}
    }

    let temp_int:i32 = temp.trim().parse().expect("Impossible to convert.");


    if s == "A" {
        let res = (temp_int as f64) * 9.0/5.0 + 32.0;
        println!("Celcius in fahrenheit : {} = {}", temp_int, res);
    } else if s == "B" {
        let res =  (temp_int as f64 - 32.0) * 5.0/9.0;
        println!("Fahrenheit in celcius : {} = {}", temp_int, res);
    }
}
