// https://doc.rust-lang.org/book/ch09-00-error-handling.html
// https://doc.rust-lang.org/book/ch09-03-to-panic-or-not-to-panic.html
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    error_scenarios();
    propagate_errors();
    propagate_errors_with_shortcut();
    custom_types_for_validation();
    return Ok(()); // just returning Result enum, () variant which is the unit type or an empty tuple.
}

fn error_scenarios() {
    use std::fs::File;
    use std::io::ErrorKind;
    let f = File::open("hello.txt");

    let _f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };

    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
    dbg!(f);

    let _f = File::open("hello.txt").unwrap(); // match shortcut to get Ok or panic
    let f = File::open("hello.txt").expect("Failed to open hello.txt"); // like to unwrap but to specify panic message
    dbg!(f);

    std::fs::remove_file("hello.txt").unwrap_or_else(|error| {
        panic!("Problem removing the file: {:?}", error);
    });
    println!("File cleaned");
}

fn propagate_errors() {
    use std::fs::File;
    use std::io::{self, Read};

    let username = read_username_from_file();
    let u = match username {
        Ok(u) => u,
        Err(_error) => "Default username".to_string(),
    };
    println!("{}", u);

    fn read_username_from_file() -> Result<String, io::Error> {
        let f = File::open("hello.txt");

        let mut f = match f {
            Ok(file) => file,
            Err(e) => return Err(e),
        };

        let mut s = String::new();

        match f.read_to_string(&mut s) {
            Ok(_) => Ok(s),
            Err(e) => Err(e),
        }
    }
}

fn propagate_errors_with_shortcut() {
    use std::fs::File;
    use std::io;
    use std::io::Read;

    /*
    The ? placed after a Result value is defined to work in almost the same way
    as the match expressions we defined to handle the Result values in previous example.
    If the value of the Result is an Ok, the value inside the Ok will get returned from this expression,
    and the program will continue.
    If the value is an Err, the Err will be returned from the whole function
    as if we had used the return keyword so the error value gets propagated to the calling code.
     */
    fn read_username_from_file() -> Result<String, io::Error> {
        let mut f = File::open("hello.txt")?; // notice the ?
        let mut s = String::new();
        f.read_to_string(&mut s)?; // notice the ?
        Ok(s)
    }

    let username = read_username_from_file();
    let u = match username {
        Ok(u) => u,
        Err(_error) => "Default username".to_string(),
    };
    println!("{}", u);

    // even shorter syntax
    fn read_username_from_file_shortened() -> Result<String, io::Error> {
        let mut s = String::new();
        File::open("hello.txt")?.read_to_string(&mut s)?;
        Ok(s)
    }

    let username = read_username_from_file_shortened();
    let u = match username {
        Ok(u) => u,
        Err(_error) => "Default username".to_string(),
    };
    println!("{}", u);

    // and this is even shorter with fs function that manages everything for us.
    fn read_username_from_file_using_fs() -> Result<String, io::Error> {
        std::fs::read_to_string("hello.txt")
    }

    let username = read_username_from_file_using_fs();
    let u = match username {
        Ok(u) => u,
        Err(_error) => "Default username".to_string(),
    };
    println!("{}", u);
}

fn custom_types_for_validation() {
    #[allow(dead_code)]
    pub struct Guess {
        value: i32, // this value is private
    }

    #[allow(dead_code)]
    impl Guess {
        pub fn new(value: i32) -> Guess { // constructor/factory method
            if value < 1 || value > 100 {
                panic!("Guess value must be between 1 and 100, got {}.", value);
            }

            Guess { value }
        }

        pub fn value(&self) -> i32 { // getter
            self.value
        }
    }
}