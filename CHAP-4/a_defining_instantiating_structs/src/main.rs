// Normal struct
#[derive(Clone)]
struct User {
    name : String,
    first_name : String,
    age : u32,
}

// Tuple Struct
struct StructTuple (u64, u64, u64);

// Unit-like Struct
struct AlwaysEqual;

fn main() {
    let mut user1 = User {
        name : String::from("RAHERINOTOAVINA"),
        first_name : String::from("Safidy Mariel"),
        age : 22
    };

    println!("The name of the user is : {}", user1.name);
    println!("The first name of the user is : {}", user1.first_name);
    println!("The age of the user is : {}", user1.age);
    
    user1.name = String::from("TOAVINA");
    println!("The new name of our user is : {}", user1.name);

    // Create user by using the build_user function
    let user2 = build_user(String::from("RAKOTO"), String::from("Jean"), 25);
    println!("User 2 name is : {}", user2.name);
    println!("User 2 first name is : {}", user2.first_name);
    println!("User 2 age is : {}", user2.age);

    // Creating instance of struct with an existing struct
    let user3 = User {..user1.clone()};
    println!("User 3 name is : {}", user3.name);
    println!("User 1 name is : {}", user1.name);

    // Using struct like a tuple
    let color = StructTuple (0, 0, 0);
    println!("Color 1 : {}", color.0);

}

fn build_user (name:String, first_name:String, age:u32) -> User {
    User {
        name,
        first_name,
        age
    }
}