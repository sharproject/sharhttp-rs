use crate::App::{
    add_route_pub, pub_add_multiple_handler, HandleCallback, HandlerBTreeMapKeyString, HandlerType,
};

#[derive(Clone)]
pub struct RouterManager {
    AddGlobalHandler: HandlerType,
    // RemoveHandler: Vec<String>,
    RouterChange: bool,
    LocalPath: String,
    AddLocalHandler: HandlerType,
}

impl RouterManager {
    pub fn new() -> RouterManager {
        Self {
            AddGlobalHandler: HandlerType::default(),
            RouterChange: false,
            LocalPath: String::from(""),
            AddLocalHandler: HandlerType::default(),
        }
    }
    pub fn add_global_handle(
        &mut self,
        key: HandlerBTreeMapKeyString,
        handler: Vec<HandleCallback>,
    ) {
        self.RouterChange = true;
        pub_add_multiple_handler(&mut self.AddGlobalHandler, key, handler)
    }
    pub fn add_local_handle(
        &mut self,
        key: HandlerBTreeMapKeyString,
        handler: Vec<HandleCallback>,
    ) {
        self.RouterChange = true;
        pub_add_multiple_handler(&mut self.AddLocalHandler, key, handler)
    }

    pub fn setLocalPath(&mut self, path: String) {
        self.LocalPath = path.to_string()
    }
    pub fn ProcessingRouter(&self, handler: &mut HandlerType) {
        add_route_pub(handler, "/".to_string(), &self.AddGlobalHandler);
        add_route_pub(handler, self.LocalPath.clone(), &self.AddLocalHandler);
    }
}
