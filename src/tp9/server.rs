use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;
use futures_util::{StreamExt, SinkExt};

pub async fn run_server() {
    let listener = TcpListener::bind("127.0.0.1:9001").await.unwrap();
    println!("Serveur WebSocket en attente sur ws://127.0.0.1:9001");

    while let Ok((stream, addr)) = listener.accept().await {
        println!("Nouvelle connexion: {}", addr);

        tokio::spawn(async move {
            let ws_stream = accept_async(stream).await.expect("Erreur lors du handshake WebSocket");
            println!("Handshake WebSocket réussi: {}", addr);
            let (mut write, mut read) = ws_stream.split();

            while let Some(msg) = read.next().await {
                match msg {
                    Ok(msg) => {
                        println!("Reçu: {}", msg);
                        if msg.is_text() {
                            let response = format!("Echo: {}", msg.to_text().unwrap());
                            write.send(response.into()).await.unwrap();
                        }
                    }
                    Err(e) => {
                        eprintln!("Erreur lors de la lecture: {}", e);
                        break;
                    }
                }
            }

            println!("Client déconnecté: {}", addr);
        });
    }
}
