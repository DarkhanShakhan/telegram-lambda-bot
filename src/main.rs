use std::env;

use lambda_http::{run, service_fn, Body, Error, Request, Response};
use teloxide::{
    requests::Requester,
    types::{ChatId, Message, Update},
    Bot,
};

async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    let from_chat_id = env::var("FROM_CHAT_ID")?.parse::<i64>()?;
    let to_chat_id = env::var("TO_CHAT_ID")?.parse::<i64>()?;
    let trigger_phrase = env::var("TRIGGER_PHRASE")?;
    let token = env::var("TOKEN")?;
    let bot = Bot::new(token);
    // println!("request: {:?}", event.body());
    let update = convert_input_to_json(event).await?;
    // println!("message: {:?}", update);
    match update.kind {
        teloxide::types::UpdateKind::Message(msg) => {
            let current_chat_id = msg.chat.id;
            println!("update type is message");
            println!("message from {current_chat_id}");
            println!("message: {}", msg.text().unwrap_or("undefined"));
            if current_chat_id.0 == from_chat_id && has_trigger_phrases(&msg, &trigger_phrase) {
                bot.send_message(
                    ChatId(to_chat_id),
                    msg.text().unwrap_or("some notification"),
                )
                .await?;
                println!("sent message to {to_chat_id}")
            }
        }
        teloxide::types::UpdateKind::ChannelPost(msg) => {
            let current_chat_id = msg.chat.id;
            println!("update type is channel post");
            println!("message from {current_chat_id}");
            println!("message: {}", msg.text().unwrap_or("undefined"));
            if current_chat_id.0 == from_chat_id && has_trigger_phrases(&msg, &trigger_phrase) {
                bot.send_message(
                    ChatId(to_chat_id),
                    msg.text().unwrap_or("some notification"),
                )
                .await?;
                println!("sent message to {to_chat_id}")
            }
        }
        _ => {}
    }
    let resp = Response::builder().status(200).body(Body::Empty)?;
    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .without_time()
        .init();

    run(service_fn(function_handler)).await
}

async fn convert_input_to_json(input: Request) -> Result<Update, Error> {
    if let Body::Text(body_str) = input.body() {
        let body_json: Update = serde_json::from_str(body_str)?;
        return Ok(body_json);
    }
    Err(String::from("is not text").into())
}

fn has_trigger_phrases(msg: &Message, trigger_phrase: &str) -> bool {
    msg.text()
        .unwrap_or_default()
        .to_lowercase()
        .contains(&trigger_phrase.to_lowercase())
}
