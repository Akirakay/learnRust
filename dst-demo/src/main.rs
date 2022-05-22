// #![allow(unused)]
// fn main() {
//     fn my_function(n: usize) {
//         let array = [123; n];
//     }
// }

// #![allow(unused)]
// fn main() {
//     // error
//     let s1: str = "Hello there!";
//     let s2: str = "How's it going?";

//     // ok
//     let s3: &str = "on?";
// }

// #![allow(unused)]
// fn main() {
//     fn foobar_1(thing: &dyn MyThing) {} // OK
//     fn foobar_2(thing: Box<dyn MyThing>) {} // OK
//     fn foobar_3(thing: MyThing) {} // ERROR!
// }

// #![allow(unused)]
// fn main() {
//     fn generic<T: ?Sized>(t: &T) {
//         // --snip--
//     }
// }

#![allow(unused)]
fn main() {
    let s1: Box<str> = "Hello there!".into();
}
