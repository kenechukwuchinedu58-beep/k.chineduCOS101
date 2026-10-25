use std::io::Write;

fn main() {
    let table1 = ["Commissioner","Aigbogun Alamba Daudu","Murtala Afeez Bendu","Okorocha Calistus Ogbona","Adewale Jimoh Akanbi","Osazuwa Faith Etieye",];
    let table2 = ["Ministries","Internal Affairs","Justice","Defense","Power & Steel","Petroleum",];
    let table3 = ["Zones","South West","North East","South South","South West","South East"];

     let mut file = std::fs::File::create("table.csv").expect("Yo this shi buggin, my bad brochacho");
    
    for i in 0..table1.len() {
        println!("{} | {} | {}", table1[i], table2[i], table3[i]);
        file.write_all(table1[i].as_bytes()).expect("Srry bruv cant help u");
        file.write_all(",".as_bytes()).expect("Srry bruv cant help u");
        file.write_all(table2[i].as_bytes()).expect("Srry bruv cant help u");
        file.write_all(",".as_bytes()).expect("Srry bruv cant help u");
        file.write_all(table3[i].as_bytes()).expect("Srry bruv cant help u");
        file.write_all(",".as_bytes()).expect("Srry bruv cant help u");
        file.write_all("\n".as_bytes()).expect("Srry bruv cant help u");
        }

    println!("\nTable written to file.");

    println!("files merged successfully :)");
}
// BTW I didn't use AI, is this impressive enough :)