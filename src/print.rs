pub fn run() {
    // Print to Console
    println!("Hello from the print file!");

    // String Interpolation
    println!("AGE: {}", 35);

    // Strin Interpolation with Variables
    let name = "Jim";
    println!("NAME: {}", name);
    println!("My name is {} and I am {} years old.", name, 35);

    // Positional Arguments
    // println!(
    //     "My name is {0} from {1} and {0} likes to {2}.",
    //     "Jim", "Alabama", "play guitar"
    // );

    // Named Arguments
    // println!(
    //     "Hello, my name is {name} and I am {age} years old.",
    //     name = "Jim",
    //     age = 35
    // );

    // Placeholder Traits
    println!("Binary: {:b}, Hex: {:x}, Octal: {:o}", 10, 10, 10);

    // Placeholder for Debug Trait
    println!("{:?}", (12, true, "Hello, World!"));

    // Basic Math
    println!("10 + 10 = {}", 10 + 10);
}
