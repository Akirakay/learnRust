//
// use std::fs::File;
// use std::io::ErrorKind;

// fn main() {
//     let f = File::open("hello.txt");

//     let f = match f {
//         Ok(file) => file,
//         Err(error) => match error.kind() {
//             ErrorKind::NotFound => match File::create("hello.txt") {
//                 Ok(fc) => fc,
//                 Err(e) => panic!("Problem creating the file: {:?}", e),
//             },
//             other_error => panic!("Problem opening the file: {:?}", other_error),
//         },
//     };
// }

// use std::fs::File;

// fn main() {
//     let f = File::open("hello.txt").expect("Failed to open hello.txt");
// }

// #![allow(unused)]
// fn main() {
//     use std::fs::File;
//     use std::io::{self, Read};

//     fn read_username_from_file() -> Result<String, io::Error> {
//         // 打开文件，f是`Result<文件句柄,io::Error>`
//         let f = File::open("hello.txt");

//         let mut f = match f {
//             // 打开文件成功，将file句柄赋值给f
//             Ok(file) => file,
//             // 打开文件失败，将错误返回(向上传播)
//             Err(e) => return Err(e),
//         };
//         // 创建动态字符串s
//         let mut s = String::new();
//         // 从f文件句柄读取数据并写入s中
//         match f.read_to_string(&mut s) {
//             // 读取成功，返回Ok封装的字符串
//             Ok(_) => Ok(s),
//             // 将错误向上传播
//             Err(e) => Err(e),
//         }
//     }
// }

// #![allow(unused)]
// fn main() {
//     use std::fs::File;
//     use std::io;
//     use std::io::Read;

//     fn read_username_from_file() -> Result<String, io::Error> {
//         let mut f = File::open("hello.txt")?;
//         let mut s = String::new();
//         f.read_to_string(&mut s)?;
//         Ok(s)
//     }
// }

#![allow(unused)]
fn main() {
    fn first(arr: &[i32]) -> Option<&i32> {
        let v = arr.get(0)?;
        Some(v)
    }
}
