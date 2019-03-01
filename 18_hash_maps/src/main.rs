fn main() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();
    scores.insert(String::from("blue"), 10);
    scores.insert(String::from("yellow"), 50);



    let teams = vec![String::from("blue"), String::from("yellow")];
    let mut initial_scores = vec![10, 50];



    initial_scores.push(3);

    let scores: Vec<(&String, &i32)> = teams.iter().zip(initial_scores.iter()).collect();
    let scores: HashMap<&String, &i32> = teams.iter().zip(initial_scores.iter()).collect();

    let x = &initial_scores[0];

    println!("{}", x);
    for (x, y) in scores {
        println!("x: {}, y: {}", x, y);
    }



    let field_name = String::from("favorite color");
    let field_value = String::from("blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);

//    println!("{}", field_name);



    let mut scores = HashMap::new();
    scores.insert(String::from("blue"), 10);
    scores.insert(String::from("yellow"), 50);

    let team_name = String::from("blue");
    let score = scores.get(&team_name);

    println!("{}", score.unwrap());

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }



    let mut scores = HashMap::new();
    scores.insert(String::from("blue"), 10);
    scores.insert(String::from("blue"), 50);

    println!("{:?}", scores);



    let mut scores = HashMap::new();
    scores.insert(String::from("blue"), 10);

    scores.entry(String::from("yellow")).or_insert(50);
    let mut x = scores.entry(String::from("blue")).or_insert(50);

    *x = 33;
    println!("{:?}", scores);



    let text = "hello world cruel world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);




    let mut list = vec![3, 1, 2, 3];
    let sum: i32 = list.iter().sum();
    let mean = sum as f32 / list.len() as f32;

    list.sort();
    let median = list[list.len() / 2];

    let mut frequencies: HashMap<i32, i32> = HashMap::new();
    for i in &list {
        let count = frequencies.entry(*i).or_insert(0);
        *count += 1;
    }

    let mut enumeration: Vec<(&i32, &i32)> =  frequencies.iter().collect();
    enumeration.sort_by(|a, b| b.1.cmp(a.1));
    let mode = enumeration[0].0;

    println!("mean: {}, median: {}, mode: {}", mean, median, mode);



    fn to_pig_latin(s: &str) -> String {
        let cs: Vec<char> = s.chars().collect();

        match cs.split_first() {
            Some((first, last)) => {
                let last: String = last.iter().collect();
                format!("{}{}{}", last, first, "ay")
            },
            None => String::from("")
        }
    }

    println!("{}", to_pig_latin("luis"));




}
