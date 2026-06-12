use std::sync::{
    Arc,
    atomic::{AtomicPtr, AtomicU8, AtomicUsize, Ordering},
};

use crate::{BlockingReceive, BlockingSend, RecvError, SendError};

#[derive(Debug)]
pub struct Sender<T> {
    inner: Arc<ChannelInner<T>>,
}

#[derive(Debug)]
pub struct Receiver<T> {
    inner: Arc<ChannelInner<T>>,
}

#[derive(Debug)]
struct ChannelInner<T> {
    senders: AtomicUsize,
    receivers: AtomicUsize,
    queue: ConcurrentBlockingList<T>,
}

impl<T> BlockingReceive<T> for Receiver<T> {
    fn recv(&self) -> Result<T, RecvError> {
        todo!();
    }
}

impl<T: Send> BlockingSend<T> for Sender<T> {
    fn send(&self, data: T) -> Result<(), SendError<T>> {
        todo!();
    }
}

#[cfg(feature = "bench")]
impl<T: Send> crate::BBlockingReceive<T> for Receiver<T> {
    fn b_recv(&self) -> Result<(T, usize), crate::BRecvError> {
        todo!();
    }
}

#[cfg(feature = "bench")]
impl<T: Send> crate::BBlockingSend<T> for Sender<T> {
    fn b_send(&self, data: T) -> Result<usize, crate::BSendError<T>> {
        todo!();
    }
}

impl<T> Clone for Receiver<T> {
    fn clone(&self) -> Self {
        todo!();
    }
}

impl<T> Clone for Sender<T> {
    fn clone(&self) -> Self {
        todo!();
    }
}

impl<T> Drop for Receiver<T> {
    fn drop(&mut self) {
        todo!();
    }
}

impl<T> Drop for Sender<T> {
    fn drop(&mut self) {
        todo!();
    }
}

pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
    let inner = Arc::new(ChannelInner {
        senders: AtomicUsize::new(1),
        receivers: AtomicUsize::new(1),
        queue: ConcurrentBlockingList::new(),
    });
    (
        Sender {
            inner: inner.clone(),
        },
        Receiver { inner },
    )
}

#[cfg(feature = "bench")]
pub struct V3Maker;
#[cfg(feature = "bench")]
impl crate::ChannelMaker for V3Maker {
    fn channel<T>(
        &self,
    ) -> (
        impl crate::BBlockingSend<T> + Send + 'static,
        impl crate::BBlockingReceive<T> + Send + 'static,
    )
    where
        T: Send + 'static,
    {
        channel()
    }
}

// INVARIANT: front and back always points to a valid node or are null pointers
#[derive(Debug)]
pub struct ConcurrentBlockingList<T> {
    front: AtomicPtr<Node<T>>,
    back: AtomicPtr<Node<T>>,
}

impl<T> ConcurrentBlockingList<T> {
    pub fn new() -> Self {
        todo!();
    }

    pub fn pop_front_wait(&self) -> T {
        // 1. mark front as taken

        todo!()
    }

    // pub fn push_back(&self, data: T) {
    //     let node = Node {
    //         inner: data,
    //         next: AtomicPtr::new(null_mut()),
    //     };

    //     let a = AtomicPtr::new(Box::leak(Box::new(node)));

    //     while let Err(_) = self.back.compare_exchange(null_mut(), success, failure) {}
    // }
}

struct Front<T> {
    flag: AccessFlag,
    front: AtomicPtr<Node<T>>,
}

struct Back<T> {
    flag: AccessFlag,
    back: AtomicPtr<Node<T>>,
}

struct Node<T> {
    flag: AccessFlag,
    next: AtomicPtr<Node<T>>,
    inner: T,
}

const RELEASED: u8 = 0;
const ACCESSED: u8 = 1;
const TAKEN: u8 = 2;

struct AccessFlag {
    flag: AtomicU8,
}

impl AccessFlag {
    fn access<'a>(&'a self) -> Result<ReleaseGuard<'a>, ()> {
        match self
            .flag
            .compare_exchange(RELEASED, ACCESSED, Ordering::AcqRel, Ordering::Acquire)
        {
            Ok(_) => Ok(ReleaseGuard { inner: &self.flag }),
            Err(f) => {
                if f == ACCESSED {
                    Err(())
                } else if f == TAKEN {
                    // taken values should be guarded
                    // in this case, they represent a node which is about to be dropped
                    // which includes this access flag
                    unreachable!("reading an AccessFlag after it was taken");
                } else {
                    unreachable!("impossible or unhandled flag {}", f);
                }
            }
        }
    }

    fn take(&self) -> Result<(), ()> {
        match self
            .flag
            .compare_exchange(RELEASED, TAKEN, Ordering::AcqRel, Ordering::Acquire)
        {
            Ok(_) => Ok(()),
            Err(f) => {
                if f == ACCESSED {
                    Err(())
                } else if f == TAKEN {
                    // taken values should be guarded
                    // in this case, they represent a node which is about to be dropped
                    // which includes this access flag
                    unreachable!("reading an AccessFlag after it was taken");
                } else {
                    unreachable!("impossible or unhandled flag {}", f);
                }
            }
        }
    }
}

pub struct ReleaseGuard<'a> {
    inner: &'a AtomicU8,
}

impl<'a> ReleaseGuard<'a> {
    fn release(&self) {
        match self
            .inner
            .compare_exchange(ACCESSED, RELEASED, Ordering::AcqRel, Ordering::Acquire)
        {
            Ok(_) => {}
            Err(f) => {
                unreachable!("could not release an accessed resource: flag was {}", f);
            }
        }
    }
}

impl<'a> Drop for ReleaseGuard<'a> {
    fn drop(&mut self) {
        self.release();
    }
}
