// Variables hold primitive data or references to data
// Variables are immutable by default
// Rust is a block-scoped

pub fn run() {
    // Variables are immutable by defualt
    let file = "variables";
    println!("Hello from the {file} file.");

    // Variables can be mutated with the keyword "mut"
    let name = "Jim";
    let mut age = 35;

    println!("My name is {name} and I am {}", age);

    age = 36;
    println!("In one week I will be {}", age);

    // Define Constants
    // Constants need to explicitly define a type
    // Constants typically have uppercase naming convention
    const ID: i32 = 123;
    println!("ID: {}", ID);

    // Assigning Mutliple Variables
    let (my_name, my_age) = ("Jim", 35);
    println!("{} is {}.", my_name, my_age)
}
