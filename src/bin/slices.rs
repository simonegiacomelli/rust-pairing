fn main() {
    let mut s = String::from("hello world");
    let fw = first_word(&mut s);
    // s.clear();
    println!("{fw}")
}

// const x: &str = "ciccio pasticcio";

fn first_word(s: &mut String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    s
}