fn main() {
   let mut s = String::from("hello world");
   
   let word = first_word(&s);

   s.clear();

   println!("{}", word);
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes(); //bytes is the array of bytes.

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}