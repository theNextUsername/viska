mod dialogs;
pub mod elements;

pub use dialogs::Dialogs;

use common::async_trait::async_trait;
use std::fmt::Debug;

use models::transport::{RequestMsg, ResponseMsg, TransportMsg};

#[async_trait]
pub trait TuProcessor: Send + Sync + Debug + 'static {
    async fn process_incoming_message(&self, msg: TransportMsg) -> Result<(), crate::Error>;
}

#[async_trait]
pub trait ReqProcessor: Send + Sync + Debug + 'static {
    async fn process_incoming_request(&self, msg: RequestMsg) -> Result<(), crate::Error>;
}

#[async_trait]
pub trait RespProcessor: Send + Sync + Debug + 'static {
    async fn process_incoming_response(&self, msg: ResponseMsg) -> Result<(), crate::Error>;
}

/*
#[async_trait]
pub trait DialogsProcessor: Send + Sync + Any + Debug {
    fn new(sip_manager: Weak<SipManager>) -> Self
    where
        Self: Sized;
    async fn has_dialog(&self, dialog_id: &str) -> bool;
    async fn new_uac_dialog(&self, msg: RequestMsg) -> Result<(), crate::Error>;
    async fn new_uas_dialog(&self, msg: RequestMsg) -> Result<(), crate::Error>;
    async fn process_incoming_message(&self, msg: RequestMsg) -> Result<(), crate::Error>;
    fn as_any(&self) -> &dyn Any;
}*/

//#[async_trait]
//pub trait ProxyProcessor: Send + Sync + Any + Debug {
//    fn new(sip_manager: Weak<SipManager>) -> Self
//    where
//        Self: Sized;
//    async fn validate_request(&self, msg: RequestMsg) -> Result<(), crate::Error>;
//    async fn preprocess_routing_info(&self, msg: RequestMsg) -> Result<(), crate::Error>;
//    async fn determine_targets(&self, msg: RequestMsg) -> Result<(), crate::Error>;
//    async fn forward_request(&self, msg: RequestMsg) -> Result<(), crate::Error>;
//    async fn process_response(&self, msg: ResponseMsg) -> Result<(), crate::Error>;
//    fn as_any(&self) -> &dyn Any;
//}
