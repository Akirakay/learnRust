#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// fn main() {
//     // let user1 = User {
//     //     email: String::from("someone@example.com"),
//     //     username: String::from("someusername123"),
//     //     active: true,
//     //     sign_in_count: 1,
//     // };
//     let user1 = build_user1("zk_tao@163.com".to_string(), "akira".to_string());
//     println!("user1.email = {}", user1.email);
//     println!("user1.username = {}", user1.username);

//     // let user2 = build_user2("zk_tao@163.com".to_string(), "taozk".to_string());
//     // println!("user2.email = {}", user2.email);
//     // println!("user2.username = {}", user2.username);

//     let user2 = User {
//         email: String::from("another@example.com"),
//         // .. 语法表明凡是我们没有显示声明的字段全部从 user1 中自动获取
//         // ..user1 必须在结构体的尾部使用
//         ..user1
//     };

//     println!("user2.email = {}", user2.email);
//     println!("user2.username = {}", user2.username);
//     println!("user2 = {:?}", user2);
//     println!("user2 = {:#?}", user2);
// }

// fn build_user1(email: String, username: String) -> User {
//     User {
//         email: email,
//         username: username,
//         active: true,
//         sign_in_count: 1,
//     }
// }

// fn build_user2(email: String, username: String) -> User {
//     User {
//         email,
//         username,
//         active: true,
//         sign_in_count: 1,
//     }
// }

// fn main() {
//     let user1 = User {
//         email: String::from("someone@example.com"),
//         username: String::from("someusername123"),
//         active: true,
//         sign_in_count: 1,
//     };
//     let user2 = User {
//         active: user1.active,
//         username: user1.username,
//         email: String::from("another@example.com"),
//         sign_in_count: user1.sign_in_count,
//     };
//     println!("{}", user1.active);
//     // 下面这行会报错
//     // println!("{:?}", user1);
// }

// #[allow(unused)]
// fn main() {
//     struct Color(i32, i32, i32);
//     struct Point(i32, i32, i32);

//     let black = Color(0, 0, 0);
//     let origin = Point(0, 0, 0);
// }

#[allow(unused)]
fn main() {
    struct AlwaysEqual;

    let subject = AlwaysEqual;

    // 我们不关心 AlwaysEqual 的字段数据，只关心它的行为，因此将它声明为单元结构体，然后再为它实现某个特征
    // impl SomeTrait for AlwaysEqual {}
}
