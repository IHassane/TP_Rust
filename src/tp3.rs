use crate::fichier::Fichier;
use std::io;

pub fn gestion_fichier() {
    let fichier = Fichier::new("monfichier.csv");

    loop {
        println!("\n=== Menu ===");
        println!("1. Lire le fichier");
        println!("2. Écrire une ligne");
        println!("3. Supprimer le fichier");
        println!("4. Quitter");

        print!("Choix : ");
        let mut choix = String::new();
        io::stdin()
            .read_line(&mut choix)
            .expect("Attention erreur de lecture");

        match choix.trim() {
            "1" => fichier.lire(),
            "2" => {
                println!("Entrez une ligne CSV (ex: nom,age,ville) :");
                let mut ligne = String::new();
                io::stdin().read_line(&mut ligne).unwrap();
                fichier.ecrire(ligne.trim());
            }
            "3" => fichier.supprimer(),
            "4" => {
                println!("Fin du programme");
                break;
            }
            _ => println!("Choix invalide. Saisir un numéro entre 1 et 4."),
        }
    }
}
