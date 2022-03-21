// fn add<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
//     a + b
// }

// fn main() {
//     println!("add i8: {}", add(2i8, 3i8));
//     println!("add i32: {}", add(20, 30));
//     println!("add f64: {}", add(1.23, 1.23));
// }

// #[derive(Debug)]
// struct Point<T> {
//     x: T,
//     y: T,
// }

// fn main() {
//     let integer = Point { x: 5, y: 10 };
//     let float = Point { x: 1.0, y: 4.0 };
//     println!("integer Point: {:?}", integer);
//     println!("float Point: {:?}", float);
// }

// #![allow(unused)]
// fn main() {
//     enum Option<T> {
//         Some(T),
//         None,
//     }
// }

// struct Point<T> {
//     x: T,
//     y: T,
// }

// impl<T> Point<T> {
//     fn x(&self) -> &T {
//         &self.x
//     }
// }

// fn main() {
//     let p = Point { x: 5, y: 10 };

//     println!("p.x = {}", p.x());
// }

// fn display_array(arr: [i32; 3]) {
//     println!("{:?}", arr);
// }
// fn main() {
//     let arr: [i32; 3] = [1, 2, 3];
//     display_array(arr);

//     let arr: [i32; 2] = [1, 2];
//     display_array(arr);
// }

fn display_array<T: std::fmt::Debug, const N: usize>(arr: [T; N]) {
    println!("{:?}", arr);
}
fn main() {
    let arr: [i32; 3] = [1, 2, 3];
    display_array(arr);

    let arr: [i32; 2] = [1, 2];
    display_array(arr);
}
