pub fn run() {
    // print to console
    println!("Hello from print.rs");

    // basic formating
    println!("{} is from {}", "Arif", "Jogja");

    // positional arguments
    println!(
        "{0} is from {1} and {0} likes to {2}",
        "Arif", "Jogja", "code"
    );

    // named arguments
    println!(
        "{name} like to play {activity}",
        name = "Arif Budiman Arrosyid",
        activity = "Football"
    );

    // placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

    // placeholder for debug trait
    println!("{:?}", (12, true, "hello"));

    // basic math
    println!("10 + 10 = {}", 10 + 10);
}
