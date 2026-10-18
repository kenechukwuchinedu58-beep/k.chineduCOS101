use std::io;

fn main() {
    // each row is a tuple: (level, office admin, academic, lawyer, teacher)
    let table = [
        ("APS 1-2", "Intern", "-", "Paralegal", "Placement"),
        ("APS 3-5", "Administrator", "Research Assistant", "Junior Associate", "Classroom Teacher"),
        ("APS 5-8", "Senior Administrator", "PhD Candidate", "Associate", "Senior Teacher"),
        ("EL1 8-10", "Office Manager", "Post-Doc Researcher", "Senior Associate 1-2", "Leading Teacher"),
        ("EL2 10-13", "Director", "Senior Lecturer", "Senior Associate 3-4", "Deputy Principal"),
        ("SES", "CEO", "Dean", "Partner", "Principal"),
    ];

    // slice of rows 0 to 6 (6 is not included)
    let rows = &table[0..6];

    println!("=== APS Level Checker ===");
    println!("1. Office Admin");
    println!("2. Academic");
    println!("3. Lawyer");
    println!("4. Teacher");
    println!("Choose a profession (1-4):");

    // get the profession choice from the user
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read line");
    let choice: u32 = choice.trim().parse().expect("Please enter a number");

    println!("Enter the position (e.g. Associate):");

    // get the position from the user
    let mut job = String::new();
    io::stdin().read_line(&mut job).expect("Failed to read line");
    let job = job.trim();

    // start with a default answer
    let mut level = "Position not found. Please check your spelling.";

    // go through each row (tuple) in the slice
    for i in 0..rows.len() {
        let row = rows[i];

        // row.1 is office admin, row.2 is academic, row.3 is lawyer, row.4 is teacher
        let mut position = row.1;
        if choice == 2 {
            position = row.2;
        } else if choice == 3 {
            position = row.3;
        } else if choice == 4 {
            position = row.4;
        }

        // if the position matches, replace the default answer with the level (row.0)
        if position.to_lowercase() == job.to_lowercase() {
            level = row.0;
        }
    }

    println!("Level: {}", level);
}