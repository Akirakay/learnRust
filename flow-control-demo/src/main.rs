// fn main() {
//     let condition = true;
//     let number = if condition { 5 } else { 6 };

//     println!("The value of number is: {}", number);
// }

// fn main() {
//     for i in 1..=5 {
//         println!("{}", i);
//     }
// }

// fn main() {
//     let mut counter = 0;

//     let result = loop {
//         counter += 1;

//         if counter == 10 {
//             break counter * 2;
//         }
//     };

//     println!("The result is {}", result);
// }

// fn main() {
//     let names = [String::from("liming"), String::from("hanmeimei")];
//     for name in &names {
//         // do something with name...
//     }

//     println!("{:?}", names);

//     let numbers = [1, 2, 3];
//     // numbers中的元素实现了 Copy，因此无需转移所有权
//     for n in numbers {
//         // do something with name...
//     }

//     println!("{:?}", numbers);
// }

fn main() {
    let a = [4, 3, 2, 1];

    // 通过索引和值的方式迭代数组 `a`
    for (i, v) in a.iter().enumerate() {
        println!("第{}个元素是{}", i + 1, v);
    }
}
