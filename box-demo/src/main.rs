// fn main() {
//     let b = foo("world");
//     println!("{}", b);
// }

// fn foo(x: &str) -> String {
//     let a = "Hello, ".to_string() + x;
//     a
// }

// fn main() {
//     let a = Box::new(3);
//     println!("a = {}", a); // a = 3

//     // 下面一行代码将报错
//     // let b = a + 1; // cannot add `{integer}` to `Box<{integer}>`
// }

// fn main() {
//     // 在栈上创建一个长度为1000的数组
//     let arr = [0; 1000];
//     // 将arr所有权转移arr1，由于 `arr` 分配在栈上，因此这里实际上是直接重新深拷贝了一份数据
//     let arr1 = arr;

//     // arr 和 arr1 都拥有各自的栈上数组，因此不会报错
//     println!("{:?}", arr.len());
//     println!("{:?}", arr1.len());

//     // 在堆上创建一个长度为1000的数组，然后使用一个智能指针指向它
//     let arr = Box::new([0; 1000]);
//     // 将堆上数组的所有权转移给 arr1，由于数据在堆上，因此仅仅拷贝了智能指针的结构体，底层数据并没有被拷贝
//     // 所有权顺利转移给 arr1，arr 不再拥有所有权
//     let arr1 = arr;
//     println!("{:?}", arr1.len());
//     // 由于 arr 不再拥有底层数组的所有权，因此下面代码将报错
//     // println!("{:?}", arr.len());
// }

trait Draw {
    fn draw(&self);
}

struct Button {
    id: u32,
}
impl Draw for Button {
    fn draw(&self) {
        println!("这是屏幕上第{}号按钮", self.id)
    }
}

struct Select {
    id: u32,
}

impl Draw for Select {
    fn draw(&self) {
        println!("这个选择框贼难用{}", self.id)
    }
}

fn main() {
    let elems: Vec<Box<dyn Draw>> = vec![Box::new(Button { id: 1 }), Box::new(Select { id: 2 })];

    for e in elems {
        e.draw()
    }
}
