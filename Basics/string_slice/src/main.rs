fn first_word(s: &str) -> &str {

    let bytes = s.as_bytes().iter().enumerate();

    for (i, &item) in bytes {

        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]

}

fn main() {
    let s = String::from("Hello world");
    println!("{}", first_word(&s))
}
