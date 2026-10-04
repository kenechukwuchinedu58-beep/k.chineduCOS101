use std::io;

fn main() {
    // Menu (edit to match your own)
    println!("====== MENU ======");
    println!("P - Pounded Yam / Edinkaiko Soup   N3,200");
    println!("F - Fried Rice & Chicken           N3,000");
    println!("A - Amala & Ewedu Soup             N2,500");
    println!("E - Eba & Egusi Soup               N2,000");
    println!("W - White Rice & Stew              N2,500");

    // Read the food letter
    println!("Enter food letter:");
    let mut letter = String::new();
    io::stdin().read_line(&mut letter).expect("Process failed, pls try again");
    let letter = letter.trim().to_uppercase();

    // Read the quantity
    println!("Enter quantity:");
    let mut qty = String::new();
    io::stdin().read_line(&mut qty).expect("Process failed, pls try again");
    let qty: f64 = qty.trim().parse().expect("Incorrect input, quantity must be an integer");

    // Select a letter my Gee(0.0 is for invalid input)
     let mut price = 0.0;
 
    if letter == "P" {
        price = 3200.0;
    } else if letter == "F" {
        price = 3000.0;
    } else if letter == "A" {
        price = 2500.0;
    } else if letter == "E" {
        price = 2000.0;
    } else if letter == "W" {
        price = 2500.0;
    }

 
    if price == 0.0 {
        println!("Invalid choice");
    } else {
        // Total
        let mut total = price * qty;

        // Discount
        if total > 10000.0 {
            total = total - total * 0.05;
        }

        println!("Total: N{}", total);
    }
}