//Rust program to display menu

use std::io;

fn main() {

    //P
    let pricep:f64 = 3200.0;

    //F
    let pricef:f64 = 3000.0;

    //A
    let pricea:f64 = 2500.0;

    //E
    let pricee:f64 = 2000.0;

    //W
    let pricew:f64 = 2500.0; 

    println!(" (S/N)         Menu                               Price");
    println!("P(1) = Poundo Yam/Edinkaiko Soup                 N3,200");
    println!("F(2) = Fried Rice & Chicken                      N3,000");
    println!("A(3) = Amala & Ewedu Soup                        N2,500");
    println!("E(4) = Eba & Egusi Soup                          N2,000");
    println!("W(5) = White Rice & Stew                         N2,500");

    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("\nInput S/N of food wanted ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let foodtype:f64 = input1.trim().parse().expect("Not a valid S/N"); 

    println!("What quantity do you want?");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let quantity:f64 = input2.trim().parse().expect("Not a valid number");


    if foodtype == 1.0 {
        let total = quantity * pricep;
        println!("Total is {}",total);

        if total > 10000.0 {
        let discount:f64 = total - (0.05 * total);
        println!("You get a 5% discount!! Your new total is {}",discount);
       }
       else {
        println!("Your total is {}",total);
       }
    }
    else if foodtype == 2.0 {
        let total = quantity * pricef;
        println!("Total is {}",total);

        if total > 10000.0 {
        let discount:f64 = total - (0.05 * total);
        println!("You get a 5% discount!! Your new total is {}",discount);
        }
        else {
        println!("Your total is {}",total);
        }
    }
    else if foodtype == 3.0 {
        let total = quantity * pricea;
        println!("Total is {}",total);

        if total > 10000.0 {
        let discount:f64 = total - (0.05 * total);
        println!("You get a 5% discount!! Your new total is {}",discount);
        }
        else {
        println!("Your total is {}",total);
        }
    }
    else if foodtype == 4.0 {
        let total = quantity * pricee;
        println!("Total is {}",total);

        if total > 10000.0 {
        let discount:f64 = total - (0.05 * total);
        println!("You get a 5% discount!! Your new total is {}",discount);
        }
        else {
        println!("Your total is {}",total);
        }
    }
    else {
        let total = quantity * pricew;
        println!("Total is {}",total);

        if total > 10000.0 {
        let discount:f64 = total - (0.05 * total);
        println!("You get a 5% discount!! Your new total is {}",discount);
        }
        else {
        println!("Your total is {}",total);
        }
    }
}