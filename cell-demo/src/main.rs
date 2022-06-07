// use std::cell::Cell;
// fn main() {
//   let c = Cell::new("asdf");
//   let one = c.get();
//   c.set("qwer");
//   let two = c.get();
//   println!("{},{}", one, two);
// }

use std::cell::Cell;
fn main() {
    // code snipet 1
    let x = Cell::new(1);
    let y = &x;
    let z = &x;
    x.set(2);
    y.set(3);
    z.set(4);
    println!("{}", x.get());

    // code snipet 2
    let mut x = 1;
    let y = &mut x;
    let z = &mut x;
    x = 2;
    *y = 3;
    *z = 4;
    println!("{}", x);
}
