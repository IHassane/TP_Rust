#[derive(Debug, Clone)]
pub enum Message {
    Connect { username: String },           // Se connecter avec un nom
    Send { to: String, content: String },   // Envoyer un message
    List,                                   // Lister les utilisateurs connectés
    Disconnect,                             // Se déconnecter
}

#[derive(Debug, Clone)]
pub enum Response {
    Ok { message: String },                 // Succès
    Error { message: String },              // Erreur
    UserList { users: Vec<String> },        // Liste des utilisateurs
    NewMessage { from: String, content: String }, // Nouveau message reçu
}

// Sérialisation très simple (format: TYPE:DATA)
impl Message {
    pub fn serialize(&self) -> String {
        match self {
            Message::Connect { username } => format!("CONNECT:{}", username),
            Message::Send { to, content } => format!("SEND:{}:{}", to, content),
            Message::List => "LIST:".to_string(),
            Message::Disconnect => "DISCONNECT:".to_string(),
        }
    }

    pub fn deserialize(data: &str) -> Result<Self, String> {
        let parts: Vec<&str> = data.splitn(3, ':').collect();
        match parts[0] {
            "CONNECT" => Ok(Message::Connect { username: parts[1].to_string() }),
            "SEND" => Ok(Message::Send { 
                to: parts[1].to_string(), 
                content: parts[2].to_string() 
            }),
            "LIST" => Ok(Message::List),
            "DISCONNECT" => Ok(Message::Disconnect),
            _ => Err("Message invalide".to_string()),
        }
    }
}

impl Response {
    pub fn serialize(&self) -> String {
        match self {
            Response::Ok { message } => format!("OK:{}", message),
            Response::Error { message } => format!("ERROR:{}", message),
            Response::UserList { users } => format!("USERS:{}", users.join(",")),
            Response::NewMessage { from, content } => format!("MESSAGE:{}:{}", from, content),
        }
    }

    pub fn deserialize(data: &str) -> Result<Self, String> {
        let parts: Vec<&str> = data.splitn(3, ':').collect();
        match parts[0] {
            "OK" => Ok(Response::Ok { message: parts[1].to_string() }),
            "ERROR" => Ok(Response::Error { message: parts[1].to_string() }),
            "USERS" => Ok(Response::UserList { 
                users: parts[1].split(',').map(|s| s.to_string()).collect() 
            }),
            "MESSAGE" => Ok(Response::NewMessage { 
                from: parts[1].to_string(), 
                content: parts[2].to_string() 
            }),
            _ => Err("Réponse invalide".to_string()),
        }
    }
}
