use std::io;

fn main() {
    println!("How many people are you interviewing?");

    let mut count = String::new();
    io::stdin().read_line(&mut count).expect("Failed to read line");
    let count: u32 = count.trim().parse().expect("Please enter a number");

    // tuple that holds the best person so far: (name, years)
    let mut best = (String::new(), 0);

    for i in 0..count {
        println!("Enter name of person {}:", i + 1);
        let mut name = String::new();
        io::stdin().read_line(&mut name).expect("Failed to read line");
        let name = name.trim().to_string();

        println!("Enter years of experience:");
        let mut years = String::new();
        io::stdin().read_line(&mut years).expect("Failed to read line");
        let years: u32 = years.trim().parse().expect("Please enter a number");

        // if this person has more years, they are the new best
        if years > best.1 {
            best = (name, years);
        }
    }

    println!("Highest experience: {} with {} years", best.0, best.1);
}