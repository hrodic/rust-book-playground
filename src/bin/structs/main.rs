// https://doc.rust-lang.org/book/ch05-00-structs.html
#[derive(Debug)]
#[allow(dead_code)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    basic();
    builder();
    struct_update();
    tuple_struct();
    unit_like_struct();
    calculate_area();
    struct_methods();
    methods_with_params();
}

fn basic() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    println!("user: {:?}", user1);

    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");
    println!("user: {:?}", user1);
}

fn builder() {
    let user = build_user(String::from("name@example.com"), String::from("name"));

    fn build_user(email: String, username: String) -> User {
        User {
            email,
            username,
            active: true,
            sign_in_count: 1,
        }
    }
    println!("user: {:?}", user);
}

fn struct_update() {
    let user = User {
        active: true,
        username: String::from("the user name"),
        email: String::from("another@example.com"),
        sign_in_count: 2,
    };
    println!("user: {:?}", user);

    let user2 = User {
        email: String::from("updated@email.com"),
        ..user  // from this point we cannot user user anymore as it moved the data to user2
    };
    println!("user: {:?}", user2);
}

fn tuple_struct() {
    // naming a tuple without whole struct boilerplate
    #[derive(Debug)]
    struct Color(i32, i32, i32);
    #[derive(Debug)]
    struct Point(i16, i16, i16);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("Color: {:?}, Point: {:?}", black, origin);
}

fn unit_like_struct() {
    // without any field for traits
    struct AlwaysEqual;
    let _subject = AlwaysEqual;
}

fn calculate_area() {
    // no struct
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );

    fn area(width: u32, height: u32) -> u32 {
        width * height
    }

    // using tuples
    let rect1 = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area_tuple(rect1)
    );

    fn area_tuple(dimensions: (u32, u32)) -> u32 {
        dimensions.0 * dimensions.1
    }

    // using structs
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area_struct(&rect1)
    );

    fn area_struct(rectangle: &Rectangle) -> u32 {
        rectangle.width * rectangle.height
    }

    dbg!(&rect1); // debug message to stderr
}

fn struct_methods() {
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        fn area(&self) -> u32 {
            self.width * self.height
        }
        fn width(&self) -> u32 {
            self.width
        }
    }

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle of width {} is {} square pixels.",
        rect1.width(), rect1.area()
    );
}

fn methods_with_params() {
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }
    impl Rectangle {
        fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }
    }
    impl Rectangle { // you can use multiple impl blocks for the same struct
        fn build_square(size: u32) -> Rectangle { // factory method
            Rectangle {
                width: size,
                height: size,
            }
        }
    }

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let square = Rectangle::build_square(10);
    dbg!(square);
}

