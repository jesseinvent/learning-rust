fn main() {
    let mut s = String::from("hello world");

    let slice = &s[0..2];

    println!("{}", slice)

}

fn first_words(s: &String) -> &str {
    // Convert string to array of bytes to check element by element
    let bytes = s.as_bytes(); 

    // Iterate over the array
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}