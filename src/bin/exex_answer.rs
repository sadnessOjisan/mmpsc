use std::{collections::HashMap, sync::mpsc::channel, thread::spawn};

fn get_rand() -> u32 {
    let num1: Vec<u32> = vec![2, 3];
    let address1 = &num1 as *const Vec<u32>;
    let number1 = address1 as u32;
    let ranged = number1 % 100;
    ranged
}

fn main() {
    let (tx, rx) = channel::<String>();
    let store = HashMap::<String, String>::default();

    // spawn(move || {
    //     let random = get_rand();
    //     println!("strt");
    //     loop {
    //         let val = rx.recv();
    //         match val {
    //             Result::Ok(v) => {
    //                 println!("success")
    //             }
    //             Result::Err(e) => {
    //                 println!("error")
    //             }
    //         }
    //     }
    // });
    let random = get_rand();
    println!("{}", random);
}
