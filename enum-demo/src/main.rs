// #[derive(Debug)]
// enum PokerSuit {
//     Clubs,
//     Spades,
//     Diamonds,
//     Hearts,
// }

// fn main() {
//     let heart = PokerSuit::Hearts;
//     let diamond = PokerSuit::Diamonds;

//     print_suit(heart);
//     print_suit(diamond);
// }

// fn print_suit(card: PokerSuit) {
//     println!("{:?}", card);
// }

// #[derive(Debug)]
// enum PokerCard {
//     Clubs(u8),
//     Spades(u8),
//     Diamonds(u8),
//     Hearts(u8),
// }

// fn main() {
//     let c1 = PokerCard::Spades(5);
//     let c2 = PokerCard::Diamonds(13);
//     print_card(c1);
//     print_card(c2);
// }

// fn print_card(card: PokerCard) {
//     println!("{:?}", card);
// }

#![allow(unused)]
fn main() {
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    print_option_data(five);
    print_option_data(six);
    print_option_data(none);
}

fn print_option_data(x: Option<i32>) {
    match x {
        None => println!("None"),
        Some(i) => println!("{}", i),
    }
}
