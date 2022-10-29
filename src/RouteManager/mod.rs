use crate::App::{
    add_route_pub, pub_add_multiple_handler, HandleCallback,  HandlerBTreeMapKeyString,
    HandlerType,
};

#[derive(Clone)]
pub struct RouterManager {
    AddGlobalHandler: HandlerType,
    // RemoveHandler: Vec<String>,
    RouterChange: bool,
}

impl RouterManager {
    pub fn new() -> RouterManager {
        Self {
            AddGlobalHandler: HandlerType::default(),
            RouterChange: false,
        }
    }
    pub fn add_global_handle(
        &mut self,
        key: HandlerBTreeMapKeyString,
        handler: Vec<HandleCallback>,
    ) {
        self.RouterChange = true;
        pub_add_multiple_handler(
            &mut self.AddGlobalHandler,
            key.clone(),
            handler
        )
    }
    pub fn ProcessingRouter(&self, handler: &mut HandlerType) {
        add_route_pub(handler, "/".to_string(), &self.AddGlobalHandler);
    }
}
