use {
    futures::prelude::*,
    crate::utils::{
        DynStream,
        DynFuture,
        DynIO,
    },
};

pub trait Transport {
    fn listen(&self, addr: &[u8]) -> DynStream<DynFuture<DynIO>>;
    fn connect(&self, addr: &[u8]) -> DynFuture<DynIO>;
}
