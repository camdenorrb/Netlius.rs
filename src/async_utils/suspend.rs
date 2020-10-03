use async_std::task::{Context, Poll, Waker};
use async_std::pin::Pin;
use async_std::prelude::Future;
use async_std::sync::{Arc, Mutex};
use futures::executor::block_on;


pub struct Suspend {
    should_suspend: bool,
    wakers: Arc<Mutex<Vec<Waker>>>
}

impl Default for Suspend {
    fn default() -> Self {
        Suspend {
            should_suspend: false,
            wakers: Arc::new(Mutex::new(Vec::new()))
        }
    }
}

impl Suspend {

    pub fn new(should_suspend: bool) -> Suspend {
        Suspend {
            should_suspend,
            wakers: Arc::new(Mutex::new(Vec::new()))
        }
    }

    pub fn suspend(&mut self) {
        self.should_suspend = true
    }

    pub async fn unsuspend(&mut self) {

        self.should_suspend = false;
        channel()

        println!("{}", self.wakers.lock().await.len());
        for _ in 0..self.wakers.lock().await.len() {
            self.wakers.lock().await.remove(0).wake()
        }
    }

}

impl Future for Suspend {

    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match self.should_suspend {
            true  => {
                block_on(async {
                    self.wakers.lock().await.push(cx.waker().clone());
                });
                Poll::Pending
            },
            false => Poll::Ready(())
        }
    }

}
