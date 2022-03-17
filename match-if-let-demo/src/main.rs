// enum Direction {
//     East,
//     West,
//     North,
//     South,
// }

// fn main() {
//     let dire = Direction::South;
//     match dire {
//         Direction::East => println!("East"),
//         Direction::North | Direction::South => {
//             println!("South or North");
//         }
//         _ => println!("West"),
//     };
// }

// enum IpAddr {
//     Ipv4,
//     Ipv6,
// }

// fn main() {
//     // let d_panic = Direction::South;
//     let ip1 = IpAddr::Ipv6;
//     let ip_str = match ip1 {
//         IpAddr::Ipv4 => "127.0.0.1",
//         _ => "::1",
//     };

//     println!("{}", ip_str);
// }

// enum Action {
//     Say(String),
//     MoveTo(i32, i32),
//     ChangeColorRGB(u16, u16, u16),
// }

// fn main() {
//     let actions = [
//         Action::Say("Hello Rust".to_string()),
//         Action::MoveTo(1, 2),
//         Action::ChangeColorRGB(255, 255, 0),
//     ];
//     for action in actions {
//         match action {
//             Action::Say(s) => {
//                 println!("{}", s);
//             }
//             Action::MoveTo(x, y) => {
//                 println!("point from (0, 0) move to ({}, {})", x, y);
//             }
//             Action::ChangeColorRGB(r, g, _) => {
//                 println!(
//                     "change color into '(r:{}, g:{}, b:0)', 'b' has been ignored",
//                     r, g,
//                 );
//             }
//         }
//     }
// }

// #[derive(Debug)]
// enum MyEnum {
//     Foo,
//     Bar,
// }

// fn main() {
//     let v = vec![MyEnum::Foo, MyEnum::Bar, MyEnum::Foo];
//     v.iter()
//         .filter(|x| matches!(x, MyEnum::Foo))
//         .for_each(|x| println!("{:?}", x));
// }

fn main() {
    let age = Some(30);
    println!("在匹配前，age是{:?}", age);
    if let Some(age) = age {
        println!("匹配出来的age是{}", age);
    }

    println!("在匹配后，age是{:?}", age);
}
