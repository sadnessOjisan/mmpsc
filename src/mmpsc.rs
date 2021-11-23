pub mod mmpsc {
    trait SendSync: Send + Sync {}

    use std::{ops::Not, sync::Arc};

    struct Sender<T: SendSync> {
        queue: Arc<Vec<T>>,
    }

    impl<T: SendSync> Sender<T> {
        fn send(&mut self, item: T) {
            self.queue.push(item) // Q: Arc の中のメソッドそのまま呼べるのか？
        }
    }

    struct Receiver<T: SendSync> {
        queue: Arc<Vec<T>>,
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
        let queue: Arc<Vec<T>> = Arc::new(Default::default());
        (
            Sender {
                queue: queue.clone(),
            },
            Receiver {
                queue: queue.clone(),
            },
        )
    }
}
