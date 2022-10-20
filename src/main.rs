fn main() {
    /* Slices
        
        A slice is a refrence and DOES NOT have ownership of the memory
        So it is an immutible refrence
        
        let a_str: &str = "Hello"; // This is a string literal
        let a_string: String = String::from("Hello");
        let a_slice: &str = &a_str[..]; // This is a slice
        let b_slice: &str = &a_string[..]; // This is a slice

        A string literal is a slice

    */

    // A look into a world without slices

    let mut s = String::from("hello world");

    let word = first_word_end_index(&s); // word will get the value 5

    s.clear(); // this empties the String, making it equal to ""

    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally invalid!
    println!("{word} word");

    // A look into a world with slices

    let s = String::from("hello world");

    let hello = &s[0..5];
    println!("{hello}");
    let world = &s[6..11];
    println!("{world}");
    let whole= &s[..]; // a slice of the whole string

    let fw = first_word(&s);
    println!("{fw}");
    let fw = first_word(whole);
    println!("{fw}");


    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);

}

fn first_word_end_index(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word(s: &str) -> &str {
    let bytes
 = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}