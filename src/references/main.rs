// https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html
fn main() {
    borrow();
    borrow_mutable();
    multiple_mutable_refs();
    no_dangle();
}

fn borrow() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    fn calculate_length(s: &String) -> usize {
        s.len()
    }
}

fn borrow_mutable() {
    let mut s = String::from("hello");
    change(&mut s);

    println!("The length of '{}' is {}.", s, s.len());

    fn change(some_string: &mut String) {
        some_string.push_str(", world");
    }
}

fn multiple_mutable_refs() { // no simultaneous to avoid race conditions on pointers
    let mut s = String::from("hello");
    let _immutable1 = &s;
    let _immutable2 = &s;    // immutable can be simultaneous
    {
        let r1 = &mut s;
        println!("reference 1: {}", r1);
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut s;
    println!("reference 2: {}", r2);
}

fn no_dangle() -> String {
    let s = String::from("hello");
    s // ownership of heap is moved out, no dangling pointer
    // &s  this would be invalid as will be dangled pointer.
}