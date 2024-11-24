use crate::types::{
    AtMessage, GetGroupInfo, GetGroupInfoResponse, OneBotRequest, SendMessage, SendMessageData,
    SendMessageMessage,
};
use egui_messagebox::{input, load_icon_from_url, DialogParams};
use std::future::Future;

pub async fn handle<F, Fut>(message: String, self_id: i64, onebot_api_caller: F)
where
    F: Fn(OneBotRequest) -> Fut,
    Fut: Future<Output = String>,
{
    let event: AtMessage = match serde_json::from_str(&message) {
        Ok(v) => v,
        Err(_e) => {
            return;
        }
    };
    let group_id = event.group_id;
    for msg in event
        .message
        .iter()
        .filter(|msg| msg.message_type == Some("at".to_string()))
    {
        let msg_data = match &msg.data {
            Some(data) => data,
            _ => continue,
        };
        let at_qq = match &msg_data.qq {
            Some(qq) => qq.clone(),
            _ => continue,
        };
        if at_qq != self_id.to_string() {
            continue;
        }

        let group_name = match serde_json::from_str::<GetGroupInfoResponse>(
            &onebot_api_caller(OneBotRequest {
                action: "get_group_info".to_string(),
                data: serde_json::to_string(&GetGroupInfo {
                    group_id: group_id,
                    no_cache: true,
                })
                .unwrap(),
            })
            .await,
        ) {
            Ok(v) => v.data.group_name,
            Err(_e) => "未知群聊".to_string(),
        };

        let message = format!("{}/{}", group_name, event.sender.user_id);
        let is_private = event.message_type == "private".to_string();
        let icon_url = format!(
            "https://q.qlogo.cn/headimg_dl?dst_uin={}&spec=100",
            event.sender.user_id
        );
        let icon_data = load_icon_from_url(&icon_url).await;
        let dialog_params = DialogParams::create(
            Some(icon_data),
            Some("有人@你了".to_string()),
            Some(message),
            None,
            Some("确定".to_string()),
            Some("取消".to_string()),
        );
        let result = input(dialog_params).await;
        if !result.is_empty() {
            let msg = SendMessage {
                group_id: if is_private {
                    None
                } else {
                    Some(group_id.to_string())
                },
                message: vec![SendMessageMessage {
                    data: Some(SendMessageData { text: result }),
                    message_type: Some("text".to_string()),
                }],
                message_type: if is_private {
                    "private".to_string()
                } else {
                    "group".to_string()
                },
                user_id: if is_private {
                    Some(event.sender.user_id.to_string())
                } else {
                    None
                },
            };
            let _ = onebot_api_caller(OneBotRequest {
                action: "send_msg".to_string(),
                data: serde_json::to_string(&msg).unwrap(),
            })
            .await;
        }
    }
}
