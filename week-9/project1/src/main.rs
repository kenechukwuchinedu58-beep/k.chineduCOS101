use std::fs::File;
use std::io::Write;

fn main() {
    // This is the table for the Nigerian Breweries dude
    let table = "Lager,Stout,Non-Alcholic\n33 Export,Legend,Maltina\nDesperados,Turbo King,Amstel Malta\nGoldberg,Williams,Malta Gold\nGulder,,Fayrouz\nHeineken,,\nStar,,\n";
    // This creates the file bro
    let mut file = std::fs::File::create("table.csv").expect("Yo this shi buggin, my bad brochacho");
    // I beleive this writes the table into the file
    file.write_all(table.as_bytes()).expect("Abeg no vex oh but this shi ain't working");
    println!("\nTable written to file.");
}
// BTW I didn't use AI, is this impressive enough :)