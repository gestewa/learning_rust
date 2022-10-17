fn main() {

    // If else

    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    /*

        Note: You must be explicit and always provide if with a Boolean as its
            condition

        Consider this example: 

            let number = 3;
            if number {
                println!("number was three");
            }
            
        Since number is an i32, it is not a bool
        Since it is not a bool, Rust will not compile
    */

    if number != 0 {
        println!("number was something other than zero");
    }

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let number = if true { 5 } else { 6 };
    println!("The value of number is: {number}");
    // let number = if condition { 5 } else { "six" }; ERROR types

    // Loops
    /*

        There are three kinds of loops: 
            * loop
            * while
            * for
    */
    /* loop

        // runs forever until we explicitly tell it to stop
        loop {
            println!("again!");
        }
    */
    

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");

    // We can label loops

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
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
    println!("End count = {count}");

    /* while

        // checks a condition, can still run forever
        let number = 3
        while number != 0 {
            println!("{number}!");
        }
    */
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
    
    /* for
    */

    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }

    // (1..4) creates a range
    // rev() is for reversing
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");

}
