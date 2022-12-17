// variables hold primitive data or references to data
// variables are immutable by default
// rust is a block-scoped languange

pub fn run() {
    // immutable
    let name = "Arif Budiman Arrosyid";
    println!("My name is {}", name);

    // mutable
    let name = "Arif Budiman";
    let mut age = 20;
    println!("My name is {} and I am {}", name, age);
    age = 21;
    println!("My name is {} and I am {}", name, age);

    // define constant
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // assign multiple vars
    let (my_name, my_age) = ("Arif Budiman Arrosyid", 21);
    println!("{} is {}", my_name, my_age);
}
