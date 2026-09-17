use std::io;

fn main()
{
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

    println!("Pls kind Entity! Enter the first value(c): ");
    io::stdin().read_line(&mut input1).expect("This is not a valid input Oga");
    let a:f32 = input1.trim().parse().expect("This is not a valid input Oga");

    println!("Pls kind Entity! Enter the second value(c): ");
    io::stdin().read_line(&mut input2).expect("This is not a valid input Oga");
    let b:f32 = input2.trim().parse().expect("This is not a valid input Oga");

     println!("Pls kind Entity! Enter the third value(c): ");
    io::stdin().read_line(&mut input3).expect("This is not a valid input Oga");
    let c:f32 = input3.trim().parse().expect("This not a valid input Oga");

    let d = b * b - 4.0 * a * c;

    if d > 0.0 {
        let root_1:f32 = ( -b + (b*b - 4.0*a*c).sqrt()) / (2.0 * a);
        let root_2:f32 = ( -b - (b*b - 4.0*a*c).sqrt()) / (2.0 * a);
        println!("{} and {} are the 2 distinct roots of the equation guy",root_1, root_2);
    }
    else if d == 0.0 {
        let root_3:f32 = -b / (2.0 * a);
        println!("{} is the only real root",root_3);       
    }

    else if d < 0.0 {
        println!("There aren't any roots broski! Chop rice:)")
    }
    
} 