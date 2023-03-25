use tokio::sync::{mpsc, oneshot};
use tokio::sync::mpsc::Sender;

pub trait Handler<M: Send + 'static> {
    type Result: Send + 'static;

    fn handle(&mut self, message: M) -> Self::Result;
}

type Mail<T> = Box<dyn FnOnce(&mut T) + Send>;

#[derive(Clone)]
pub struct Address<T> {
    sender: Sender<Mail<T>>,
}

impl<T> Address<T> {
    pub async fn send<M: Send + 'static>(&self, message: M) -> <T as Handler<M>>::Result
    where
        T: Handler<M>,
    {
        let (response_sender, response_receiver) = oneshot::channel();

        let mail: Mail<T> = Box::new(move |t| {
            let result = T::handle(t, message);
            response_sender.send(result).unwrap_or_default() // just ignore when the caller lost interest in the result
        });

        self.sender.send(mail).await
            .or(Err("the actor panicked")).unwrap();

        response_receiver.await.expect("the actor panicked")
    }
}

pub fn spawn<T: Send + 'static>(mut t: T, buffer: usize) -> Address<T> {
    let (sender, mut receiver) = mpsc::channel(buffer);

    let addr = Address { sender };

    tokio::spawn(async move {
        while let Some(f) = receiver.recv().await {
            f(&mut t);
        }
    });

    addr
}
