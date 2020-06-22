use {
    super::utils::*,
    std::collections::{LinkedList, BTreeMap},
    std::io::Error as StdError,
    std::sync::Arc,
    async_lock::{Lock, LockGuard},
    async_mutex::Mutex,
    futures::prelude::*,
};

pub struct Switch {
    // TODO: use a better name
    create_listener: LinkedList<Box<dyn Fn(Vec<u8>) -> Option<DynStream>>>,    
    
    // TODO: use a better name
    create_connector: LinkedList<Box<dyn Fn(Vec<u8>) -> Option<DynFuture>>>,
    
    listeners: Mutex<BTreeMap<Vec<u8>, LockGuard<()>>>,
}


impl Switch {
    pub fn new() -> Self {
        Switch {
            create_listener: LinkedList::new(),
            create_connector: LinkedList::new(),
            listeners: Mutex::new(BTreeMap::new()),
        }
    }
    
    // TODO: use a better name
    pub fn create_listener<F, St, Fut, Con, StEr, FutEr>(&mut self, f: F) 
    where 
        F: Fn(Vec<u8>) -> Option<St> + Send + 'static,
        St: Stream<Item=Result<Fut, StEr>> + Send + 'static,
        Fut: Future<Output=Result<Con, FutEr>> + Send + 'static,
        Con: AsyncRead + AsyncWrite + Send + 'static,
        StEr: Into<StdError>,
        FutEr: Into<StdError>,
    {
        self.create_listener
            .push_back(Box::new(move |addr| {
                f(addr).map(|st| {
                    st.map_ok(|fut| {
                        fut.map_ok(|con| {
                            Box::pin(con) as DynIO
                        }).map_err(|e| e.into()).boxed()
                    }).map_err(|e| e.into()).boxed()
                    
                })
            }))
    }
    
    // TODO: use a better name   
    pub fn create_connector<F, Fut, Con, FutEr>(&mut self, f: F) 
    where
        F: Fn(Vec<u8>) -> Option<Fut> + Send +'static,
        Fut: Future<Output=Result<Con, FutEr>> + Send + 'static,
        Con: AsyncRead + AsyncWrite + Send + 'static,
        FutEr: Into<StdError>,
    {
        self.create_connector
            .push_back(Box::new(move |addr| {
                f(addr).map(|fut|{
                    fut.map_ok(|con| {
                        Box::pin(con) as DynIO
                    }).map_err(|e| e.into()).boxed()
                })
            }))
    }

    pub fn build(self) -> Arc<Self> {
        Arc::new(self)
    }
    pub async fn listen(self: Arc<Self>, addr: Vec<u8>) {
        if let Some(listener) = self.create_listener.iter().find_map(|f| {
            f(addr.clone()) 
        }) {
            let lock = Lock::new(());
            let guard = lock.lock().await;
            self.listeners.lock().await.insert(addr, guard);
            unimplemented!() 
        }
    }
}
