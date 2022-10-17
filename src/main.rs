fn a_function(x: i32) {
    println!("The value of x is {x}");
}
fn main() {
/* Functions

    fn keyword declares new functions
    Uses snake case as convention for function & variable names
    Functions are a series of statements optionally ending in an expression
    Statements are instructions that perform an action & do not return a value.
    Expressions evaluate to a resulting value.
*/

fn another_function() {
    println!("Another function.");
}

a_function(321);
another_function();

/* 
Statements

    Statements are instructions that perform an action & do not return a value.
    Statements do not return values. 

    let y = 6; // statement
    fn main() { let y = 6; } // statement    
    let x = (let y = 6); // ERROR cannot assign a statement to a statement

Expressions 

    Expressions evaluate to a resulting value.
    Expressions do not include ending semicolons.
    If you add a semicolon to the end of an expression, you turn it into a
        statement, and it will then not return a value.
    
    5 + 6 // expression
    6 // expression
    { let x = 3; x + 1 } // expression

*/
    
    fn five() -> i32 {
        5
    }
    let x = five();
    println!("The value of x is {x}");
    
    let x = plus_one(5);
    println!("The value of x is: {x}");

}

fn plus_one(x: i32) -> i32 {
    x + 1
    // it would be an error to say x + 1; here 
    // b/c then it would not be an expression
}