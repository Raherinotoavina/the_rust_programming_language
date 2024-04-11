use std::io;

fn main() {
    let x:i32 = "42".parse().expect("This is not a number");
    println!("Our number : {x}");

    // Tuples
    println!("TUPLES");
    let tup:(i32, f64, char) = (20, 2.5, 'a');
    let (x,y,z) = tup;
    println!("Our tuples : {:?}",tup);
    println!("x : {:?}", x);
    println!("y : {:?}",y);
    println!("z : {:?}",z);

    // Array
    let arr:[i32; 5] = [32,52, 2, 5,8];
    println!("Notre array : {:?}", arr);
    println!("Enter an array index : ");
    let mut index = String::new();
    io::stdin().read_line(&mut index).expect("Failed to readline");
    let index:usize = index.trim().parse().expect("failed to convert");
    println!("L'element a l'index {:?} est {:?}", index, arr[index]);
}
