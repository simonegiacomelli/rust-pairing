use serde::__private::de::Content::String;

fn main() {
    let mut s = String::from("hello");

    // push_str() appends a literal to a String
    appendi(&mut s);

    println!("{}", s); // This will print `hello, world!`

    let s1 = String::from("hello");
    let s2 = s1;
}

fn appendi(s: &mut String) {
    println!("{:?}", s.len());
    s.push_str(", world!");
}

