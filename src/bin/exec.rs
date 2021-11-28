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
let mut cnt =0;
    loop {
        let duration = Duration::from_millis(1000);
        thread::sleep(duration);
        sender.send(cnt.to_string());
        cnt += 1;
    }
}
