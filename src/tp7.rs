mod client;
mod server;

pub async fn dns() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: dns_simple [client|server]");
        return;
    }

    match args[1].as_str() {
        "client" => client::run_client().await,
        "server" => server::run_server().await,
        _ => eprintln!("Unknown command: {}", args[1]),
    }
}