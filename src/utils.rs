use {
    futures::prelude::*,
    std::io::Error,
    std::{
        task::Context,
    },
};

pub type DynStream = std::pin::Pin<Box<dyn Stream<Item = Result<DynFuture, Error>> + Send>>;
pub type DynFuture = std::pin::Pin<Box<dyn Future<Output = Result<DynIO, Error>> + Send>>;
pub type DynIO = std::pin::Pin<Box<dyn IO>>;



pub trait IO: AsyncRead + AsyncWrite + Send + 'static {}
impl<T: AsyncRead + AsyncWrite + Send + 'static> IO for T {}

