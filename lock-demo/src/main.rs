// use std::sync::Mutex;

// fn main() {
//     // 使用`Mutex`结构体的关联函数创建新的互斥锁实例
//     let m = Mutex::new(5);

//     {
//         // 获取锁，然后deref为`m`的引用
//         // lock返回的是Result
//         let mut num = m.lock().unwrap();
//         *num = 6;
//         // 锁自动被drop
//     }

//     println!("m = {:?}", m);
// }

// use std::sync::{Arc, Mutex};
// use std::thread;

// fn main() {
//     let counter = Arc::new(Mutex::new(0));
//     let mut handles = vec![];

//     for _ in 0..10 {
//         let counter = Arc::clone(&counter);
//         let handle = thread::spawn(move || {
//             let mut num = counter.lock().unwrap();

//             *num += 1;
//         });
//         handles.push(handle);
//     }

//     for handle in handles {
//         handle.join().unwrap();
//     }

//     println!("Result: {}", *counter.lock().unwrap());
// }

// RwLock
// use std::sync::RwLock;

// fn main() {
//     let lock = RwLock::new(5);

//     // 同一时间允许多个读
//     {
//         let r1 = lock.read().unwrap();
//         let r2 = lock.read().unwrap();
//         assert_eq!(*r1, 5);
//         assert_eq!(*r2, 5);
//     } // 读锁在此处被drop

//     // 同一时间只允许一个写
//     {
//         let mut w = lock.write().unwrap();
//         *w += 1;
//         assert_eq!(*w, 6);

//         // 以下代码会panic，因为读和写不允许同时存在
//         // 写锁w直到该语句块结束才被释放，因此下面的读锁依然处于`w`的作用域中
//         // let r1 = lock.read();
//         // println!("{:?}",r1);
//     } // 写锁在此处被drop
// }

// Condvar
use std::sync::{Arc, Condvar, Mutex};
use std::thread::{sleep, spawn};
use std::time::Duration;

fn main() {
    let flag = Arc::new(Mutex::new(false));
    let cond = Arc::new(Condvar::new());
    let cflag = flag.clone();
    let ccond = cond.clone();

    let hdl = spawn(move || {
        let mut m = { *cflag.lock().unwrap() };
        let mut counter = 0;

        while counter < 3 {
            while !m {
                m = *ccond.wait(cflag.lock().unwrap()).unwrap();
            }

            {
                m = false;
                *cflag.lock().unwrap() = false;
            }

            counter += 1;
            println!("inner counter: {}", counter);
        }
    });

    let mut counter = 0;
    loop {
        sleep(Duration::from_millis(1000));
        *flag.lock().unwrap() = true;
        counter += 1;
        if counter > 3 {
            break;
        }
        println!("outside counter: {}", counter);
        cond.notify_one();
    }
    hdl.join().unwrap();
    println!("{:?}", flag);
}
