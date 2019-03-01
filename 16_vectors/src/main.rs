fn main() {
    let v: Vec<i32> = Vec::new();



    let v = vec![1, 2, 3];



    let mut v = Vec::new();

    v.push(1);
    v.push(2);
    v.push(3);



    {
        let v = vec![1, 2, 3];
    }



    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The 3rd element is {}", third);

    match v.get(2) {
        Some(third) => println!("The 3rd element is {}", third),
        None => println!("There is no 3rd element")
    }



//    let mut v = vec![1, 2, 3, 4, 5];
//
//    let first = &v[0];
//
//    v.push(6);
//
//    println!("The first element is: {}", first);



    let v = vec![100, 32, 57];

    for i in &v {
        println!("{}", i);
    }



    let mut v = vec![100, 32, 57];

    for i in &mut v {
        *i += 50;
    }

    for i in &v {
        println!("{}", i);
    }



    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String)
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell:: Float(10.12)
    ];
}
