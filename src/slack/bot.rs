use log::*;
use slack::{Event, EventHandler, Message, RtmClient};

pub struct Handler;

impl EventHandler for Handler {
    fn on_event(&mut self, cli: &RtmClient, event: Event) {
        trace!("Got event {:?}", event);
        match event.clone() {
            Event::Message(message) => self.handle_message(*message, cli, &event),
            _ => return
        }
    }

    fn on_close(&mut self, cli: &RtmClient) {
        info!("Closed connection")
    }

    fn on_connect(&mut self, cli: &RtmClient) {
        info!("Opened connection")
    }
}

impl Handler {
    fn handle_message(&mut self, message: Message, cli: &RtmClient, event: &Event) {
        match message {
            Message::Standard(message) =>
                trace!("Got message {} in {} from {}", message.text.unwrap_or_default(), message.channel.unwrap_or_default(), message.user.unwrap_or_default()),
            _ => return
        }
    }
}