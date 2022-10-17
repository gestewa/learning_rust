fn main() {
    // Const cannot be mutible
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    // Scopes and shadowing
    // Shadowing allows us to change an immutible value
    println!("The value of x is: {THREE_HOURS_IN_SECONDS}");
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}"); // 12
    }

    println!("The value of x is: {x}"); // 6

    // Shadowing allows us to change type, mut does not
    let spaces = "   ";
    let spaces = spaces.len();
    println!("Spaces {spaces}");

    // e.g. not allowed
    // let mut spaces = "   ";
    // spaces = spaces.len();
    
    // Mutability demo
    let mut x = 5;
    println!("The value of x is: {x}"); // 5
    x = 6;
    println!("The value of x is: {x}"); // 6
}