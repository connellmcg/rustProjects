fn main() {
    // set up string type 
    let s1 = String::from("hello");

    // pass reference &s1 into calculate_length
    // and in its definition, it takes &String instead of String
    // The ampersands represent references, you refer to the value
    // instead of taking ownership of it.
    // Because it doesnt own it, the value it points to will not
    // be dropped when it stops being used. 
    let len = calculate_length(&s1);
    
    println!("The length of {} is {}", s1, len);

    // Mutable reference
    let mut s = String::from("hello");
    change(&mut s);
    println!("{}",s );
}


fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
} // Here, s goes out of scope, but nothing happens because it does not 
// have ownership of what it refers to. 


fn change(some_string: &mut String) {
    some_string.push_str(", world!");
}