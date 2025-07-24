use tokio::net::{TcpListener, TcpStream};
use tokio::io::{BufReader, AsyncBufReadExt, AsyncWriteExt};
use tokio::fs::OpenOptions;
use chrono::Local;
use std::sync::Arc;
use tokio::sync::Mutex;
use std::io;

pub async fn journalisation() -> Result<(), io::Error> {
    let fichier_log = ouvrir_fichier_log().await?;
    
    // Étape 2: Démarrer le serveur TCP
    println!("Démarrage du serveur...");
    let listener = demarrer_serveur().await?;
    
    // Étape 3: Boucle principale pour les clients
    println!("Serveur prêt ! En attente de clients");
    boucle_principale_serveur(listener, fichier_log).await
}

// Fonction pour configurer le fichier de log
async fn ouvrir_fichier_log() -> Result<Arc<Mutex<tokio::fs::File>>, io::Error> {
    println!("Ouverture du fichier logs/server.log");
    
    let fichier = OpenOptions::new()  
        .append(true)  
        .open("logs/server.log")
        .await?;
    
    // Envelopper le fichier dans Arc<Mutex<>> pour le partage entre threads
    let fichier_partage = Arc::new(Mutex::new(fichier));
    
    println!("Fichier de log prêt");
    Ok(fichier_partage)
}

// Fonction pour démarrer le serveur TCP
async fn demarrer_serveur() -> Result<TcpListener, io::Error> {
    let adresse = "127.0.0.1:8080";
    let listener = TcpListener::bind(adresse).await?;
    Ok(listener)
}

// Boucle principale qui accepte les connexions clients
async fn boucle_principale_serveur(
    listener: TcpListener, 
    fichier_log: Arc<Mutex<tokio::fs::File>>
) -> Result<(), io::Error> {
    
    loop {
        println!("\nEn attente d'une nouvelle connexion...");
        
        // Attendre qu'un client se connecte
        match listener.accept().await {
            Ok((socket, adresse_client)) => {
                println!("Nouveau client connecté depuis : {}", adresse_client);
                
                // Créer une copie du fichier de log pour ce client
                let fichier_pour_ce_client = fichier_log.clone();
                
                // Lancer une tâche séparée pour gérer ce client
                tokio::spawn(async move {
                    println!("Démarrage du traitement pour {}", adresse_client);
                    
                    // Traiter les messages de ce client
                    let resultat = traiter_messages_client(socket, fichier_pour_ce_client).await;
                    
                    // Gérer les erreurs
                    match resultat {
                        Ok(_) => {
                            println!("Client {} déconnecté", adresse_client);
                        }
                        Err(erreur) => {
                            println!("Erreur avec le client {} : {:?}", adresse_client, erreur);
                        }
                    }
                });
            }
            Err(erreur) => {
                println!("Erreur lors de l'acceptation d'une connexion : {:?}", erreur);
                // Continuer malgré l'erreur (ne pas arrêter le serveur)
            }
        }
    }
}

// Fonction pour traiter les messages d'un client spécifique
async fn traiter_messages_client(
    socket: TcpStream, 
    fichier_log: Arc<Mutex<tokio::fs::File>>
) -> Result<(), io::Error> {
    
    println!("Préparation de la lecture des messages...");
    
    // Créer un lecteur pour lire les lignes du socket
    let reader = BufReader::new(socket);
    let mut lignes = reader.lines();
    
    println!("Prêt à recevoir des messages");
    
    // Lire chaque ligne envoyée par le client
    while let Some(ligne) = lignes.next_line().await? {
        println!("Message reçu : '{}'", ligne);
        
        // Traiter cette ligne de message
        traiter_une_ligne(ligne, &fichier_log).await?;
    }
    
    println!("Plus de messages à lire (client déconnecté)");
    Ok(())
}

// Fonction pour traiter une ligne de message
async fn traiter_une_ligne(
    message: String, 
    fichier_log: &Arc<Mutex<tokio::fs::File>>
) -> Result<(), io::Error> {
    
    // Étape 1: Créer un timestamp
    let maintenant = Local::now();
    let timestamp = maintenant.format("%Y-%m-%d %H:%M:%S").to_string();
    
    // Étape 2: Formater la ligne de log
    let ligne_complete = format!("[{}] {}\n", timestamp, message);
    println!("Ligne formatée : '{}'", ligne_complete.trim_end());
    
    // Étape 3: Écrire dans le fichier
    ecrire_dans_fichier(ligne_complete, fichier_log).await?;
    
    Ok(())
}

// Fonction pour écrire dans le fichier de log
async fn ecrire_dans_fichier(
    ligne_log: String, 
    fichier_log: &Arc<Mutex<tokio::fs::File>>
) -> Result<(), io::Error> {
    
    println!("Acquisition du verrou sur le fichier");
    
    // Récupération du verrou sur le fichier (un seul thread peut écrire à la fois)
    let mut fichier = fichier_log.lock().await;
    
    println!("Écriture dans le fichier");
    
    // Écrire la ligne dans le fichier
    fichier.write(ligne_log.as_bytes()).await?;   
    
    println!("Ligne sauvegardée avec succès");
    
    // Le verrou est automatiquement libéré a la fin de la méthode
    Ok(())
}