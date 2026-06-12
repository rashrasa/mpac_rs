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
    Self: Clone + Send,
{
    fn send(&self, data: T) -> Result<(), SendError<T>>;
}

pub trait BlockingReceive<T>
where
    Self: Clone,
{
    fn recv(&self) -> Result<T, RecvError>;
}

#[cfg(feature = "bench")]
pub trait ChannelMaker {
    fn channel<T>(
        &self,
    ) -> (
        impl BBlockingSend<T> + Send + 'static,
        impl BBlockingReceive<T> + Send + 'static,
    )
    where
        Self: Sized,
        T: Send + 'static;
}

#[cfg(feature = "bench")]
#[derive(Debug)]
pub enum BSendError<T> {
    Closed((T, usize)),
}
#[cfg(feature = "bench")]
#[derive(Debug)]
pub enum BRecvError {
    Closed(usize),
}

#[cfg(feature = "bench")]
pub trait BBlockingSend<T>
where
    Self: Clone + BlockingSend<T>,
{
    fn b_send(&self, data: T) -> Result<usize, BSendError<T>>;
}
#[cfg(feature = "bench")]
pub trait BBlockingReceive<T>
where
    Self: Clone + BlockingReceive<T>,
{
    fn b_recv(&self) -> Result<(T, usize), BRecvError>;
}

#[cfg(not(feature = "bench"))]
mod v1;

#[cfg(feature = "bench")]
pub mod v1;

#[cfg(not(feature = "bench"))]
mod v2;

#[cfg(feature = "bench")]
pub mod v2;

#[cfg(not(feature = "bench"))]
mod v3;

#[cfg(feature = "bench")]
pub mod v3;

pub use v2::*;
