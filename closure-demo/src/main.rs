// fn main() {
//     let x = 1;
//     let sum = |y| x + y;

//     assert_eq!(3, sum(2));
// }

// #![allow(unused)]
// fn main() {
//     let example_closure = |x| x;

//     let s = example_closure(String::from("hello"));
//     let n = example_closure(5);
// }

// struct Cacher<T>
// where
//     T: Fn(u32) -> u32,
// {
//     query: T,
//     value: Option<u32>,
// }

// fn main() {
// impl<T> Cacher<T>
// where
//     T: Fn(u32) -> u32,
// {
//     fn new(query: T) -> Cacher<T> {
//         Cacher {
//             query,
//             value: None,
//         }
//     }

//     // 先查询缓存值 `self.value`，若不存在，则调用 `query` 加载
//     fn value(&mut self, arg: u32) -> u32 {
//         match self.value {
//             Some(v) => v,
//             None => {
//                 let v = (self.query)(arg);
//                 self.value = Some(v);
//                 v
//             }
//         }
//     }
// }
// }

// fn fn_once<F>(func: F)
// where
//     F: FnOnce(usize) -> bool,
// {
//     println!("{}", func(3));
//     println!("{}", func(4));
// }

// fn main() {
//     let x = vec![1, 2, 3];
//     fn_once(|z| z == x.len())
// }

// fn main() {
//     let mut s = String::new();

//     let update_string = |str| s.push_str(str);
//     update_string("hello");

//     println!("{:?}", s);
// }

// fn main() {
//     let mut s = String::new();

//     let update_string = |str| s.push_str(str);

//     exec(update_string);

//     println!("{:?}", s);
// }

// fn exec<'a, F: Fn(&'a str)>(mut f: F) {
//     f("hello")
// }

fn main() {
    let s = String::new();

    let update_string = || println!("{}", s);

    exec(update_string);
    exec1(update_string);
    exec2(update_string);
}

fn exec<F: FnOnce()>(f: F) {
    f()
}

fn exec1<F: FnMut()>(mut f: F) {
    f()
}

fn exec2<F: Fn()>(f: F) {
    f()
}
