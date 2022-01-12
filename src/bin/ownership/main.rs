// https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html
fn main() {
    copy_simple_values();
    string_to_heap();
    clone();
    functions();
    return_values();
}

fn copy_simple_values() {
    let x = 5;
    let y = x;
    println!("copy simple values x: {}, y: {}", x, y);
}

// s2 is stack copy of the pointer to s1 heap data
// s1 is no longer valid after s1 is copied to s2
// this is called a "move" in rust because it moves from one to the other
fn string_to_heap() {
    let s1 = String::from("hello world");
    let s2 = s1;

    println!("s1 string moved to s2 string: {}", s2);
}

// deep copy of heap data
fn clone() {
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);
}

fn functions() {
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);           // s's value moves into the function...
    // ... and so is no longer valid here
    let x = 5;                          // x comes into scope

    makes_copy(x);               // x would move into the function,
    // but i32 is Copy, so it's okay to still
    // use x afterward

    fn takes_ownership(some_string: String) { // some_string comes into scope
        println!("{}", some_string);
    } // Here, some_string goes out of scope and `drop` is called. The backing
    // memory is freed.

    fn makes_copy(some_integer: i32) { // some_integer comes into scope
        println!("{}", some_integer);
    } // Here, some_integer goes out of scope. Nothing special happens.
}

fn return_values() {
    let _s1 = gives_ownership();         // gives_ownership moves its return
    // value into s1

    let s2 = String::from("hello");     // s2 comes into scope

    let _s3 = takes_and_gives_back(s2);  // s2 is moved into
    // takes_and_gives_back, which also
    // moves its return value into s3

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
}