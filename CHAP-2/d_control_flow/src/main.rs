fn main() {
    for_loop()
}

// Control flow
fn _if_control (n:i32) {

    let number = if n == 5 {10} else {5};

    println!("Le nombre est {number}");

    if n > 5 {
        println!("Le nombre est plus grand que 5");
    } else if n < 5 {
        println!("Le nombre est plus petit que 5");
    } else {
        println!("Le nombre est egale a 5");
    }
}

// Repetition with "LOOP"
// 'counting_up => "c'est une etiquette pour stopper le boucle" ici le premier boucle quand on a des boucle imbriquer
fn _loop_repetition () {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 0;
        loop {
            if remaining == 2 {break}
            if count == 3 {break 'counting_up}
            remaining += 1
        }
        count += 1;
    }
    println!("End count = {count}");
}

// Repetition with "WHILE"
// On utilise cette boucle avec un condition
fn _while_repetition (n:i32) {
    let mut i = 0;
    while i < n {
        println!("While true");
        i += 1
    }
}

// Parcourir une liste avec for
// Pour eviter l'index qui s'echappe
fn for_loop () {
    let a:[i32;4] = [10,5,3,6];
    for element in a {
        println!("Le nombre est {element}")
    }
    
    for i in 0..4 {
        println!("{i}")
    }
}