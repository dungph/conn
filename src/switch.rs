use {
    super::utils::*,
    std::collections::LinkedList,
    std::sync::Arc,
    async_lock::{Lock, LockGuard},
    futures::prelude::*,
};

pub struct Switch {
    new_listener: LinkedList<Box<dyn Fn(Vec<u8>) -> Option<DynStream<DynIO>>>>,
    new_connector: LinkedList<Box<dyn Fn(Vec<u8>) -> Option<DynFuture<DynIO>>>>,
    listener: Mutex<BTreeMap<Vec<u8>, LockGuard<()>>>,
}


impl Switch {
    pub fn new() -> Self {
        Switch {
            new_listener: LinkedList::new(),
            new_connector: LinkedList::new(),
        }
    }
    
    pub fn new_listener<F: 'static>(&mut self, f: F) 
    where 
        F: Fn(Vec<u8>) -> Option<DynStream<DynIO>>,
    {
        self.new_listener.push_back(Box::new(f));  
    }
    
    pub fn new_connector<F: 'static>(&mut self, f: F)
    where
        F: Fn(Vec<u8>) -> Option<DynFuture<DynIO>>,
    {
        self.new_connector.push_back(Box::new(f));
    }

    pub fn build(self) -> Arc<Self> {
        Arc::new(self)
    }

    pub async fn listen(self: Arc<Self>, addr: Vec<u8>) {
        if let Some(listener) = self.new_listener.iter().find_map(|f| {
            f(addr.clone()) 
        }) {
            let lock = Lock::new(());
            let guard = lock.lock().await;
            self.listener.lock().await.insert(addr, guard);
                 
        }
    }
}
