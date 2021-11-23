pub mod mmpsc {
    trait SendSync: Send + Sync {}

    trait SendSyncSized: SendSync + Sized {}
    use std::sync::Arc;

    struct Sender<T: Send + Sync> {
        queue: Arc<Vec<T>>,
    }

    impl<T: Send + Sync> Sender<T> {
        fn send(&mut self, item: T) {
            self.queue.push(item) // Q: Arc の中のメソッドそのまま呼べるのか？
        }
    }

    struct Receiver<T: Send + Sync> {
        queue: Arc<Vec<T>>,
    }

    impl<T: Send + Sync> Receiver<T> {
        fn receive(&self) {
            loop {
                let item = self.queue.get(0);
            }
        }
    }

    fn channel<T: Send + Sync>() -> (Sender<T>, Receiver<T>)
    where
        T: Send + Sync,
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
