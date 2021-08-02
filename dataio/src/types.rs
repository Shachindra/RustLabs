use std::mem;

pub fn types() {

    // i32
    let a = 1;

    // f64
    let b = 1.93;

    //Add explicit type
    let c: i64 = 121456987;

    // find max size
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);
    
    // Boolean
    let statement: bool = false;

    // Get boolean from expression
    let is_smaller = 1.0 < 5.0;

    // Char
    let d = 't';
    let smiley = '\u{1F600}';

    println!("{:?}", (a, b, c, statement, is_smaller, d, smiley));

    // String
    let primitive_str = "Hello, World";
    let mut new_str = String::from("Hello, New World");

    // Get length
    println!("Length: {}", primitive_str.len());
    println!("Length: {}", new_str.len());

    new_str.push('!');
    new_str.push(' ');
    new_str.push_str("Welcome!!");

    println!("Length: {}", new_str);

    // Capacity
    println!("Length: {}", new_str.capacity());

    // Check if empty
    println!("Length: {}", new_str.is_empty());

    // Contains
    println!("Length: {}", new_str.contains("World"));

    // Replace
    println!("Length: {}", new_str.replace("Welcome", "Goodbye"));

    // Loop through the string by whitespace
    for word in new_str.split_whitespace() {
        println!("{}", word);
    }

    // Create string with capacity
    let mut capped_str = String::with_capacity(10);
    capped_str.push('s');
    capped_str.push_str("tyle");

    println!("Capped String: {}", capped_str);

    // Assertion
    assert_eq!(5, capped_str.len());
    assert_eq!(10, capped_str.capacity());
}

pub fn tuples() {
    // User struct
    let user: (&str, &str, i8) = ("John", "Apple", 10);

    println!("{} eats {} {}(s) everyday", user.0, user.2, user.1)
}

pub fn arrays() {
    let mut ranks: [i32; 4] = [100, 205, 309, 705];

    // Print array
    println!("Array Ranks: {:?}", ranks);

    // Get single value
    println!("{}", ranks[1]);

    ranks[3] = 69;
    println!("{}", ranks[3]);

    println!("Array Length: {}", ranks.len());

    // Stack Allocation
    println!("Array occupies {} bytes", mem::size_of_val(&ranks));

    //Get Slice
    let sliced_rank: &[i32] = &ranks[1..3];
    println!("Sliced: {:?}", sliced_rank);
}

pub fn vectors() {
    // Resizable Arrays
    let mut ranks: Vec<i32> = vec![100, 205, 309, 705];
    // Print array
    println!("Vector Ranks: {:?}", ranks);

    // Get single value
    println!("{}", ranks[1]);

    ranks[3] = 69;
    println!("{}", ranks[3]);

    // Add into vectors
    ranks.push(567);
    ranks.push(435);

    println!("Vector Length: {}", ranks.len());

    // Remove last value
    ranks.pop();

    // Stack Allocation
    println!("Vector occupies {} bytes", mem::size_of_val(&ranks));

    //Get Slice
    let sliced_rank: &[i32] = &ranks[1..3];
    println!("Sliced: {:?}", sliced_rank);

    // Loop through the vector
    for rank in ranks.iter() {
        println!("Rank: {}", rank)
    }

    // Loop and Change Values
    for rank in ranks.iter_mut() {
        *rank *= 2;
    }

    println!("Ranks: {:?}", ranks);
}