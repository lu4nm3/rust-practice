fn main() {
    fn largest_i32(list: &[i32]) -> i32 {
        let mut largest = list[0];

        for &item in list.iter() {
            if item > largest {
                largest = item;
            }
        }

        largest
    }

    fn largest_char(list: &[char]) -> char {
        let mut largest = list[0];

        for &item in list.iter() {
            if item > largest {
                largest = item;
            }
        }

        largest
    }

    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    println!("The largest char is {}", result);



    struct Point<T> {
        x: T,
        y: T
    }

    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };



    struct Point2<T, U> {
        x: T,
        y: U
    }

    let both_integer = Point2 { x: 5, y: 10 };
    let both_float = Point2 { x: 1.0, y: 4.0 };
    let integer_and_float = Point2 { x: 5, y: 4.0 };



    enum Option<T> {
        Some(T),
        None
    }

    enum Result<T, E> {
        Ok(T),
        Err(E)
    }



    struct Point3<T> {
        x: T,
        y: T
    }

    impl<T> Point3<T> {
        fn x(&self) -> &T {
            &self.x
        }
    }

    impl Point3<f32> {
        fn distance_from_origin(&self) -> f32 {
            (self.x.powi(2) + self.y.powi(2)).sqrt()
        }
    }



    struct Point4<T, U> {
        x: T,
        y: U,
    }

    impl<T, U> Point4<T, U> {
        fn mixup<V, W>(self, other: Point4<V, W>) -> Point4<T, W> {
            Point4 {
                x: self.x,
                y: other.y,
            }
        }
    }

    let p1 = Point4 { x: 5, y: 10.4 };
    let p2 = Point4 { x: "Hello", y: 'c'};

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);



//    fn largest<T>(list: &[T]) -> T {
//        let mut largest: T = list[0];
//
//        for &item in list.iter() {
//            if item > largest {
//                largest = item;
//            }
//        }
//
//        largest
//    }
//
//    let number_list = vec![34, 50, 25, 100, 65];
//
//    let result = largest(&number_list);
//    println!("The largest number is {}", result);
//
//    let char_list = vec!['y', 'm', 'a', 'q'];
//
//    let result = largest(&char_list);
//    println!("The largest char is {}", result);
}
