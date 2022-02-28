fn main() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

    loop {
        println!("remaining = {}", remaining);
        if remaining == 9 {
            break;
        }
        if count == 2 {
            break 'counting_up;
        }
        remaining -= 1;
    }

    count += 1;
    }
    println!("End count = {}", count);

    //return value from a loop
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {}", result);

    //conditional loop with a while
    let mut number = 3;

    while number !=0 {
        println!("{}!", number);_
        number -= 1;
    }
    println!("LIFTOFF!!!");

    //Loop through a collection with for.println!
    //This is slow as the condition had to be checked every run
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("the value is : {}", a[index]);

        index += 1;
    }

    // A more concise alternative is this: 
    // We also increase the safety as we dont need to check the size of the array
    // This is the most common way to write a loop in rust
    let a = [10,20,30,40,50];

    for element in a {
        println!("the value is: {}", element);
    }

    // We can use a for loop to do the countdown more efficiently too
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");

}
