use crate::tp8::protocol::{Message, Response}; 
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

type UserMap = Arc<Mutex<HashMap<String, TcpStream>>>;

pub async fn run_server() {
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    println!("Serveur de messagerie en écoute sur 127.0.0.1:8080");
    
    let users: UserMap = Arc::new(Mutex::new(HashMap::new()));
    
    loop {
        let (socket, addr) = listener.accept().await.unwrap();
        println!("Nouvelle connexion: {}", addr);
        
        let users_clone = users.clone();
        tokio::spawn(async move {
            handle_client(socket, users_clone).await;
        });
    }
}

async fn handle_client(mut socket: TcpStream, users: UserMap) {
    let mut buffer = [0; 1024];
    let mut username = String::new();
    
    loop {
        match socket.read(&mut buffer).await {
            Ok(0) => break, // Connexion fermée
            Ok(n) => {
                let data = String::from_utf8_lossy(&buffer[..n]);
                let message_str = data.trim();
                
                println!("Message reçu: {}", message_str);
                
                match Message::deserialize(message_str) {
                    Ok(message) => {
                        let response = process_message(message, &mut username, &users).await;
                        let response_str = format!("{}\n", response.serialize());
                        
                        if socket.write_all(response_str.as_bytes()).await.is_err() {
                            break;
                        }
                    }
                    Err(e) => {
                        let error_response = Response::Error { message: e };
                        let response_str = format!("{}\n", error_response.serialize());
                        let _ = socket.write_all(response_str.as_bytes()).await;
                    }
                }
            }
            Err(_) => break,
        }
    }
    
    // Nettoyer à la déconnexion
    if !username.is_empty() {
        users.lock().unwrap().remove(&username);
        println!("Utilisateur {} déconnecté", username);
    }
}

async fn process_message(message: Message, username: &mut String, users: &UserMap) -> Response {
    match message {
        Message::Connect { username: user } => {
            if users.lock().unwrap().contains_key(&user) {
                Response::Error { message: "Nom d'utilisateur déjà pris".to_string() }
            } else {
                *username = user.clone();
                println!("Utilisateur {} connecté", user);
                Response::Ok { message: format!("Bienvenue {}", user) }
            }
        }
        
        Message::List => {
            let user_list: Vec<String> = users.lock().unwrap().keys().cloned().collect();
            Response::UserList { users: user_list }
        }
        
        Message::Send { to, content } => {
            if username.is_empty() {
                return Response::Error { message: "Vous devez vous connecter d'abord".to_string() };
            }
            
            let users_lock = users.lock().unwrap();
            if users_lock.contains_key(&to) {
                // Dans un vrai serveur, on enverrait le message au destinataire
                // Ici on simule juste
                Response::Ok { message: format!("Message envoyé à {}", to) }
            } else {
                Response::Error { message: "Utilisateur introuvable".to_string() }
            }
        }
        
        Message::Disconnect => {
            if !username.is_empty() {
                users.lock().unwrap().remove(username);
                Response::Ok { message: "Au revoir !".to_string() }
            } else {
                Response::Error { message: "Vous n'êtes pas connecté".to_string() }
            }
        }
    }
}