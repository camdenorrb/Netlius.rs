use async_std::pin::Pin;
use async_std::prelude::Future;
use async_std::sync::Arc;
use async_std::task::{Context, Poll, Waker};

use crate::async_utils::holder::UnsafeHolder;

// You need to use a Holder rather than a Mutex to utilize this

pub struct Suspend {
    should_suspend: bool,
    wakers: Arc<UnsafeHolder<Vec<Waker>>>
}

impl Default for Suspend {
    fn default() -> Self {
        Suspend {
            should_suspend: false,
            wakers: Arc::new(UnsafeHolder::new(Vec::new()))
        }
    }
}

impl Suspend {

    pub fn new(should_suspend: bool) -> Suspend {
        Suspend {
            should_suspend,
            wakers: Arc::new(UnsafeHolder::new(Vec::new()))
        }
    }

    pub fn suspend(&mut self) {
        self.should_suspend = true
    }

    pub async fn unsuspend(&mut self) {

        self.should_suspend = false;

        unsafe {
            for _ in 0..self.wakers.get_mut().len() {
                self.wakers.get_mut().remove(0).wake();
            }
        }
    }

}

impl Future for Suspend {

    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match self.should_suspend {
            true  => {

                unsafe {
                    self.wakers.get_mut().push(cx.waker().clone());
                }

                Poll::Pending
            },
            false => Poll::Ready(())
        }
    }

}
