//Rust program to determine the annual incentive based on certain criteria

use std::io;

fn main() {

    println!("Experienced = 10 years or more");

    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

    println!("Enter the name of employee: ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");

    println!("\nHow many years of experience does this employee have? {}", input2);
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let experience:i32 = input2.trim().parse().expect("Not a valid number");


    if experience >=10 {  

        println!("Input age of employee: ");
        io::stdin().read_line(&mut input3).expect("Not a valid string");
        let age:i32 = input3.trim().parse().expect("Not a valid number");

     if age >=40 {
            println!("The incentive of this employee is N1,560,000");
        }
     else if 30 <= age && age <40 {
            println!("The incentive of this epmloyee is N1,480,000");
        }
     else {
            println!("The incentive of this employee is N1,300,000");
        }
    }
     else {
        println!("The incentive of this employee is N100,000.");
    }
}
