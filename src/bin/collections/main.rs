// https://doc.rust-lang.org/book/ch08-00-common-collections.html
fn main() {
    vectors();
    string_utf8();
    indexing_into_strings();
    hash_maps();
}

fn vectors() {
    // creating
    let _v: Vec<i32> = Vec::new();
    let _v = vec![1, 2, 3]; // macro with auto type T detection

    // writing
    let mut v = Vec::new();
    v.push(5);
    dbg!(v.as_slice());

    // reading elements
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    // iterate
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }

    // iterate and change
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;   // de-reference in order to change the value
        println!("{}", i);
    }

    // use enum to store different types
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    dbg!(row);
}

#[allow(unused_variables)]
fn string_utf8() {
    // create
    let s = String::new();

    let data = "initial contents";
    let s = data.to_string();

    // the method also works on a literal directly:
    let s = "initial contents".to_string();

    let hello = String::from("안녕하세요"); // they are UTF-8 encoded

    // update
    let mut s = String::from("foo");
    s.push_str("bar");

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    s1.push('1');
    println!("s1 is {}", s1);

    // concat
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = s1 + "-" + &s2 + "-" + &s3; // s1 is copied so cannot be used anymore
    // dbg!(s1); // will fail

    let s1 = String::from("tic");
    let s = format!("{}-{}-{}", s1, s2, s3); // more readable and s1 is not copied here, only borrowed.

    dbg!(s);
    dbg!(s1); // still valid!
}

fn indexing_into_strings() {
    let hello = "Здравствуйте";
    // let answer = &hello[0]; // this is not possible in RUST because of internal storage of bytes
    let s = &hello[0..4];
    dbg!(s);

    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }
}

fn hash_maps() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    dbg!(scores);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let scores: HashMap<_, _> =
        teams.into_iter().zip(initial_scores.into_iter()).collect();
    dbg!(scores);

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    dbg!(&map);

    let field_name = String::from("Favorite color");
    let color = map.get(&field_name);
    dbg!(color);

    // iterate
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // overwrite value
    scores.insert(String::from("Blue"), 25);
    println!("{:?}", scores);

    // only insert if no value
    scores.entry(String::from("Blue")).or_insert(50); // not overwritten
    scores.entry(String::from("Orange")).or_insert(50);
    println!("{:?}", scores);

    // Updating a Value Based on the Old Value
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        // The or_insert method actually returns a mutable reference (&mut V) to the value for this key.
        let count = map.entry(word).or_insert(0);
        *count += 1; // dereference to update value of the hashmap
    }
    println!("{:?}", map);
}