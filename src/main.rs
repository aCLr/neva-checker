use crate::result::Result;

mod bot;
mod client;
mod result;
mod structs;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    let login = std::env::var("NEVA_LOGIN").expect("NEVA_LOGIN not specified");
    let password = std::env::var("NEVA_PASSWORD").expect("NEVA_PASSWORD not specified");
    let bot = std::env::var("TG_TOKEN").expect("TG_TOKEN not specified");

    let client = client::Client::new(login, password);
    let bot_handle = bot::start_bot(bot, client);

    tokio::select! {
        sig = tokio::signal::ctrl_c() => {match sig {
            Err(e) => log::error!("failed to listen ctrl+c: {:?}", e),
            Ok(_) => log::info!("\
            ctrl+c received"),
        }},
        b = bot_handle => {
            b??
        }
    }
    Ok(())
}
