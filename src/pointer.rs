// Pointer
// a reference to a memory location

pub fn run() {
    // Primitive Array
    let arr1 = [1, 2, 3];
    let arr2 = arr1;

    // Non-Primitive Array
    let vec1 = vec![1, 2, 3];
    let vec2 = &vec1;

    println!("Primitive Array Values: {:?}", (arr1, arr2));
    println!("Non Primitive Array Values: {:?}", (&vec1, vec2));
}
