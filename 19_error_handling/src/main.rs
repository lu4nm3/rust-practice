use std::fs::File;
use std::io::ErrorKind;
use std::io;
use std::io::Read;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
//    panic!("crash and burn");



//    let v = vec![1, 2, 3];
//    v[99];



//    let f = File::open("hello.txt");
//
//    let f = match f {
//        Ok(file) => file,
//        Err(error) => {
//            panic!("There was a problem openinig file: {:?}", error)
//        }
//    };



//    let f = File::open("hello.txt");
//
//    let f = match f {
//        Ok(file) => file,
//        Err(error) => match error.kind() {
//            ErrorKind::NotFound => match File::create("hello.txt") {
//                Ok(fc) => fc,
//                Err(e) => panic!("Unable to create file: {:?}", e)
//            },
//            other_error => panic!("Problem opening file: {:?}", other_error)
//        }
//    };



//    let f = File::open("hello.txt").map_err(|error| {
//        if error.kind() == ErrorKind::NotFound {
//            File::create("hello.txt").unwrap_or_else(|error| {
//                panic!("Unable to create file: {:?}", error);
//            })
//        } else {
//            panic!("Problem opening file: {:?}", error);
//        }
//    });



//    let f = File::open("hello.txt").unwrap();



//    let f = File::open("hello.txt").expect("Failed to open file");



    let x = read_username_from_file();
    Ok(())
}

fn read_username_from_file() -> Result<String, io::Error> {
//    let f = File::open("hello.txt");
//
//    let mut f = match f {
//        Ok(file) => file,
//        Err(e) => return Err(e)
//    };
//
//    let mut s = String::new();
//
//    match f.read_to_string(&mut s) {
//        Ok(_) => Ok(s),
//        Err(e) => Err(e)
//    }



//    File::open("hello.txt")
//        .and_then(|mut file| file.read_to_string(&mut s))
//        .and_then(|_| Ok(s))



//    let mut f = File::open("hello.txt")?;
//    let mut s = String::new();
//    f.read_to_string(&mut s)?;
//    Ok(s)



//    let mut s = String::new();
//    let x = File::open("hello.txt")?.read_to_string(&mut s)?;
//    Ok(s)



    std::fs::read_to_string("hello.txt")
}