//Program to for a Incentive Calculator
use std::io;

fn main()
{
    println!("\nEmployees Annual Incentives Form:");

    // Input your name here
    println!("Enter your fullname here pls:");
    let mut name = String::new();
        io::stdin()
        .read_line(&mut name)
        .expect("Incorrect input");
    println!("Your name is: {}", name);

    // Input age here
    println!("\nEnter your age here pls:");
    let mut age = String::new();
        io::stdin()
        .read_line(&mut age)
        .expect("Failed to read input");
    let age:i32 = age
    .trim()
    .parse()
    .expect("Input incorrect, pls input an integer");

    println!("Your age is: {}", age);

    // Input whether or not you're experienced here
    let mut answer = String::new();

    println!("Are you experienced? (yes/no)");
    io::stdin()
    .read_line(&mut answer)
    .unwrap();

    // Calculate incentive
    if answer.trim() == "no" {
        println!("Your annual incentive is $100_000.00")
    }
    else if age < 28 && answer.trim() == "yes"
    {
        println!("Your annual incentive is $1_300_000.00")
    } 
    else if age >= 30 && age <= 39  && answer.trim() == "yes"
    {
        println!("Your annual incentive is $1_480_000.00")
    }
    else if age >= 40 && answer.trim() == "yes"
    {
        println!("Your annual incentive is $1_560_000")
    }
     else 
    {
        println!("No incentive is provided.");
    }
}