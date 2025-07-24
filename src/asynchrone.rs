use std::time::Instant;

use tokio::time::{sleep, Duration};

pub async fn affichermessage() {
    println!("Je suis un message asynchrone !");
    sleep(Duration::from_secs(3)).await;
}

pub async fn attendre_message(nom: &str, secondes: u64) {
    println!("{} en attente pendant {} secondes...", nom, secondes);
    sleep(Duration::from_secs(2)).await;
    println!("{} terminé !", nom);
}

pub async fn afficher_messages_en_parallele() {
    println!("\n--- Exécution parallèle ---");
    let debut = Instant::now(); 

    let t1 = attendre_message("Tâche 1", 2);
    let t2 = attendre_message("Tâche 2", 3);
    let t3 = attendre_message("Tâche 3", 1);

    tokio::join!(t1, t2, t3);

    let duree = debut.elapsed(); 
    println!("Toutes les tâches parallèles sont terminées en {:?} !", duree);
}

pub async fn afficher_sequentiellement() {
    println!("\n--- Exécution séquentielle ---");
    let debut = Instant::now(); 

    attendre_message("Tâche A", 2).await;
    attendre_message("Tâche B", 3).await;
    attendre_message("Tâche C", 1).await;

    let duree = debut.elapsed(); 
    println!("Exécution séquentielle terminée en {:?} !", duree);
}