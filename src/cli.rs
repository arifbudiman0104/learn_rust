// CLI

use std::env;

pub fn run() {
    let args: Vec<String> = env::args().collect();
    let command = args[1].clone();

    if command == "hello" {
        println!("Hello World");
    } else if command == "bye" {
        println!("Bye World");
    } else {
        println!("Command not found");
    }

    let name = "Arif";
    let status = "100%";

    if command == "hi" {
        println!("Hi {}, how are you?", name);
    } else if command == "status" {
        println!("Status is {}", status);
    } else {
        println!("Command not found");
    }

    println!("Args : {:?}", args);
}

// go to terminal and type: cargo run [command]
