pub fn greeting(greet: &str, name: &str) {
    println!("{} {}, Namaskara!", greet, name);
}

pub fn calculate() {
    // Bind function values to variables
    let n1: i32 = 8;
    let n2: i32 = 19;
    let n3: i32 = 19;
    let get_mul = mul(8, 19);
    println!("Multiplication of {} & {} yields {}", n1, n2, get_mul);

    // Closure
    let sub_nums = |num1: i32, num2: i32| num1 - num2 + n3;
    println!("Subtract:  {}", sub_nums(59, 98));
}

fn mul(num1: i32, num2: i32) -> i32 {
    num1 * num2
}