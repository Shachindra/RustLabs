pub fn print_to_console() {
    // Print to Console
    println!("Hola from the printToConsole Function");
    
    // Print to Console
    println!("Number: {}", 1);
    
    // Basic Formatting
    println!("{} is {}", "Climate Change", "Real");

    // Positional Arguments
    println!("{0} and {1} are ways to combat {2} and of course {0} is fun", "Planting Trees", "Effective Recycling", "Climate Change");

    // Named Arguments
    println!("{name} contributes to {objective}", name = "Michael", objective = "ClimateAction");

    // Placeholder traits
    println!("Binary: {:b} and Hex: {:x} and Octal: {:o}", 10, 10, 10);

    // Placeholder debug traits
    println!("{:?}", (16,true, "Smart"));

    // Simple Math
    println!("10 + 10 = {}", 10 + 10);
}