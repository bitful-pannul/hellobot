use frankenstein::{
    ChatId, SendMessageParams, TelegramApi,
    UpdateContent::{ChatJoinRequest, Message as TgMessage},
};
use kinode_process_lib::{await_message, call_init, println, Address, Message};
mod tg_api;
use tg_api::{init_tg_bot, Api, TgResponse};

wit_bindgen::generate!({
    path: "wit",
    world: "process",
    exports: {
        world: Component,
    },
});

fn handle_message(_our: &Address, api: &Api, worker: &Address) -> anyhow::Result<()> {
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
            TgResponse::Update(tg_update) => {
                let updates = tg_update.updates;
                // assert update is from our worker
                if source != worker {
                    return Err(anyhow::anyhow!(
                        "unexpected source: {:?}, expected: {:?}",
                        source,
                        worker
                    ));
                }

                if let Some(update) = updates.last() {
                    match &update.content {
                        TgMessage(msg) => {
                            // get msg contents, and branch on what to send back!
                            let text = msg.text.clone().unwrap_or_default();

                            // fill in default send_message params, switch content later!
                            let mut params = SendMessageParams {
                                chat_id: ChatId::Integer(msg.chat.id),
                                disable_notification: None,
                                entities: None,
                                link_preview_options: None,
                                message_thread_id: None,
                                parse_mode: None,
                                text: "temp".to_string(),
                                protect_content: None,
                                reply_markup: None,
                                reply_parameters: None,
                                // todo, maybe change api so can ..Default::default()?
                            };

                            match text.as_str() {
                                "/hello" => {
                                    params.text = "good morning!".to_string();
                                    api.send_message(&params)?;
                                }
                                "/goodbye" => {
                                    params.text = "it's over for u".to_string();
                                    api.send_message(&params)?;
                                }
                                "/wen???" => {
                                    params.text = "soon™".to_string();
                                    api.send_message(&params)?;
                                }
                                _ => {
                                    params.text = "me no undersand".to_string();
                                    api.send_message(&params)?;
                                }
                            }
                        }
                        ChatJoinRequest(req) => {
                            println!("got chat join request from: {:?}", req.from);
                        }
                        _ => {
                            println!("got unhandled tg update: {:?}", update);
                        }
                    }
                }
            }
            TgResponse::Error(e) => {
                println!("error from tg worker: {:?}", e);
            }
        },
    }
    Ok(())
}

call_init!(init);

fn init(our: Address) {
    println!("hellobot: to begin, give me a token!");

    let message = await_message().unwrap();
    let token_str = String::from_utf8(message.body().to_vec()).unwrap_or_else(|_| "".to_string());

    let (api, worker) = init_tg_bot(our.clone(), &token_str, None).unwrap();

    loop {
        match handle_message(&our, &api, &worker) {
            Ok(()) => {}
            Err(e) => {
                println!("hellobot: error: {:?}", e);
            }
        };
    }
}
