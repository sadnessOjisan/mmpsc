use std::{cell::RefCell, collections::HashMap, sync::mpsc::{Receiver, Sender, channel}, thread::spawn};

fn get_rand() -> u32 {
    let num1: Vec<u32> = vec![2, 3];
    let address1 = &num1 as *const Vec<u32>;
    let number1 = address1 as u32;
    let ranged = number1 % 100;
    ranged
}

fn main() {
    let mut store = HashMap::<u32, RefCell<Sender<u32>>>::default();

    loop {
        spawn(move || {
            let range = 1..100;
            range.for_each(|x| {
                let random = get_rand();
                store.entry(random).or_insert_with(|| {
                    let (tx, rx) = channel::<u32>();
                    println!("strt");
                    spawn(move || {
                        loop {
                            let val = rx.recv();
                            match val {
                                Result::Ok(v) => {
                                    println!("success")
                                }
                                Result::Err(e) => {
                                    println!("error")
                                }
                            }
                        };
                    });
                    RefCell::new(tx)
                });
              
            });
        });
    }
    
}
