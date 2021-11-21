use std::task::Poll;

use futures::{task::SpawnExt, Future};
use mmpsc::{hello_from_lib, Execcutor};

enum StateHello {
    HELLO,
    WORLD,
    END,
}

impl Hello {
    fn new() -> Self {
        Hello {
            state: StateHello::HELLO,
        }
    }
}

struct Hello {
    state: StateHello,
}

impl Future for Hello {
    type Output = ();

    fn poll(
        // Q: Pin が分からん
        // 所有権が変わるとアドレスが変わるとは？
        // https://tech-blog.optim.co.jp/entry/2020/03/05/160000
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        match (*self).state {
            StateHello::HELLO => {
                print!("Hello, ");
                (*self).state = StateHello::WORLD;
                cx.waker().wake_by_ref();
                Poll::Pending
            }
            StateHello::WORLD => {
                print!("World!!");
                (*self).state = StateHello::END;
                cx.waker().wake_by_ref();
                Poll::Pending
            }
            StateHello::END => Poll::Ready(()),
        }
    }
}
fn main() {
    let exector = Execcutor::new();
    // Q: このtask, exector, spawnerモデルってFutureを使うことが必須なの？
    exector.get_spawner().spawn(Hello::new());
    exector.run();
}
