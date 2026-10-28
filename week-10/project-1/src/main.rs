// So um here is where I believe I declare my structure
struct Laptops {
    brand:String, 
    qty:i32,
    price:f64,
}

//Ok so here is my logic for calculating the cost of three laptops from each brand
impl Laptops {
    fn cost(&self)->f64 {
        self.price * 3.00
    }
}

fn main() {
    //Here is where I initialize the structure
    let lap1 = Laptops {
        brand:String::from("HP"),
        qty:10,
        price:650_000.00
    };
    let lap2 = Laptops {
        brand:String::from("IBM"),
        qty:6,
        price:755_000.00
    };
    let lap3 = Laptops {
        brand:String::from("Toshiba"),
        qty:10,
        price:550_000.00
    };
    let lap4 = Laptops {
        brand:String::from("Dell"),
        qty:4,
        price:850_000.00
    };

// Here is the total price
    let total = lap1.cost() + lap2.cost() + lap3.cost() + lap4.cost();
    println!("Your total price will be: N{}. Thank you for your purchase pls come again :)", total);
}