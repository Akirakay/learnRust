// fn main() {
//     let s1 = Some("some1");
//     let s2 = Some("some2");
//     let n: Option<&str> = None;

//     let o1: Result<&str, &str> = Ok("ok1");
//     let o2: Result<&str, &str> = Ok("ok2");
//     let e1: Result<&str, &str> = Err("error1");
//     let e2: Result<&str, &str> = Err("error2");

//     assert_eq!(s1.or(s2), s1); // Some1 or Some2 = Some1
//     assert_eq!(s1.or(n), s1); // Some or None = Some
//     assert_eq!(n.or(s1), s1); // None or Some = Some
//     assert_eq!(n.or(n), n); // None1 or None2 = None2

//     assert_eq!(o1.or(o2), o1); // Ok1 or Ok2 = Ok1
//     assert_eq!(o1.or(e1), o1); // Ok or Err = Ok
//     assert_eq!(e1.or(o1), o1); // Err or Ok = Ok
//     assert_eq!(e1.or(e2), e2); // Err1 or Err2 = Err2

//     assert_eq!(s1.and(s2), s2); // Some1 and Some2 = Some2
//     assert_eq!(s1.and(n), n); // Some and None = None
//     assert_eq!(n.and(s1), n); // None and Some = None
//     assert_eq!(n.and(n), n); // None1 and None2 = None1

//     assert_eq!(o1.and(o2), o2); // Ok1 and Ok2 = Ok2
//     assert_eq!(o1.and(e1), e1); // Ok and Err = Err
//     assert_eq!(e1.and(o1), e1); // Err and Ok = Err
//     assert_eq!(e1.and(e2), e1); // Err1 and Err2 = Err1
// }

// defined error
// use std::fmt;

// struct AppError {
//     code: usize,
//     message: String,
// }

// // 根据错误码显示不同的错误信息
// impl fmt::Display for AppError {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         let err_msg = match self.code {
//             404 => "Sorry, Can not find the Page!",
//             _ => "Sorry, something is wrong! Please Try Again!",
//         };

//         write!(f, "{}", err_msg)
//     }
// }

// impl fmt::Debug for AppError {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(
//             f,
//             "AppError {{ code: {}, message: {} }}",
//             self.code, self.message
//         )
//     }
// }

// fn produce_error() -> Result<(), AppError> {
//     Err(AppError {
//         code: 404,
//         message: String::from("Page not found"),
//     })
// }

// fn main() {
//     match produce_error() {
//         Err(e) => eprintln!("{}", e), // 抱歉，未找到指定的页面!
//         _ => println!("No error"),
//     }

//     eprintln!("{:?}", produce_error()); // Err(AppError { code: 404, message: Page not found })

//     eprintln!("{:#?}", produce_error());
//     // Err(
//     //     AppError { code: 404, message: Page not found }
//     // )
// }

use std::fs::File;
use std::io;

#[derive(Debug)]
struct AppError {
    kind: String,    // 错误类型
    message: String, // 错误信息
}

// 为 AppError 实现 std::convert::From 特征，由于 From 包含在 std::prelude 中，因此可以直接简化引入。
// 实现 From<io::Error> 意味着我们可以将 io::Error 错误转换成自定义的 AppError 错误
impl From<io::Error> for AppError {
    fn from(error: io::Error) -> Self {
        AppError {
            kind: String::from("io"),
            message: error.to_string(),
        }
    }
}

fn main() -> Result<(), AppError> {
    let _file = File::open("nonexistent_file.txt")?;

    Ok(())
}
