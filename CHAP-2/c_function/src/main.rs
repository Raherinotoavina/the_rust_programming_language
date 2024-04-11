fn main() {
    println!("Hello, world!");
    another_function(10, 'h');
    state_express();
    let a = five();
    println!("Valeur obtenue dans la fonction {a}");
}

// Function with parameter
fn another_function (x:i32, label:char) {
    println!("Il est {x}{label}");
}

// Statement and expression
fn state_express () {
    let a = 45; // THIS IS A STATEMENT
    let b = {
        a + 2 // THIS IS AN EXPRESSION
    };
    println!("A : {a} and B : {b}");
}

// Function with return
fn five () -> i32 {
    // return 5;
    5 
}
