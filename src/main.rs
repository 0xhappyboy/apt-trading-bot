mod client;
mod global;
mod transform;
mod wallet;
use client::*;
use global::*;
use teloxide::prelude::*;
use transform::*;
use wallet::*;

#[tokio::main]
async fn main() {
    // pretty_env_logger::init();
    // log::info!("Starting throw dice bot...");
    // let bot = Bot::from_env();
    // teloxide::repl(bot, |bot: Bot, msg: Message| async move {
    //     bot.send_dice(msg.chat.id).await?;
    //     Ok(())
    // })
    // .await;
}
