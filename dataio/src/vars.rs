pub fn operation() {
    let name = "John";
    let mut duration = 5;
    let passion = "Climate Action";
    println!("Yo! My name is {} & my paasion for the last {} years has been {}", name, duration, passion);
    
    duration = 50;
    println!("Yo! My name is {} & my paasion for the last {} years has been {}", name, duration, passion);

    // Defining constants
    const ID : i32 = 001;
    println!("My ID is {}", ID);

    // Assign Multiple Variables
    let (user_name, user_age) = ("Lil John", 29);
    println!("{} is {} years old", user_name, user_age);
}