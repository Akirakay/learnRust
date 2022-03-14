// fn main() {
//     let my_name = "Akira";
//     greet(my_name);
// }

// fn greet(name: String) {
//     println!("Hello, {}!", name);
// }

// #![allow(unused_variables)]
// fn main() {
//     let s = String::from("hello world");

//     let hello = &s[0..5];
//     let world = &s[6..11];
//     println!("{} {}", hello, world);
// }

// #![allow(unused)]
// fn main() {
//     let s = "中国人";
//     let a = &s[0..2];
//     println!("{}", a);
// }

// fn main() {
//     // 创建一个空String
//     let mut s = String::new();
//     // 将&str类型的"hello,world"添加到s中
//     s.push_str("hello,world");
//     // 将字符'!'推入s中
//     s.push('!');
//     // 最后s的内容是"hello,world!"
//     assert_eq!(s, "hello,world!");

//     // 从现有的&str切片创建String类型
//     let mut s = "hello,world".to_string();
//     // 将字符'!'推入s中
//     s.push('!');
//     // 最后s的内容是"hello,world!"
//     assert_eq!(s, "hello,world!");

//     // 从现有的&str切片创建String类型
//     // String与&str都是UTF-8编码，因此支持中文
//     let mut s = String::from("你好,世界");
//     // 将字符'!'推入s中
//     s.push('!');
//     // 最后s的内容是"你好,世界!"
//     assert_eq!(s, "你好,世界!");

//     let s1 = String::from("hello,");
//     let s2 = String::from("world!");
//     // 在下句中，s1的所有权被转移走了，因此后面不能再使用s1
//     let s3 = s1 + &s2;
//     assert_eq!(s3, "hello,world!");
//     // 下面的语句如果去掉注释，就会报错
//     // println!("{}",s1);
// }

// #![allow(unused)]
// fn main() {
//     let s1 = String::from("hello");
//     let h = s1[0];
// }

// #![allow(unused)]
// fn main() {
//     for c in "中国人".chars() {
//         println!("{}", c);
//     }
// }

#![allow(unused)]
fn main() {
    for b in "中国人".bytes() {
        println!("{}", b);
    }
}
