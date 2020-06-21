use {
    futures::prelude::*,
    std::io::Error,
    std::{
        task::Context,
    },

};

pub type DynStream<I> = std::pin::Pin<Box<dyn Stream<Item = Result<I, Error>>>>;
pub type DynFuture<O> = std::pin::Pin<Box<dyn Future<Output = Result<O, Error>>>>;
pub type DynIo = std::pin::Pin<Box<dyn IO>>;


pub trait IO: AsyncRead + AsyncWrite {}


