pub mod protocol;
pub mod server;
pub mod client;

pub async fn protocole() {
    println!("Démarrage du serveur...");
    
    // Lancer le serveur
    tokio::spawn(async {
        client::run_client().await;
    });
    
    // Attendre un peu que le serveur démarre
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    
    // Lancer le client
    server::run_server().await;
}