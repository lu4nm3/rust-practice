fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s); // 5

    s.clear(); // this empties the String, making it equal to ""

    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally invalid!



    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];

    let hello = &s[0..=4];
    let world = &s[6..=10];



    let s = String::from("hello world");

    let slice = &s[0..2];
    let slice = &s[..2];



    let s = String::from("hello world");

    let len = s.len();

    let slice = &s[0..len];
    let slice = &s[..];



//    let mut s = String::from("hello world");
//
//    let word = first_word(&s);
//
//    s.clear();
//
//    println!("the first word is: {}", word);



    let my_string = String::from("hello world");

    // first_word works on slices of `String`s
    let word = first_word(&my_string[..]);

    let my_string_literal = "hello world";

    // first_word works on slices of string literals
    let word = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);



    let a = [1,2,3,4,5];

    let slice = &a[1..3];
}

//fn first_word(s: &String) -> usize {
//    let bytes = s.as_bytes();
//
//    for (i, &item) in bytes.iter().enumerate() {
//        if item == b' ' {
//            return i;
//        }
//    }
//
//    s.len()
//}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}