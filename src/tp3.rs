use crate::fichier::Fichier;
use std::io;

pub fn gestion_fichier() {
    let fichier = Fichier::new("");

    loop {
        println!("\n=== Menu ===");
        println!("1. Lister le fichier");
        println!("2. Lire le fichier");
        println!("3. Créer fichier");
        println!("4. Modifier fichier");
        println!("5. Supprimer le fichier");
        println!("6. Quitter");

        print!("Choix : ");
        let mut choix = String::new();
        io::stdin()
            .read_line(&mut choix)
            .expect("Attention erreur de lecture");

        match choix.trim() {
            "1" => fichier.lister_fichier("."),
            "2" => fichier.lire(),
            "3" => fichier.creer(),
            "4" => fichier.modifier_fichier(),
            "5" => fichier.supprimer(),
            "6" => {
                println!("Fin du programme");
                break;
            }
            _ => println!("Choix invalide. Saisir un numéro entre 1 et 4."),
        }
    }
}
