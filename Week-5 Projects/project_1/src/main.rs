//Rust program to find the roots of a quadratic equation

use std::io;

fn main() {
    println!("a is the coefficient of x squared");
    println!("b is the coefficient of x");
    println!("c is a stand alone value");

    //input values
    println!("\n Input value for a.");
    let mut a = String::new();
    io::stdin().read_line(&mut a).expect("Not a valid string");
    let a:i32 = a.trim().parse().expect("Not a valid number");

    println!("\n Input value for b.");
    let mut b = String::new();
    io::stdin().read_line(&mut b).expect("Not a valid string");
    let b:i32 = b.trim().parse().expect("Not a valid number");

    println!("\n Input value for c.");
    let mut c = String::new();
    io::stdin().read_line(&mut c).expect("Not a valid string");
    let c:i32 = c.trim().parse().expect("Not a valid number");

    let discriminant = (b*b) - (4 * a * c);
    let root1 = (-b + discriminant) / (2 * a);
    let root2 = (-b - discriminant) / (2 * a);

    if discriminant > 0 {
        println!("Discriminant is positive, hence there are two roots");
        println!("Roots are {} and {}", root1, root2);
    }
    else {
        println!("Discriminant is zero, hence there is exactly one root");
        println!("Root is {}",root1);
    }
     loop {
        if discriminant < 0 {
        println!("Discriminant is negative, hence there are no real roots");
        break;
    }
    }
}
