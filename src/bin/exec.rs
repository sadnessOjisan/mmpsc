use std::{
    thread::{self, spawn},
    time::Duration,
};

fn main() {
    use mmpsc::mmpsc::mmpsc;
    let (mut sender, receiver) = mmpsc::channel::<String>();
    spawn(move || loop {
        receiver.receive();
    });

    loop {
        let duration = Duration::from_millis(3000);
        thread::sleep(duration);
        println!("sleeped");
        sender.send("hello".to_string());
    }
}
