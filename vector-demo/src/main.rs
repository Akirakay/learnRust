// #![allow(unused)]
// fn main() {
//     let v = vec![1, 2, 3, 4, 5];

//     let third: &i32 = &v[2];
//     println!("第三个元素是 {}", third);

//     match v.get(2) {
//         Some(third) => println!("第三个元素是 {}", third),
//         None => println!("去你的第三个元素，根本没有！"),
//     }
// }

// #![allow(unused)]
// fn main() {
//     let v = vec![1, 2, 3];
//     for i in &v {
//         println!("{}", i);
//     }
// }

// #![allow(unused)]
// fn main() {
//     let mut v = vec![1, 2, 3];
//     for i in &mut v {
//         *i += 10
//     }
//     for i in &v {
//         println!("{}", i);
//     }
// }

// #[derive(Debug)]
// enum IpAddr {
//     V4(String),
//     V6(String),
// }
// fn main() {
//     let v = vec![
//         IpAddr::V4("127.0.0.1".to_string()),
//         IpAddr::V6("::1".to_string()),
//     ];

//     for ip in v {
//         show_addr(ip)
//     }
// }

// fn show_addr(ip: IpAddr) {
//     println!("{:?}", ip);
// }

trait IpAddr {
    fn display(&self);
}

struct V4(String);
impl IpAddr for V4 {
    fn display(&self) {
        println!("ipv4: {:?}", self.0)
    }
}
struct V6(String);
impl IpAddr for V6 {
    fn display(&self) {
        println!("ipv6: {:?}", self.0)
    }
}

fn main() {
    let v: Vec<Box<dyn IpAddr>> = vec![
        Box::new(V4("127.0.0.1".to_string())),
        Box::new(V6("::1".to_string())),
    ];

    for ip in v {
        ip.display();
    }
}
