/*
Primitive Types--
Integers: u8, i8, u16, i16, u32, i32, u64, i64, 128, i128 (number of bits they take in memory)
Floats: f32, f64
Boolean (bool)
Characters (char)
Tuples
Arrays
*/

/* Rust is a statically typed language, which means that it must know the types of all variables at
compile time, however, the compiler can usually infer what type we want to use based on the value
and how we use */

pub fn run() {
    // Default is "i32"
    let x = 1;

    // Default is "f64"
    let y = 2.5;

    // Add explicit type
    let z: i64 = 8675309;

    // Find max size by bringing in the standard library
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);

    // Boolean
    let is_active = true;
    println!("{:?}", (x, y, z, is_active));

    // Boolean from expression
    let is_greater = 10 > 5;
    println!("{}", is_greater);

    // Character (char) - use singles qoutes
    let letter = 'a';
    println!("The alphabet start with {}", letter);

    // Unicode is a char as well
    let face = '\u{1F600}';

    println!("Have a great day {}!", face);
}
