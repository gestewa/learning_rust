fn main() {
    /* Ownership fundamentals

        Each value in Rust has an owner.
        There can only be one owner at a time.
        When the owner goes out of scope, the value will be dropped.
    
    */

    /* Stack & Heap
    
        Stack is for data whose size is fixed and known at compile time 
        
        Heap is for data whose size is not fixed or not known at compile time

        You get a pointer to data stored in the heap

    */

    /* Copy and Pop, Move and Drop and Clone

        Data which is stores in the stack is
            * Copy (trait)
            * Pop

        Data which is stored in the heap is 
            * Move 
            * Drop
        
        You can also clone() (trait) (deep copy) data in the heap

        A data type cannot implement both Copy and Drop on the same type. 

        Types that are Copy get implicitly duplicated by the compiler, making
            it very hard to predict when, and how often destructors will be 
            executed. As such, these types cannot have destructors.

        Types which implement Copy and Pop
            All the integer types, such as u32.
            The Boolean type, bool, with values true and false.
            All the floating point types, such as f64.
            The character type, char.
            Tuples, if they only contain types that also implement Copy. 
                For example, (i32, i32) implements Copy, (i32, String) doesn't.
            

    */
    
    /* Determine if drop trait is on a type
    
    let a = 1;
    let a_str = "asdfdsa";
    let a_string = String::from("asdfsdf");
    drop(a);
    drop(a_str);
    drop(a_string);
    println!("{a}");
    println!("{a_str}");
    println!("{a_string}"); // error

    Accessing a variable after a drop gives an error. We can use this to 
        determine if a variable is on stack or heap.
    */

    // Stored in the binary as a hardcoded data
    let a_str = "I am a cat"; // string literal, immutible, hardcoded
    // String, mutible (mut), not hardcoded (can be from user input)
    let mut a_string = String::from("I am a cat"); 
    a_string.push_str(" too");

    println!("{a_str} a_str in main 1"); 
    println!("{a_string} a_string in main 1");

    catify(a_str);
    catifyy(a_string); // move a_string

    println!("{a_str} a_str in main 2");
    // println!("{a_string}"); // invalid since 'moved'
}

fn catify(a: &str){
    println!("{a} a in catify");
}

fn catifyy(a: String){
    println!("{a} a in catifyy");
}

fn exec() {
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it's okay to still
                                    // use x afterward

} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn exec2() {
    let s1 = gives_ownership();         // gives_ownership moves its return
                                        // value into s1

    let s2 = String::from("hello");     // s2 comes into scope

    let s3 = takes_and_gives_back(s2);  // s2 is moved into
                                        // takes_and_gives_back, which also
                                        // moves its return value into s3
} // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
  // happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {             // gives_ownership will move its
                                             // return value into the function
                                             // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string                              // some_string is returned and
                                             // moves out to the calling
                                             // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
                                                      // scope

    a_string  // a_string is returned and moves out to the calling function
}