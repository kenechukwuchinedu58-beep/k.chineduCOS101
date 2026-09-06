fn main() {
    let prices =[450000.0, 1500000.0, 750000.0, 2850000.0, 250000.0];
    let quantities = [2.0, 1.0, 3.0, 3.0, 1.0];

    let mut total = 0.0;

    for i in 0..prices.len() {
        total += prices[i] * quantities[i];
    }

    let average = total / prices.len() as f64;

    println!("Total sales = {}", total);
    println!("Average price = {}", average);
}