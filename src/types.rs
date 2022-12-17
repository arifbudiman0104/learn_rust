// Integers (number with no decimal value)
// sign is for positive value and unsign is for negative value

// Length	Signed	Unsigned
// 8-bit	i8              u8
// 16-bit	i16	            u16
// 32-bit	i32(default)	u32
// 64-bit	i64	            u64
// 128-bit	i128            u128
// arch	    isize	        usize
// for more you can visit https://doc.rust-lang.org/book/ch03-02-data-types.html

// Floats (number with decimal value)
// f32, f64(default)

// String
// str = immutable
// String = mutable

// Boolean
// bool

// Characters
// char
// must use single quote ''

// Tuples
// group together values of different types with max 12 element

// Arrays
// fixed list where elements are the same data types

// Vector
// resizeable arrays

// Conditionals
// used to check the condition of something

use std::mem;

pub fn run() {
    // default integer is i32
    let x = 1;
    println!("Integer i32 {}", x);

    // default float is f64
    let y = 2.5;
    println!("Float f64 {}", y);

    // add explicit type
    let z: i64 = 234234;
    println!("Explicit integer i64 {}", z);

    // find max size
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i32: {}", std::i64::MAX);

    // boolean
    let is_active = true;

    // get boolean from expression
    let is_greater = 10 > 20;

    // char
    let a1 = 'a';
    let face = '\u{1f600}';

    println!("is it active? {}", is_active);
    println!("{:?}", (x, y, z, is_active, is_greater, a1, face));

    // string mutable
    let mut hello = String::from("Hello ");
    hello.push('W');
    hello.push_str("orld!");

    // string immutable
    let text = "this is a text";

    println!("{}", hello);
    println!("{}", text);

    // length
    println!("Length : {}", hello.len());

    // capacity in bytes
    println!("Capacity : {}", hello.capacity());

    // check is empty
    println!("Is Empty : {}", hello.is_empty());

    // contain
    println!("Contain 'World' : {}", hello.contains("World"));

    // replace
    println!("Replace : {}", hello.replace("World", "There"));

    // loop through string by whitespace
    for word in hello.split_ascii_whitespace() {
        println!("this is a lop split {}", word);
    }

    // string with byte capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    // assertion testing
    assert_eq!(2, s.len());
    // assert_eq!(11, s.len());

    println!("{}", s);

    // Tuples
    let person: (&str, &str, i32) = ("Arif", "Jogja", 21);
    println!("{} is from {} and is {}", person.0, person.1, person.2);

    // Arrays
    let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];

    // reasign value
    numbers[2] = 20;
    println!("arrays value {:?}", numbers);

    // get singel value
    println!("get singel value object number 3 is {:?}", numbers[2]);

    // get length
    println!("Array of number has {:?} length", numbers.len());

    // arrays are stack allocated
    println!("Array occupies {} bytes", std::mem::size_of_val(&numbers));
    println!("Array occupies {} bytes", mem::size_of_val(&numbers));

    // slice
    let slice: &[i32] = &numbers[1..4];
    println!("Slice: {:?}", slice);

    // Vector
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5, 8, 5656, 56, 3];

    println!("vector value {:?}", numbers);
    // get singel value
    println!("get singel value object number 3 is {:?}", numbers[2]);

    // get length
    println!("Vector of number has {:?} length", numbers.len());

    // vector are stack allocated
    println!("Vector occupies {} bytes", std::mem::size_of_val(&numbers));
    println!("Vector occupies {} bytes", mem::size_of_val(&numbers));

    // slice
    let slice: &[i32] = &numbers[1..4];
    println!("Slice: {:?}", slice);

    // add to vector
    numbers.push(300);
    numbers.push(230);
    println!("vector value {:?}", numbers);

    // loop vector values
    for x in numbers.iter() {
        println!("Number : {}", x)
    }

    // loop and mutate value
    for x in numbers.iter_mut() {
        *x *= 2;
    }
    println!("vector value {:?}", numbers);

    // Conditionals
    let age = 21;
    if age >= 18 {
        println!("Bartender : What would you like to drink?");
    } else {
        println!("Bartender : Sorry you have to leave");
    }

    let check_id = false;
    if age >= 18 && check_id {
        println!("Bartender : What would you like to drink?");
    } else if age < 18 && check_id {
        println!("Bartender : Sorry you have to leave");
    } else {
        println!("Bartender : I need to see your ID");
    }
    
    let knows_person_of_age = true;
    if age >= 18 && check_id ||knows_person_of_age{
        println!("Bartender : What would you like to drink?");
    } else if age < 18 && check_id {
        println!("Bartender : Sorry you have to leave");
    } else {
        println!("Bartender : I need to see your ID");
    }

    let is_of_age = if age >= 18 {true} else {false};
    println!("Is of Age: {}", is_of_age);

    // Loops
    
}
