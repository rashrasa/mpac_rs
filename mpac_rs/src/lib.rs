#[derive(Debug)]
pub enum SendError<T> {
    Closed(T),
}

#[derive(Debug)]
pub enum RecvError {
    Closed,
}

pub trait BlockingSend<T>
where
    Self: Clone,
{
    fn send(&self, data: T) -> Result<(), SendError<T>>;
    fn len(&self) -> usize;
}

pub trait BlockingReceive<T>
where
    Self: Clone,
{
    fn recv(&self) -> Result<T, RecvError>;
    fn len(&self) -> usize;
}

#[cfg(feature = "bench")]
pub trait ChannelMaker {
    fn channel<T>(&self) -> (Sender<T>, Receiver<T>)
    where
        Self: Sized;
}

#[cfg(not(feature = "bench"))]
mod v1;

#[cfg(feature = "bench")]
pub mod v1;

#[cfg(feature = "v1")]
pub use v1::*;
