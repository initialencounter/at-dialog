use reqwest::header::CONTENT_TYPE;
use reqwest::Client;
use tokio::sync::mpsc;
use warp::Filter;

use crate::types::{GetLoginInfoResponse, OneBotRequest};

#[derive(Clone)]
pub struct Bot {
    pub ip: String,
    pub port: u16,
    pub access_token: String,
    pub secure: bool,
    pub msg_tx: mpsc::Sender<String>,
    pub user_id: i64,
}
impl Bot {
    pub async fn new(
        ip: &str,
        port: u16,
        access_token: &str,
        secure: bool,
        msg_tx: mpsc::Sender<String>,
    ) -> Self {
        let login_info = call_onebot_api(
            ip,
            port,
            access_token,
            secure,
            OneBotRequest {
                action: "get_login_info".to_string(),
                data: "{\"action\":\"get_login_info\"}".to_string(),
            },
        )
        .await;
        let user_id = serde_json::from_str::<GetLoginInfoResponse>(&login_info)
            .unwrap()
            .data
            .user_id;
        Self {
            ip: ip.to_string(),
            port,
            access_token: access_token.to_string(),
            secure,
            msg_tx,
            user_id,
        }
    }
    pub async fn start(&mut self) {
        self.webhook().await;
    }
    pub async fn webhook(&mut self) {
        let routes = warp::post()
            .and(warp::path("webhook"))
            .and(warp::body::json())
            .and(warp::any().map({
                let tx = self.msg_tx.clone();
                move || tx.clone()
            }))
            .then(
                move |msg: serde_json::Value, tx: mpsc::Sender<String>| async move {
                    tx.send(msg.to_string()).await.unwrap();
                    warp::reply::json(&"ok")
                },
            );
        warp::serve(routes).run(([0, 0, 0, 0], 25458)).await;
    }
    pub async fn call_api(&self, message: OneBotRequest) -> String {
        call_onebot_api(
            &self.ip,
            self.port,
            &self.access_token,
            self.secure,
            message,
        )
        .await
    }
}

pub async fn call_onebot_api(
    ip: &str,
    port: u16,
    access_token: &str,
    secure: bool,
    message: OneBotRequest,
) -> String {
    let client = Client::new();
    // 使用 reqwest 发送消息
    let action = message.action;
    let data = message.data;
    let url = format!(
        "{}://{}:{}/{}",
        if secure { "https" } else { "http" },
        ip,
        port,
        action
    );
    let response = client
        .post(&url)
        .header(CONTENT_TYPE, "application/json; charset=utf-8")
        .bearer_auth(access_token)
        .body(data)
        .send()
        .await;

    match response {
        Ok(res) => res.text().await.unwrap(),
        Err(e) => {
            format!("发送消息失败: {}", e)
        }
    }
}
