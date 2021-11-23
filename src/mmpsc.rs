pub mod mmpsc {
    use std::{sync::{Arc, Mutex}};
    trait SendSync: Send + Sync {}


    struct Sender<T: SendSync> {
        queue: Arc<Mutex<Vec<T>>>,
    }

    impl<T: SendSync> Sender<T> {
        fn send(&mut self, item: T) {
            // Q: 異常系はどうしたらいいだろうか？
            self.queue.lock().unwrap().push(item)
        }
    }

    struct Receiver<T: SendSync> {
        queue: Arc<Mutex<Vec<T>>>,
    }

    impl<T: SendSync> Receiver<T> {
        fn receive(&self) -> Option<&T> {
            let mut item = None::<&T>;
            loop {
                // Q: mpsc はロックフリーな queue だが、Arc の中を変更可能にするには Mutex を使う必要が出てくるのでは
                item = self.queue.lock().unwrap().get(0);
                if (item.is_some()) {
                    break;
                }
            }
            item // Q: ローカル変数への参照をどうやって返したらいいか？
        }
    }

    fn channel<T: SendSync>() -> (Sender<T>, Receiver<T>)
    where
        T: SendSync,
    {
        let queue: Arc<Mutex<Vec<T>>> = Arc::new(Mutex::new(Default::default()));
        (
            Sender {
                queue: Arc::clone(&queue)
            },
            Receiver {
                queue:  Arc::clone(&queue)
            },
        )
    }
}
