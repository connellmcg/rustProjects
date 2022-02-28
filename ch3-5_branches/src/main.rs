fn main() {

    let number = 7;

    if number < 5 {
        println!("Condition was true");
    } else{
    println!("Condition was false");
    }


    //Not equals
    let number = 3;

     if number != 0 {
         println!("Number was something other than zero.");
     }


     // multiple conditions
     let number = 6;
     // Rust breaks when first condition is met, so div by 2 is never executed - be careful with too many else if expressions
     // 
     if number % 4 == 0 {
         println!("Number is divisible by 4");
     } else if number % 3 == 0 {
         println!("Number is divisible by 3");
     } else if number % 2 == 0 {
         println!("Number is divisible by 2");
     } else {
         println!("Number is not divisible by 4,3 or 2");
     }

     //Use if in a let statement - if is an expression, so we can use it on right side of let statement
     let condition = true;
     let number = if condition { 5 } else { 6 };
      println! ("The value of number is: {}", number);
}
