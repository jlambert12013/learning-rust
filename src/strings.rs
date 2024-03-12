// Primitive str - immutable fixed-length string somewhere in memory
// String - growable, heap-allocated data structure - Use when you need to modify or own string

pub fn run() {
    let mut greeting = String::from("Hello, ");
    println!("{}", greeting);

    // String length
    println!("Length: {}", greeting.len());

    // String push method (must be mutable)
    // push applies to char
    greeting.push('W');

    // String push for String
    greeting.push_str("orld");
    println!("{}", greeting);

    // Capacity in Bytes
    println!("Capactiy: {}", greeting.capacity());

    // Check for empty
    println!("Empty: {}", greeting.is_empty());

    // Check for substring
    println!(
        "Does string contain word 'world' : {}",
        greeting.contains("World")
    );

    // Replace method
    println!("Replace: {}", greeting.replace("World", "Everyone!"));

    let lorem = "Lorem Ipsum is simply dummy text of the printing and typesetting industry. Lorem Ipsum has been the industry's standard dummy text ever since the 1500s, when an unknown printer took a galley of type and scrambled it to make a type specimen book. It has survived not only five centuries, but also the leap into electronic typesetting, remaining essentially unchanged. It was popularised in the 1960s with the release of Letraset sheets containing Lorem Ipsum passages, and more recently with desktop publishing software like Aldus PageMaker including versions of Lorem Ipsum.";

    // Looping string
    for word in lorem.split_whitespace() {
        println!("{}", word);
    }

    // Create string with capacity
    let mut placeholder = String::with_capacity(10);

    placeholder.push('a');
    placeholder.push('b');
    println!("{}", placeholder);

    // Assertion testing (only shows if failed)
    assert_eq!(2, placeholder.len());
    // assert_eq!(11, placeholder.capacity());  // this one fails
}
