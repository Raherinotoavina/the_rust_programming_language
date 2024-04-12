fn main() {
    let mut s = String::from("Hello world!");
    let len = calculate_len(&s);
    println!("La longueur du string {:?} est {}", s, len);

    change(&mut s);
    println!("Le string modifie est {:?}", s);
}

fn calculate_len (s: &String) -> usize {
    s.trim().len()
}

fn change (s: &mut String) {
    s.push_str(" ,It's a very good news.");
}