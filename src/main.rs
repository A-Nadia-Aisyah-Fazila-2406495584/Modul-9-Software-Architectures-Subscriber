use borsh::{BorshDeserialize, BorshSerialize};
use crosstown_bus::{CrosstownBus, MessageHandler, HandleError};
use std::{thread, time};

#[derive(Debug, Clone, BorshDeserialize, BorshSerialize)]
pub struct UserCreatedEventMessage {
    pub user_id: String,
    pub user_name: String
}

pub struct UserCreatedHandler;

impl MessageHandler<UserCreatedEventMessage> for UserCreatedHandler {
    fn handle(&self, message: Box<UserCreatedEventMessage>
    ) -> Result<(), HandleError> {
        let ten_millis = time::Duration::from_millis(1000);
        let _now = time::Instant::now();

        // thread::sleep(ten_millis);

        println!("In Nadia's Computer [2406495585]. Message received: {:?}", message);

        Ok(())
    }
    
    fn get_handler_action(&self) -> String {
        todo!()
    }
}

fn main() {
    let listener = CrosstownBus::new_queue_listener("amqps://fuuxnpvb:GOf3R1nK8z8x829XiycuOVFi7fjWVnMO@cougar.rmq.cloudamqp.com/fuuxnpvb".to_owned()).unwrap();
    _ = listener.listen("user_created".to_owned(), UserCreatedHandler{}, crosstown_bus::QueueProperties { auto_delete: false, durable: false, use_dead_letter: true });

    loop {

    }
}