fn main() {
    println!("Hello, world!");

    another_function(5);

    //defining multiple characters
    print_labelled_measurments(5, 'h');

    //Expressions return a value, statements don't
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);

    //Call function with return value
    let x = five();

    println!{"The value of x is: {}", x};

    //call plus one
    let x = plus_one(5);
    println!("The value of x (plus_one) is now: {}", x);
}



//Functions

fn another_function(x: i32) {
    println!("The value of x is {}", x);
}

fn print_labelled_measurments(value: i32, unit_label: char){
    println!("The measurement is: {}{}", value, unit_label);
}

//functions with return values
//we dont name return values but we must declare their type with an arrow
//We can return early using return ketwork and specifying a value, but most functions reture last expression
fn five() -> i32 {
    5
}

//ends in an expression, not a statement (;)
fn plus_one(x: i32) -> i32 {
    x + 1
}