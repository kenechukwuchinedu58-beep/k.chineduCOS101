// Project: The Shape Calculator

use std::io;

// reads one number from the user
fn read_number(prompt: &str) -> f64 {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let number: f64 = input.trim().parse().expect("Please enter a valid number");
    number
}

// Trapezium area = height / 2 * (base1 + base2)
fn trapezium_area() -> f64 {
    let height = read_number("Enter the height:");
    let base1 = read_number("Enter base1:");
    let base2 = read_number("Enter base2:");
    height / 2.0 * (base1 + base2)
}

// Rhombus area = 1/2 * diagonal1 * diagonal2
fn rhombus_area() -> f64 {
    let diagonal1 = read_number("Enter diagonal1:");
    let diagonal2 = read_number("Enter diagonal2:");
    0.5 * diagonal1 * diagonal2
}

// Parallelogram area = base * height
fn parallelogram_area() -> f64 {
    let base = read_number("Enter the base:");
    let altitude = read_number("Enter the altitude:");
    base * altitude
}

// Cube surface area = 6 * side * side
fn cube_surface_area() -> f64 {
    let side = read_number("Enter the side:");
    6.0 * side * side
}

// Cylinder volume = pi * radius * radius * height
fn cylinder_volume() -> f64 {
    let radius = read_number("Enter the radius:");
    let height = read_number("Enter the height:");
    std::f64::consts::PI * radius * radius * height
}

fn main() {
    println!("===== Shape Calculator =====");
    println!("1. Trapezium (area)");
    println!("2. Rhombus (area)");
    println!("3. Parallelogram (area)");
    println!("4. Cube (surface area)");
    println!("5. Cylinder (volume)");
    println!("Choose a shape (1-5):");

    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");
    let choice: u32 = choice.trim().parse().expect("Please enter a number");

    if choice == 1 {
        let answer = trapezium_area();
        println!("The area of the trapezium is {}", answer);
    } else if choice == 2 {
        let answer = rhombus_area();
        println!("The area of the rhombus is {}", answer);
    } else if choice == 3 {
        let answer = parallelogram_area();
        println!("The area of the parallelogram is {}", answer);
    } else if choice == 4 {
        let answer = cube_surface_area();
        println!("The surface area of the cube is {}", answer);
    } else if choice == 5 {
        let answer = cylinder_volume();
        println!("The volume of the cylinder is {}", answer);
    } else {
        println!("Invalid choice, please pick a number from 1 to 5.");
    }
}