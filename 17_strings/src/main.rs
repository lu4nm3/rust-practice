fn main() {
    let mut s = String::new();



    let data = "stuff";

    let s = data.to_string();

    let s = "stuff".to_string();



    let s = String::from("stuff");



    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");



    let mut s = String::from("foo");
    s.push_str("bar");



    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2);



    let mut s = String::from("lo");
    s.push('l');



    let s1 = String::from("hello, ");
    let s2 = String::from("world!");
    // compiler coerces the type &String to &str
    // add ('+') takes ownership of s1
    let s3: String = s1 + &s2;



    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);



    let len = String::from("Hola").len();

    println!("{}", len);

    let len = String::from("Здравствуйте").len();

    println!("{}", len);



//    let hello = "Здравствуйте";
//    let answer = &hello[0];

    let hello = String::from("Здравствуйте");
    let answer = &hello[0..2];

    println!("answer {}", answer);



    let s = String::from("नमस्ते");
    s.chars();



    for c in "नमस्ते".chars() {
        println!("{}", c);
    }



    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }
}
