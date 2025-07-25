use tokio_tungstenite::connect_async;
use futures_util::{StreamExt, SinkExt};
use url::Url;

pub async fn run_client() {
    let url = Url::parse("ws://127.0.0.1:9001").unwrap();
    let (mut ws_stream, _) = connect_async(url.as_str()).await.unwrap();
    println!("Connecté au serveur WebSocket");

    ws_stream.send("Bonjour serveur".into()).await.unwrap();

    if let Some(Ok(msg)) = ws_stream.next().await {
        println!("Réponse du serveur: {}", msg);
    }
}