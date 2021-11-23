pub mod mmpsc {
    trait SendSync: Send + Sync {}

    use std::{sync::{Arc, Mutex}};

    struct Sender<T: SendSync> {
        queue: Arc<Mutex<Vec<T>>>,
    }

    impl<T: SendSync> Sender<T> {
        fn send(&mut self, item: T) {
            self.queue.push(item)
        }
    }

    struct Receiver<T: SendSync> {
        queue: Arc<Mutex<Vec<T>>>,
    }

    impl<T: SendSync> Receiver<T> {
        fn receive(&self) -> Option<&T> {
            let mut item = None::<&T>;
            loop {
                item = self.queue.get(0);
                if (item.is_some()) {
                    break;
                }
            }
            item
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
