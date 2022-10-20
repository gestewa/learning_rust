fn main() {
    /* Borrowing 
    
    Refrences: mutible, immutible, dangling
    
    You may only have one mutible refrence at one time.
    You may have as many immutible refrences as you would like at one time.

    You may not have immutible and mutible refrences at one time
    This is because the immutible refrence(s) should not have to to concern 
        themselves with the data changing out from under them unexpetedly

    A dangling refrnece occurs when you have a refrence (pointer) to a value 
        which has been deallocated. Rust does not allow this, see dangle()
    
    */

    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);


    let mut s = String::from("hello");

    let r1 = &s;
    calculate_length(r1);
    calculate_length(&s);
    calculate_length(&s);
    let r1 = "cat";
    change(&mut s);
    change(&mut s);
    println!("{s} s");
    println!("{r1} r1");
      let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);
      let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);
}

// a function which does not get ownership of a string
fn calculate_length(s: &String) -> usize { // s is pointer/reference to a String
    s.len()
}// Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, it is not dropped.

  fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// fn dangle() -> &String { // dangle returns a reference to a String

//     let s = String::from("hello"); // s is a new String

//     &s // we return a reference to the String, s
// } // Here, s goes out of scope, and is dropped. Its memory goes away.
//   // Danger!

fn no_dangle() -> String {
    let s = String::from("hello");

    s
}
