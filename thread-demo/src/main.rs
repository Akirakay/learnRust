// use std::thread;
// use std::time::Duration;

// fn main() {
//     thread::spawn(|| {
//         for i in 1..10 {
//             println!("hi number {} from the spawned thread!", i);
//             thread::sleep(Duration::from_millis(1));
//         }
//     });

//     for i in 1..5 {
//         println!("hi number {} from the main thread!", i);
//         thread::sleep(Duration::from_millis(1));
//     }
// }

// join
// use std::thread;
// use std::time::Duration;

// fn main() {
//     let handle = thread::spawn(|| {
//         for i in 1..5 {
//             println!("hi number {} from the spawned thread!", i);
//             thread::sleep(Duration::from_millis(1));
//         }
//     });

//     handle.join().unwrap();

//     for i in 1..5 {
//         println!("hi number {} from the main thread!", i);
//         thread::sleep(Duration::from_millis(1));
//     }
// }

// move
// use std::thread;

// fn main() {
//     let v = vec![1, 2, 3];

//     let handle = thread::spawn(move || {
//         println!("Here's a vector: {:?}", v);
//     });

//     handle.join().unwrap();

//     // 下面代码会报错borrow of moved value: `v`
//     // println!("{:?}",v);
// }

// multi-thread
// use std::thread;
// use std::time::Duration;
// fn main() {
//     // 创建一个线程A
//     let new_thread = thread::spawn(move || {
//         // 再创建一个线程B
//         thread::spawn(move || loop {
//             println!("I am a new thread.");
//         })
//     });

//     // 等待新创建的线程执行完成
//     new_thread.join().unwrap();
//     println!("Child thread is finish!");

//     // 睡眠一段时间，看子线程创建的子线程是否还在运行
//     thread::sleep(Duration::from_millis(100));
// }

// barrier
// use std::sync::{Arc, Barrier};
// use std::thread;

// fn main() {
//     let mut handles = Vec::with_capacity(6);
//     let barrier = Arc::new(Barrier::new(6));

//     for _ in 0..6 {
//         let b = barrier.clone();
//         handles.push(thread::spawn(move || {
//             println!("before wait");
//             b.wait();
//             println!("after wait");
//         }));
//     }

//     for handle in handles {
//         handle.join().unwrap();
//     }
// }

// thread-local
// #![allow(unused)]
// fn main() {
//     use std::cell::RefCell;
//     use std::thread;

//     thread_local!(static FOO: RefCell<u32> = RefCell::new(1));

//     FOO.with(|f| {
//         assert_eq!(*f.borrow(), 1);
//         *f.borrow_mut() = 2;
//     });

//     // 每个线程开始时都会拿到线程局部变量的FOO的初始值
//     let t = thread::spawn(move || {
//         FOO.with(|f| {
//             assert_eq!(*f.borrow(), 1);
//             *f.borrow_mut() = 3;
//         });
//     });

//     // 等待线程完成
//     t.join().unwrap();

//     // 尽管子线程中修改为了3，我们在这里依然拥有main线程中的局部值：2
//     FOO.with(|f| {
//         assert_eq!(*f.borrow(), 2);
//     });
// }

// condition
// use std::sync::{Arc, Condvar, Mutex};
// use std::thread;

// fn main() {
//     let pair = Arc::new((Mutex::new(false), Condvar::new()));
//     let pair2 = pair.clone();

//     thread::spawn(move || {
//         let &(ref lock, ref cvar) = &*pair2;
//         let mut started = lock.lock().unwrap();
//         println!("changing started");
//         *started = true;
//         cvar.notify_one();
//     });

//     let &(ref lock, ref cvar) = &*pair;
//     let mut started = lock.lock().unwrap();
//     while !*started {
//         started = cvar.wait(started).unwrap();
//     }

//     println!("started changed");
// }

// call-once
use std::sync::Once;
use std::thread;

static mut VAL: usize = 0;
static INIT: Once = Once::new();

fn main() {
    let handle1 = thread::spawn(move || {
        INIT.call_once(|| unsafe {
            VAL = 1;
        });
    });

    let handle2 = thread::spawn(move || {
        INIT.call_once(|| unsafe {
            VAL = 2;
        });
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    println!("{}", unsafe { VAL });
}
