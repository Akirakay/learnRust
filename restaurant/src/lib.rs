// // 餐厅前厅，用于吃饭
// mod front_of_house {
//     mod hosting {
//         fn add_to_waitlist() {}

//         fn seat_at_table() {}
//     }

//     mod serving {
//         fn take_order() {}

//         fn serve_order() {}

//         fn take_payment() {}
//     }
// }

// mod front_of_house {
//     mod hosting {
//         fn add_to_waitlist() {}
//     }
// }

// pub fn eat_at_restaurant() {
//     // 绝对路径
//     crate::front_of_house::hosting::add_to_waitlist();

//     // 相对路径
//     front_of_house::hosting::add_to_waitlist();
// }

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // 绝对路径
    crate::front_of_house::hosting::add_to_waitlist();

    // 相对路径
    front_of_house::hosting::add_to_waitlist();
}
