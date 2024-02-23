use serde::{Deserialize, Serialize};
use std::str::FromStr;

use kinode_process_lib::{
    await_message, call_init, println, Address, Message, ProcessId, Request, Response,
};

mod tg_api;
use tg_api::{Api, GetUpdatesParams, Update};

wit_bindgen::generate!({
    path: "wit",
    world: "process",
    exports: {
        world: Component,
    },
});




fn handle_message(our: &Address) -> anyhow::Result<()> {
    let message = await_message()?;

    match message {
        Message::Response { .. } => {
            return Err(anyhow::anyhow!("unexpected Response: {:?}", message));
        }
        Message::Request {
            ref source,
            ref body,
            ..
        } => match serde_json::from_slice(body)? {
            TgUpdate { updates } => {
                println!("got updates..");
            }
        },
    }
    Ok(())
}

call_init!(init);

fn init(our: Address) {
    println!("hellobot: to begin, give me a token!");

    loop {
        match handle_message(&our) {
            Ok(()) => {}
            Err(e) => {
                println!("hellobot: error: {:?}", e);
            }
        };
    }
}
