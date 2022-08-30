fn main() {
    // Create new String
    // let mut s = String::new();

    // let data = "initail contents";
    // let s = data.to_string();
    // let s = String::from("data");

    // Update a String
    let mut s1 = String::from("foo");
    let s2 = String::from("bar");
    s1.push_str(&s2);
    println!("s2 is {}", s2);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("{}{}{} {}", s1, s2, s3, s);

    // String not accept indexing
    let hello = "Здравствуйте";
    let s = &hello[0..2];
    println!("{}", s);

    let hello = String::from("hello");
    let s = &hello[0..2];
    println!("{}", s);

    // Method to Iterating over String
    for c in "Зд".chars() {
        println!("{}", c);
    }

    for b in "Зд".bytes() {
        println!("{}", b);
    }

    // Strings are not so simple

}
