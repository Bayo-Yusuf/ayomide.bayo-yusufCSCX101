fn main(){

   let n1 = "Electrical".to_string();
   let n2 =  "Electronic".to_string();
   let n3 =  "Engineering".to_string();
   let n4 =  n1 + &n2 + &n3; // n2 and n3 refrence is


    // About Electrical/Electronic
    println!("\nThe {} is infromed by the aspiration to
     train electrical/electronic engineering professionals
     in the areas of design, building and mentainace of
     electrical control systems,",n4);

    let w1 = "computer".to_string();
    let w2 = "Science".to_string();
    let w3 = w1 + &w2;   // w2 refrence is passed
    println!();
    println!("{} is aimed at developing component,creative,
      innovative, entrepreneurail and ethneically-minded persons,
      capable of creating value in the diverse fields of 
      computer Science.",w3);
}
