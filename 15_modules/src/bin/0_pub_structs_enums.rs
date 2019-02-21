use crate::plant::Vegetable;

mod plant {
    pub struct Vegetable {
        pub name: String,
        id: i32
    }

    impl Vegetable {
        pub fn new(name: &str) -> Vegetable {
            Vegetable {
                name: String::from(name),
                id: 1
            }
        }
    }
}

mod menu {
    pub enum Appetizer {
        Soup,
        Salad
    }
}

fn main() {
    let mut v = plant::Vegetable::new("tomato");

    v.name = String::from("roma tomato");
    println!("{} are delicious", v.name);

//    The next line won't compile if we uncomment it:
//
//    print!("The id is {}", v.id);

//    Can't construct instance directly
//
//    let x = Vegetable {
//        name: String::from(""),
//        id: 3
//    };



    let order1 = menu::Appetizer::Soup;
    let order2 = menu::Appetizer::Salad;
}