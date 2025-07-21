use std::io;

struct CompteBancaire {
    nom: String,
    solde: f64,
}

impl CompteBancaire {
    fn afficher(&self) {
        println!("Compte de {} Euros", self.solde)
    }

    fn deposer(&mut self, montant: f64) {
        self.solde += montant;
        println!("+{} Euros déposés:", montant);
    }

    fn retirer(&mut self, montant: f64) {
        if self.solde >= montant {
            self.solde -= montant;
            println!("-{} Euros retirés", montant);
        } else {
            println!("Solde insuffisant");
        }
    }

    fn fermer(self) {
        println!(
            "Le compte de {} est fermé, dernier solde : {} Euros",
            self.nom, self.solde
        )
    }
    // self ici est consomé ici , on ne peut plus utiliser l'objet ensuite
}

fn lire_entree_en_float() -> f64 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Erreur de lecture");
    match input.trim().parse::<f64>() {
        Ok(m) => m,
        Err(_) => {
            println!("Montant invalide !");
            0.0
        }
    }
}

pub fn main() {
    let mut compte = CompteBancaire {
        nom: String::from("Issame"),
        solde: 100.0,
    };

    let options = ["Afficher solde", "Retrait", "Dépôt", "Quitter"];

    loop {
        println!("\n📋 Menu :");
        for (i, option) in options.iter().enumerate() {
            println!("{}. {}", i + 1, option);
        }

        println!("➡️  Entrez le numéro de votre choix :");
        let mut choix = String::new();
        io::stdin().read_line(&mut choix).expect("Erreur de lecture");

        let choix: usize = match choix.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Veuillez entrer un nombre valide.");
                continue;
            }
        };

        match choix {
            1 => compte.afficher(),
            2 => {
                println!("Montant à retirer :");
                let montant = lire_entree_en_float();
                if montant > 0.0 {
                    compte.retirer(montant);
                }
            }
            3 => {
                println!("Montant à déposer :");
                let montant = lire_entree_en_float();
                if montant > 0.0 {
                    compte.deposer(montant);
                }
            }
            4 => {
                println!("À bientôt, {} !", compte.nom);
                break;
            }
            _ => println!("Option invalide"),
        }
    }
}
