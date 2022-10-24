fn main() {
    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let home = IpAddr::V4(127, 0, 0, 1);

    let work = IpAddr::V6(String::from("::1"));

    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }
    impl Message {
        fn call(&self) {
            // method body would be defined here
        }
    }

    // Create a message of type Write
    let m = Message::Write(String::from("hello"));
    m.call();

    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    // Illegal b/c we cannot use option as a type until it has been unwrapped
    // this prevents us from using a val we assume is not null when it might be
    // let sum = x + y;

    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn move_player(num_spaces: u8) {}

    println!("{}", 3u8);

    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }
    // This is equivalent in this case to the above match
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }
    // if let as syntax sugar for a match that runs code when the value matches
    //    one pattern and then ignores all other values.
    // (no exhaustive case checking, can be dangerous)


    #[derive(Debug)] // so we can inspect the state in a minute
    enum UsState {
        Alabama,
        Alaska,
        // --snip--
    }

    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }

    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("State quarter from {:?}!", state);
                25
            }
        }
    }

    // count all non-quarter coins we see while also announcing the state of 
    // the quarters, we could do that with a match expression like this:
    let mut count = 0;
    let coin = Coin::Dime;
    match coin {
        Coin::Quarter(state) => println!("State quarter from {:?}!", state),
        _ => count += 1,
    }
    // or equivalently 
    let coin = Coin::Quarter(UsState::Alabama);
    if let Coin::Quarter(state) = coin {
        // Note: this moves & owns the state value so it cannot be used anymore
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
}