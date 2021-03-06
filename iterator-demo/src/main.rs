// #![allow(unused)]
// fn main() {
//     let arr = [1, 2, 3];
//     for v in arr.into_iter() {
//         println!("{}", v);
//     }
// }

// fn main() {
//     let v1 = vec![1, 2, 3];

//     let v1_iter = v1.iter();

//     let total: i32 = v1_iter.sum();

//     assert_eq!(total, 6);

//     // v1_iter 是借用了 v1，因此 v1 可以照常使用
//     println!("{:?}", v1);

//     // 以下代码会报错，因为 `sum` 拿到了迭代器 `v1_iter` 的所有权
//     // println!("{:?}",v1_iter);
// }

#![allow(unused)]
fn main() {
    let v1: Vec<i32> = vec![1, 2, 3];

    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    assert_eq!(v2, vec![2, 3, 4]);
}
