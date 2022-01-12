fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is:  {}", x);
    shadowing();
    floating();
    numeric_operations();
    character();
    tuple();
    array();
}

fn shadowing() {
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }
    println!("The value of x is: {}", x);
}

fn floating() {
    let x = 2.2; // f64
    let y: f32 = 3.5; // f32
    println!("The value of x is: {} and y is: {}", x, y);
}

fn numeric_operations() {
    // addition
    let _sum = 5 + 10;

    // subtraction
    let _difference = 95.5 - 4.3;

    // multiplication
    let _product = 4 * 30;

    // division
    let _quotient = 56.7 / 32.2;
    let _floored = 2 / 3; // Results in 0

    // remainder
    let _remainder = 43 % 5;
}

fn character() {
    let _c = 'z';
    let _z = 'ℤ';
    let _heart_eyed_cat = '😻';
}

fn tuple() {
    let _typed_tup: (i32, f64, u8) = (500, 6.4, 1);
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;

    println!("The value of x,y,z are: {} {} {}", x, y, z);

    let x: (i32, f64, u8) = (500, 6.4, 1);
    let _five_hundred = x.0;
    let _six_point_four = x.1;
    let _one = x.2;
}

fn array() {
    let a = [1, 2, 3, 4, 5];
    println!("len of a {}, last value {}", a.len(), a[4]);
    let months = ["January", "February", "March", "April", "May", "June", "July",
        "August", "September", "October", "November", "December"];
    println!("len of months {}, last value {}", months.len(), months[months.len()-1]);
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("len of a {}, last value {}", a.len(), a[a.len()-1]);
    let a = [3; 5];
    println!("len of a {}, last value {}", a.len(), a[a.len()-1]);
}