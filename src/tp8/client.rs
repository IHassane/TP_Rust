use crate::tp8::protocol::{Message, Response}; 
use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub async fn run_client() {
    let mut socket = TcpStream::connect("127.0.0.1:8080").await.unwrap();
    println!("Connecté au serveur");
    
    // Se connecter avec un nom d'utilisateur
    let connect_msg = Message::Connect { username: "Alice".to_string() };
    send_message(&mut socket, &connect_msg).await;
    read_response(&mut socket).await;
    
    // Lister les utilisateurs
    let list_msg = Message::List;
    send_message(&mut socket, &list_msg).await;
    read_response(&mut socket).await;
    
    // Essayer d'envoyer un message
    let send_msg = Message::Send { 
        to: "Bob".to_string(), 
        content: "Salut Bob !".to_string() 
    };
    send_message(&mut socket, &send_msg).await;
    read_response(&mut socket).await;
    
    // Se déconnecter
    let disconnect_msg = Message::Disconnect;
    send_message(&mut socket, &disconnect_msg).await;
    read_response(&mut socket).await;
}

async fn send_message(socket: &mut TcpStream, message: &Message) {
    let data = format!("{}\n", message.serialize());
    socket.write_all(data.as_bytes()).await.unwrap();
    println!("Envoyé: {:?}", message);
}

async fn read_response(socket: &mut TcpStream) {
    let mut buffer = [0; 1024];
    let n = socket.read(&mut buffer).await.unwrap();
    let data = String::from_utf8_lossy(&buffer[..n]);
    
    match Response::deserialize(data.trim()) {
        Ok(response) => println!("Reçu: {:?}", response),
        Err(e) => println!("Erreur de parsing: {}", e),
    }
}