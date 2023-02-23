use tokio::sync::{mpsc, oneshot};
use tokio::sync::mpsc::UnboundedSender;

pub trait Handler<M> {
    type Result: Send;

    fn handle(&mut self, message: M) -> Self::Result;
}

type Mail<T> = Box<dyn FnOnce(&mut T) + Send>;

pub struct Address<T> {
    sender: UnboundedSender<Mail<T>>,
}

impl<T: Send + 'static> Address<T> {
    pub async fn send<M: Send + 'static>(&self, message: M) -> T::Result
    where
        T: Handler<M>,
    {
        let (sender, receiver) = oneshot::channel();

        self.sender
            .send(Box::new(move |t| {
                let result = T::handle(t, message);
                sender.send(result).unwrap_or_default() // just ignore when the caller lost interest in the result
            }))
            .or(Err("the actor panicked")).unwrap();

        receiver.await.expect("the actor panicked")
    }
}

pub fn spawn<T: Send + 'static>(mut t: T) -> Address<T> {
    let (sender, mut receiver) = mpsc::unbounded_channel();

    let addr = Address { sender };

    tokio::spawn(async move {
        while let Some(f) = receiver.recv().await {
            f(&mut t);
        }
    });

    addr
}
