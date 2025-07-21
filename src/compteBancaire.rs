use core::time;
use std::{io, thread, time::Duration};

struct CompteBancaire {
    nom: String,
    solde: f64,
}

impl CompteBancaire {
    fn afficher(&self) {
        println!("Solde: {} Euros", self.solde)
    }

    fn deposer(&mut self, montant: f64) {
        self.solde += montant;
        println!("+{} Euros déposés:", montant);
        println!("Nouveau solde : {}", self.solde)
    }

    fn retirer(&mut self, montant: f64) {
        if self.solde >= montant {
            self.solde -= montant;
            println!("-{} Euros retirés", montant);
            println!("Nouveau solde : {}", self.solde)
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

    fn renommer(self, nouveau_nom: String) -> CompteBancaire {
        CompteBancaire {
            nom: nouveau_nom,
            solde: self.solde,
        }
    }
    // self ici est consomé ici , on ne peut plus utiliser l'objet ensuite
}

fn lire_entree_en_float() -> f64 {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Erreur de lecture");
    match input.trim().parse::<f64>() {
        Ok(m) => m,
        Err(_) => {
            println!("Montant invalide !");
            0.0
        }
    }
}

pub fn main() {
    let mut comptes = vec![
        CompteBancaire {
            nom: String::from("Luigi"),
            solde: 500.0,
        },
        CompteBancaire {
            nom: String::from("Pierre-Henry"),
            solde: 390.0,
        },
        CompteBancaire {
            nom: String::from("Issame"),
            solde: 100.0,
        },
    ];

    let options = ["Afficher solde", "Retrait", "Dépôt", "Quitter","Supprimer"];

    loop {
        println!("Veuillez selctionner un compte:");

        for (_i, compte) in comptes.iter().enumerate() {
            println!("Numero : {}, Nom: {}", _i, compte.nom)
        }

        println!("Numero du compte:...");

        let mut compte_numero = String::new();
        io::stdin()
            .read_line(&mut compte_numero)
            .expect("Erreur de lecture");

        let compte_numero: usize = match compte_numero.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Veuillez saisir un numéro valide.");
                continue;
            }
        };

        if compte_numero >= comptes.len() {
            println!("Numero de compte invalide");
            continue;
        }

        let compte = &mut comptes[compte_numero];

        println!("Le compte selectionné est celui de {}", compte.nom);
        thread::sleep(Duration::from_secs(2));

        loop {
            println!("Menu :");
            for (i, option) in options.iter().enumerate() {
                println!("{}. {}", i + 1, option);
            }

            println!("➡️  Entrez le numéro de votre choix :");
            let mut choix = String::new();
            io::stdin()
                .read_line(&mut choix)
                .expect("Erreur de lecture");

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
                    if montant > 10.0 {
                        compte.deposer(montant);
                    } else {
                        println!("Le montant minimum pour un dépôt et de 10 euros")
                    }
                }
                4 => {
                    println!("À bientôt, {} !", compte.nom);
                    break;
                }
                5 => {
                    let compte = comptes.remove(compte_numero);
                    println!("Suppression du compte {} !!!", compte.nom);
                    compte.fermer();
                    break;
                }
                _ => println!("Option invalide"),
            }

            thread::sleep(Duration::from_secs(4));
        }
    }
}
