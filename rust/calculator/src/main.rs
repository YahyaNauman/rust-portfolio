//simple calculator

use std::io;

fn main() {
    println!("calculator");

    // input num 1
    println!("Enter first number : ");
    let mut num1_str = String::new();
    io::stdin()
        .read_line(&mut num1_str)
        .expect("Failed to read line");
    let number1: f64 = num1_str
        .trim()
        .parse()
        .expect("Please type a valid number!");

    // input opperator
    println!("enter opperator : ");
    let mut opp = String::new();
    io::stdin()
        .read_line(&mut opp)
        .expect("Failed to read line");
    let opp = opp.trim();

    //input num 2
    println!("enter second number : ");
    let mut num2_str = String::new();
    io::stdin()
        .read_line(&mut num2_str)
        .expect("Failed to read line");
    let number2: f64 = num2_str
        .trim()
        .parse()
        .expect("Please type a valid number!");

    if opp == "+" {
        println!("Answer : {}", number1 + number2)
    } else if opp == "-" {
        println!("Answer : {}", number1 - number2)
    } else if opp == "*" {
        println!("Answer : {}", number1 * number2)
    } else if opp == "**" {
        println!("Answer : {}", number1.powf(number2))
    } else if opp == "/" {
        println!("Answer : {}", number1 / number2)
    } else if opp == "%" {
        println!("Answer : {}", number1 % number2)
    } else {
        println!("Answer : {}", "invalid opperator")
    }
}
