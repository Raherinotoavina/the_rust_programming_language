fn main() {
    let str = String::from("Hello World");
    let s = first_word(&str);

    let slice_str = &str[0..5];
    let slice_str1 = &str[4..];

    println!("La str slice est : {} and {}", slice_str, slice_str1);

    println!("Le premier mot est {}", s);

    let arr:[i32; 5] = [1,2,3,6,5];
    let slice_arr = &arr[0..4];

    println!("Votre arr : {:?}", slice_arr);
}

/**
 * var octets : &[u8]
*/
fn first_word(s: &String) -> &str {
    let octets = s.as_bytes();

    for (i, &element) in octets.iter().enumerate() {
        if element == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}