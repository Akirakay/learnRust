// trait Draw {
//     fn draw(&self) -> String;
// }

// impl Draw for u8 {
//     fn draw(&self) -> String {
//         format!("u8: {}", *self)
//     }
// }

// impl Draw for f64 {
//     fn draw(&self) -> String {
//         format!("f64: {}", *self)
//     }
// }

// fn draw1(x: Box<dyn Draw>) {
//     x.draw();
// }

// fn draw2(x: &dyn Draw) {
//     x.draw();
// }

// fn main() {
//     let x = 1.1f64;
//     // do_something(&x);
//     let y = 8u8;

//     draw1(Box::new(x));
//     draw1(Box::new(y));
//     draw2(&x);
//     draw2(&y);
// }

pub trait Draw {
    fn draw(&self) -> String;
}
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) -> String {
        // 绘制按钮的代码
        "Button".to_string()
    }
}

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) -> String {
        // 绘制SelectBox的代码
        "SelectBox".to_string()
    }
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            let str = component.draw();
            println!("{}", str);
        }
    }
}

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    screen.run();
}
