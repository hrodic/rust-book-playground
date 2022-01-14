use std::fmt::{Debug, Display, Formatter};

// https://doc.rust-lang.org/book/ch10-00-generics.html
fn main() {
    extract_function();
    generics();
    monomorphization(); // what compiler does with generics
    traits();
    trait_where_clause();
    return_types_that_implement_traits();
    trait_bounds_conditional_method_implementation();
    validate_references_with_lifetimes();
    lifetime_annotation_in_structs();
    lifetime_elision();
    generic_traits_and_lifetimes();
}

fn extract_function() {
    // extracting
    fn largest_no_generic(list: &[i32]) -> i32 {
        let mut largest = list[0];

        for &item in list {
            if item > largest {
                largest = item;
            }
        }

        largest
    }
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_no_generic(&number_list);
    println!("The largest number is {}", result);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let result = largest_no_generic(&number_list);
    println!("The largest number is {}", result);

    // generic
    fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
        let mut largest = list[0];

        for &item in list {
            if item > largest {
                largest = item;
            }
        }

        largest
    }
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("Generic: The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("Generic: The largest char is {}", result);
}

#[allow(dead_code)]
fn generics() {
    #[derive(Debug)]
    struct Point<T, U> {
        x: T,
        y: U, // to allow 2 different types
    }
    enum Option<T> {
        Some(T),
        None,
    }
    let both_integer = Point { x: 5, y: 10 };
    let both_float = Point { x: 1.0, y: 4.0 };
    let integer_and_float = Point { x: 5, y: 4.0 }; // int and float
    dbg!(both_integer);
    dbg!(both_float);
    dbg!(integer_and_float);

    // in methods
    impl<T, U> Point<T, U> {
        fn x(&self) -> &T {
            &self.x
        }
    }
    impl Point<f32, f32> {
        // method only for specific type
        fn distance_from_origin(&self) -> f32 {
            (self.x.powi(2) + self.y.powi(2)).sqrt()
        }
    }
    let p = Point { x: 5, y: 10 };

    println!("p.x = {}", p.x());

    // mixing type parameters
    struct Point2<X1, Y1> {
        x: X1,
        y: Y1,
    }
    impl<X1, Y1> Point2<X1, Y1> {
        fn mixup<X2, Y2>(self, other: Point2<X2, Y2>) -> Point2<X1, Y2> {
            Point2 {
                x: self.x,
                y: other.y,
            }
        }
    }
    let p1 = Point2 { x: 5, y: 10.4 };
    let p2 = Point2 { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}

#[allow(non_camel_case_types)]
#[allow(dead_code)]
fn monomorphization() {
    // this...
    let _integer = Some(5);
    let _float = Some(5.0);

    // becomes this...
    enum Option_i32 {
        Some(i32),
        None,
    }

    enum Option_f64 {
        Some(f64),
        None,
    }

    let _integer = Option_i32::Some(5);
    let _float = Option_f64::Some(5.0);
}

fn traits() {
    pub trait Summary {
        fn summarize_author(&self) -> String; // no implementation

        fn summarize(&self) -> String { // default implementation
            format!("(Read more from {}...)", self.summarize_author())
        }
    }

    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }

    impl Summary for NewsArticle {
        fn summarize_author(&self) -> String {
            format!("@{}", self.author) // implementation
        }
        fn summarize(&self) -> String { // override default implementation
            format!("{}, by {} ({})", self.headline, self.author, self.location)
        }
    }

    pub struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }

    impl Summary for Tweet {
        fn summarize_author(&self) -> String {
            format!("@{}", self.username) // implementation
        }
        // do not override default summarize()
    }

    impl Display for Tweet {
        // std::fmt
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.content)
        }
    }

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };
    println!("1 new tweet: {}", tweet.summarize());
    println!("New article available! {}", article.summarize());

    // traits as parameters
    pub fn notify(item: &impl Summary) {
        println!("Breaking news! {}", item.summarize());
    }
    notify(&article);

    // Trait bound syntax: forcing two parameters of the same type
    pub fn notify_sametype<T: Summary>(item1: &T, item2: &T) {
        println!("1: {}", item1.summarize());
        println!("2: {}", item2.summarize());
    }
    notify_sametype(&article, &article);

    // multiple trait bounds with + syntax
    pub fn notify_multiple_trait(item: &(impl Summary + Display)) {
        println!("Summary trait: {}", item.summarize());
        println!("Display trait: {}", item);
    }
    notify_multiple_trait(&tweet);
}

fn trait_where_clause() {
    // to specify multiple traits for multiple types in a cleaner syntax
    fn _some_function<T, U>(_t: &T, _u: &U) -> i32
        where T: Display + Clone,
              U: Clone + Debug
    {
        1
    }
}

fn return_types_that_implement_traits() {
    pub struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }
    pub trait Summary {}

    impl Summary for Tweet {}

    fn returns_summarizable() -> impl Summary {
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from(
                "of course, as you probably already know, people",
            ),
            reply: false,
            retweet: false,
        }
    }
    let _item = returns_summarizable();
}

fn trait_bounds_conditional_method_implementation() {
    struct Pair<T> {
        x: T,
        y: T,
    }
    impl<T> Pair<T> {
        fn new(x: T, y: T) -> Self {
            Self { x, y }
        }
    }
    impl<T: Display + PartialOrd> Pair<T> {
        fn cmp_display(&self) {
            if self.x >= self.y {
                println!("The largest member is x = {}", self.x);
            } else {
                println!("The largest member is y = {}", self.y);
            }
        }
    }

    let a = 1;
    let b = 2;
    let p = Pair::new(a, b);
    p.cmp_display();
}

// https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html
fn validate_references_with_lifetimes() {
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    let string1 = String::from("long string is long");
    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }

    let string1 = String::from("long string is long");
    let string2 = String::from("xyz");
    let result;
    {
        // this will also work because all variables have the same lifetime duration for outer scope
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {}", result);
}

fn lifetime_annotation_in_structs() {
    // This annotation means an instance of ImportantExcerpt can’t outlive the reference it holds in its part field.
    struct ImportantExcerpt<'a> {
        part: &'a str,
    }

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    println!("{}", i.part);
}

// https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html
#[allow(dead_code)]
fn lifetime_elision() {
    // compiler infers some lifetimes by default
    fn first_word<'a>(s: &'a str) -> &str {
        s
    }
    fn first_word_elided(s: &str) -> &str { // equivalent without lifetimes specified
        s
    }
    struct ImportantExcerpt<'a> {
        part: &'a str,
    }
    impl<'a> ImportantExcerpt<'a> {
        fn level(&self) -> i32 {
            3
        }
    }
    impl<'a> ImportantExcerpt<'a> {
        fn announce_and_return_part(&self, announcement: &str) -> &str {
            println!("Attention please: {}", announcement);
            self.part
        }
    }
    let i = ImportantExcerpt{
        part: "something"
    };
    println!("{} -> {}", i.part, i.level());
    i.announce_and_return_part(i.part);
}

fn generic_traits_and_lifetimes() {
    fn longest_with_an_announcement<'a, T>(
        x: &'a str,
        y: &'a str,
        ann: T,
    ) -> &'a str
        where
            T: Display,
    {
        println!("Announcement! {}", ann);
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    let longest = longest_with_an_announcement("aa", "bb", "announcement");
    println!("{}", longest);
}