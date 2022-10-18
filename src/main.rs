fn main() {

    // Temperatures between Fahrenheit and Celsius.
    println!("32 degress Fahrenheit is {} degrees Celsius", f_to_c(32));
    
    // Generate the nth Fibonacci number.
    println!("The 9th fibonacci number is {}", nth_fibonacci(9));
    
    // Print the lyrics to the Christmas carol “The Twelve Days of Christmas,” taking advantage of the repetition in the song.
    christmas_carol();
}

fn f_to_c(temp_in_f: i32)-> i32{
    (temp_in_f-32)*5/9
}

fn nth_fibonacci(n: u64) -> u64{
    let mut a = 0;
    let mut b = 1;
    let mut c: u64;
    if n ==0 {return a;}
    for _ in 1..n{
        c = a + b;
        a = b;
        b = c;
    }
    b
}

fn christmas_carol(){
    let days = ["first", "second", "third", "fourth", "fifth", "sixth", 
        "seventh", "eighth", "ninth", "tenth", "11th", "12th"];
    let items = [
        "A song and a Christmas tree",
        "Two candy canes",
        "Three boughs of holly",
        "Four colored lights",
        "A shining star",
        "Little silver bells",
        "Candles a-glowing",
        "Gold and silver tinsel",
        "A guardian angel",
        "Some mistletoe",
        "Gifts for one and all",
        "All their good wishes",
    ];
    
    let second_line = "My good friends brought to me";

    let pre_first_line = "On the ";
    let post_first_line = " day of Christmas";

    // After the first verse, we want the last line of each verse to be 
    //   "And a song and a Christmas tree" 
    let mut last_line = String::from("And ")+items[0];
    last_line = last_line.replace(" A ", " a ");

    let mut daily_items: Vec<String> = Vec::new();

    for i in 0..days.len(){
        if i == 1 {
            daily_items[0]=last_line.clone();
        }
        daily_items.insert(0, String::from(items[i]));
        
        println!(
            "{}{}{}\n{}\n{}\n",
            pre_first_line,
            days[i],
            post_first_line, 
            second_line,
            daily_items.join("\n")
        );
        
    }
}
