pub fn iterate() {
    let mut count: u8 = 0;

    // Finite Loop
    loop {
        count += 1;
        println!("Count: {}", count);

        if count == 10 {
            break;
        }
    }

    // While
    while count <= 25 {
        if count % 2 == 0 {
            println!("Eeeeven");
        }
        else if count % 3 == 0 {
            println!("treester");
        }
        else if count % 5 == 0 {
            println!("feverrr");
        }
        else {
            println!("{}", count);
        }
        // Increment
        count += 1;
    }

    // For range
    for x in 25..50 {
        if x % 2 == 0 {
            println!("Eeeeven");
        }
        else if x % 3 == 0 {
            println!("treester");
        }
        else if x % 5 == 0 {
            println!("feverrr");
        }
        else {
            println!("{}", x);
        }
        // Increment
        count += 1;
    }
}
