mod handle;
mod http;
mod types;
use http::Bot;
use tokio::sync::mpsc;
use types::OneBotRequest;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(100);
    let mut bot = Bot::new("192.168.3.5", 3000, "1234567890", false, tx).await;
    let bot_clone = bot.clone();
    tokio::spawn(async move { bot.start().await });
    while let Some(msg) = rx.recv().await {
        handle::handle(msg, bot_clone.user_id, |action: OneBotRequest| async {
            let result = bot_clone.call_api(action).await;
            return result;
        })
        .await;
    }
}
