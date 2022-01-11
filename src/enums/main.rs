// https://doc.rust-lang.org/book/ch06-00-enums.html
fn main() {
    generic_ip();
    specialized_ip();
    variants();
    option_enum();
    match_control_flow_operator();
    matching_with_option_generic();
    catch_all_patterns();
    if_let_control_flow();
}

#[allow(dead_code)]
fn generic_ip() {
    #[derive(Debug)]
    enum IpAddrKind {
        V4,
        V6,
    }
    #[derive(Debug)]
    struct IpAddr {
        kind: IpAddrKind,
        address: String,
    }
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
    dbg!(home);
    dbg!(loopback);
}

#[allow(dead_code)]
fn specialized_ip() {
    #[derive(Debug)]
    enum IpAddr {
        V4(String),
        V6(String),
    }
    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));
    dbg!(home);
    dbg!(loopback);
}

#[allow(dead_code)]
fn variants() {
    #[derive(Debug)]
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }
    impl Message {
        fn call(&self) {
            dbg!(self);
        }
    }
    let q = Message::Quit;
    let mv = Message::Move { x: 1, y: 2 };
    let m = Message::Write(String::from("hello"));
    let c = Message::ChangeColor(1,2,3);
    q.call();
    mv.call();
    m.call();
    c.call();
}

fn option_enum() {
    let some_number = Some(5);
    let absent_number: Option<i32> = None;

    dbg!(some_number);

    match absent_number {
        Option::None => println!("None"),
        _ => println!("Anything else")
    }
}

fn match_control_flow_operator() {
    #[derive(Debug)] // so we can inspect the state in a minute
    enum UsState {
        Alaska,
    }

    enum Coin {
        Penny,
        Quarter(UsState),
    }
    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => {
                println!("Lucky penny!");
                1
            }
            Coin::Quarter(state) => {
                println!("State quarter from {:?}!", state);
                25
            }
        }
    }
    let coin = Coin::Penny;
    value_in_cents(coin);

    let coin = Coin::Quarter(UsState::Alaska);
    value_in_cents(coin);
}

fn matching_with_option_generic() {
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(placeholder) => Some(placeholder + 1), // placeholder can be anything (usually is "i")
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("six: {:?}, none: {:?}", six, none);
}

fn catch_all_patterns() {
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn move_player(num_spaces: u8) {
        println!("move player {} spaces", num_spaces);
    }

    // Here, we’re telling Rust explicitly that we aren’t going to use any other value that doesn’t match
    // a pattern in an earlier arm, and we don’t want to run any code in this case.
    let dice_roll = 7;
    match dice_roll {
        3 => println!("roll 3"),
        7 => println!("roll 7"),
        _ => (), // no value and no code to run
    }
}

fn if_let_control_flow() {
    // with match
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }

    // with if let instead of match
    let config_max = Some(3u16);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    } else { // optional else
        println!("not executed");
    }
}