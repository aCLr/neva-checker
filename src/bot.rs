use crate::client::Client;
use crate::result::Result;
use futures::StreamExt;
use telegram_bot::*;
use tokio::task::JoinHandle;

pub fn start_bot<S: AsRef<str>>(token: S, client: Client) -> JoinHandle<Result<()>> {
    let api = Api::new(token);
    tokio::spawn(async move {
        log::info!("bot started");
        let mut stream = api.stream();
        while let Some(update) = stream.next().await {
            log::info!("get new update: {:?}", update);
            let update = update?;
            if let UpdateKind::Message(message) = update.kind {
                if let MessageKind::Text { ref data, .. } = message.kind {
                    let text = match data.as_str() {
                        "/last_trip" => match client.get_last_trip().await {
                            Ok(resp) => match resp {
                                None => "can't get last trip".to_string(),
                                Some(trip) => {
                                    format!(
                                        "date={date}\n\
                                        amount={amount}",
                                        amount = trip.amount(),
                                        date = trip.dt()
                                    )
                                }
                            },
                            Err(err) => format!("{}", err),
                        },
                        "/balance" => match client.get_balance().await {
                            Ok(resp) => match resp {
                                None => "can't get balance".to_string(),
                                Some(balance) => {
                                    format!("balance={}", balance.remainder())
                                }
                            },
                            Err(err) => format!("{}", err),
                        },
                        _ => format!("unknown command: {}", data),
                    };
                    log::info!("sending text: {}", text);
                    if let Err(err) = api.send(message.text_reply(text)).await {
                        log::error!("can't send message to bot: {}", err);
                    };
                }
            }
        }
        Ok(())
    })
}
