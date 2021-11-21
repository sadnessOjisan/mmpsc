use futures::{
    future::BoxFuture,
    task::{waker_ref, ArcWake},
    Future, FutureExt,
};
use std::{
    sync::{
        mpsc::{sync_channel, Receiver, SyncSender},
        Arc, Mutex,
    },
    task::Context,
};

// Pin: move したらもうい使わないからということでアドレスを変える

struct Task {
    // 実行するコルーチン
    // Q:なんでこんな型？
    // ouput: associated type(関連型), trait が持っている型エイリアス
    future: Mutex<BoxFuture<'static, ()>>,
    // Exector へスケジューリングするためのチャネル
    // dyn で vtable, box のポインタで検索
    sender: SyncSender<Arc<Task>>,
}

impl ArcWake for Task {
    // future を実行する親が必要
    // 実行してもらえるようにスケジューラに送り込む
    fn wake_by_ref(arc_self: &Arc<Self>) {
        let self0 = arc_self.clone();
        arc_self.sender.send(self0).unwrap();
    }
}

pub struct Execcutor {
    sender: SyncSender<Arc<Task>>,
    receiver: Receiver<Arc<Task>>,
}

// queue から タスクを取り出し、タスクの中のコルーチンを実行
impl Execcutor {
    pub fn new() -> Self {
        let (sender, receiver) = sync_channel(1024);
        Execcutor {
            sender: sender.clone(), // Q: なぜ clone が必要？
            receiver,
        }
    }

    pub fn get_spawner(&self) -> Spawner {
        Spawner {
            // Q: clone した sender は対の receiver に値を送り込める？
            sender: self.sender.clone(),
        }
    }

    pub fn run(&self) {
        while let Ok(task) = self.receiver.recv() {
            // Q: 一般的に future ってどういう意味？
            // future: task, js: future promise
            let mut future = task.future.lock().unwrap();
            let waker = waker_ref(&task);
            let mut ctx = Context::from_waker(&waker);
            // Q: 一般的に poll ってどういう意味？ あと、wake はどういう意味？
            // poll に対して task の問い合わせ
            // wake: コルーチンの起動
            let _ = future.as_mut().poll(&mut ctx);
        }
    }
}

pub struct Spawner {
    sender: SyncSender<Arc<Task>>,
}

impl Spawner {
    pub fn spawn(&self, future: impl Future<Output = ()> + 'static + Send) {
        let future = future.boxed();
        // Q: Arc の使い道がよく分からない. Arc じゃなければ何が困るのだろうか。参照するだけならcloneでもいいのでは？
        let task = Arc::new(Task {
            future: Mutex::new(future),
            sender: self.sender.clone(),
        });
        self.sender.send(task).unwrap();
    }
}

struct mmpsc {
    queue: Vec<>
}

trait Hoge:Send + Sync {
}

mod mmpsc {
    use crate::Hoge;
    struct  Sender<T : Hoge>  {
        queue: Arc<Vec<T>>
    }

    impl Sender {
        fn send(){}
    }

    struct Receiver<T> {
        queue: Arc<Vec<T>>
    }

    impl Receiver{
        fn receive( ){}
    }

    fn channel<T: Hoge>() -> (Sender, Receiver) where T: Hoge {
        let queue = Arc::new::<Vec<T>>();
        (Sender{
            queue: queue.clone()
        }, Receive{queue: queue.clone()})
    }
}

impl mmpsc {
    fn send(){
        self.queue.push(hoge)
    }

    fn receive(){
        
    }
}

pub fn hello_from_lib() {
    println!("hello from lib")
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
