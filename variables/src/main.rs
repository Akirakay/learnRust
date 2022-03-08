// struct Struct {
//     e: i32,
// }

fn main() {
    // println!("1. mutable var");
    // let mut x = 5;
    // println!("The value of x is: {}", x);
    // x = 6;
    // println!("The value of x is: {}", x);

    // println!("2. let deconstruction");
    // let (a, mut b): (bool, bool) = (true, false);
    // // a = true,不可变; b = false，可变
    // println!("a = {:?}, b = {:?}", a, b);

    // b = true;
    // assert_eq!(a, b);

    // println!("3. deconstruction");
    // let (a, b, c, d, e);

    // (a, b) = (1, 2);
    // [c, .., d, _] = [1, 2, 3, 4, 5];
    // Struct { e, .. } = Struct { e: 5 };

    // assert_eq!([1, 2, 1, 4, 5], [a, b, c, d, e]);

    println!("4. var shadowing");
    let x = 5;
    // 在main函数的作用域内对之前的x进行遮蔽
    let x = x + 1;

    {
        // 在当前的花括号作用域内，对之前的x进行遮蔽
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }

    println!("The value of x is: {}", x);
}
