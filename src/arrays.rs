use std::mem;

// Arrays - Fixed list where elements are the same data types

pub fn run() {
    let mut number_array: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", number_array);

    // Getting a single value
    println!("Single Value: {}", number_array[0]);
    println!("Single Value: {}", number_array[2]);
    println!("Single Value: {}", number_array[4]);

    // Reassign Value
    number_array[4] = 23;
    println!("{}", number_array[4]);

    // Get array length
    println!("Length of array: {}", number_array.len());

    // Get amount of memory
    // Arrays are stack allocated
    println!("Array occupies {} bytes.", mem::size_of_val(&number_array));

    // Get Slice
    let slice: &[i32] = &number_array[0..2];
    println!("Slice: {:?}", slice)
}
