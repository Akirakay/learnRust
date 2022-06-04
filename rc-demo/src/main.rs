// use std::rc::Rc;
// fn main() {
//     let a = Rc::new(String::from("hello, world"));
//     let b = Rc::clone(&a);

//     assert_eq!(2, Rc::strong_count(&a));
//     assert_eq!(Rc::strong_count(&a), Rc::strong_count(&b))
// }

use std::rc::Rc;
fn main() {
    let a = Rc::new(String::from("test ref counting"));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Rc::clone(&a);
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Rc::clone(&a);
        println!("count after creating c = {}", Rc::strong_count(&c));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}
