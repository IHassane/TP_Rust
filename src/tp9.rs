mod server;
mod client;

pub async fn websocket() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() >= 2 {
        match args[1].as_str() {
            "server" => server::run_server().await,
            "client" => client::run_client().await,
            _ => eprintln!("Utilisation: cargo run -- [server|client]"),
        }
    } else {
        eprintln!("Utilisation: cargo run -- [server|client]");
    }
}