mod compteBancaire;
mod tp0;
mod tp1;
mod ecrire;
mod lire;
mod tp3;
mod fichier;
mod asynchrone;
mod tp4;
mod tp7;

#[tokio::main]
async fn main() {
    //tp0::tp0();
    //tp1::compte();
    //compteBancaire::main();
    //ecrire :: ecrire();
    //lire :: lire();
    //tp3 :: gestion_fichier();
    //asynchrone :: affichermessage().await;
    //asynchrone :: afficher_messages_en_parallele().await;
    //asynchrone :: afficher_sequentiellement().await;
    //tp4 :: journalisation().await;
    tp7 :: dns().await;
}
