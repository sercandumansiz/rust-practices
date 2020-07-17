use std::io;

fn main() {

    println!("please input the n'th fibanocci number");

    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("failed to read line");

    let number = input.trim().parse().unwrap();

    let fibonacci = calculate_fibonacci_number(number);

    println!("{}", fibonacci);
}

// 1, 1, 3, 5, 8, 13, 21, 34
fn calculate_fibonacci_number(index: u32) -> u32 {
    match index {
        0 => 1,
        1 => 1,
        _ => calculate_fibonacci_number(index - 1) + calculate_fibonacci_number(index - 2)
    }
}
