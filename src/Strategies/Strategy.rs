use async_trait::async_trait;
use futures::Future;
use thirtyfour::WebDriver;
use tokio::sync::mpsc::WeakUnboundedSender;


pub trait IStrategy: Sync {
    fn init(&self);
    fn execute(&self);
    fn end(&self) {}
}



