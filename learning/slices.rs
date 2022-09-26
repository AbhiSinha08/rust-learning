fn main() {
    let s = "Hello World!";
    let first = first_word(&s);
    println!("{first}");

    let arr = [1, 2, 3, 4, 5];

    let slice = &arr[1..3];

    assert_eq!(slice, &[2, 3]);
    println!("after assert");

}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (index, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..index];
        }
    }

    return &s;
}