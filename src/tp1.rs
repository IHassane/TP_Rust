use std::io;

pub fn compte() {
    println!("Bonjour, merci de saisir votre choix : ");
    let mut end = false;
    let options = ["Afficher solde", "Retrait", "Liste compte", "Quitter"];

    while !end {
        println!("\nMenu :");

        for (i, option) in options.iter().enumerate() {
            println!("{}. {}", i + 1, option);
        }

        let mut choix = String::new();
        io::stdin()
            .read_line(&mut choix)
            .expect("Erreur de lecture");

        let choix: usize = match choix.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Veuillez saisir un numéro valide.");
                continue;
            }
        };

        match choix {
            1 => println!("Solde : 589,15 euros"),
            2 => {
                println!("Combien voulez-vous retirer ?");
                let mut retrait = String::new();
                io::stdin()
                    .read_line(&mut retrait)
                    .expect("Erreur de lecture");

                let montant: f64 = match retrait.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Veuillez saisir un montant valide.");
                        continue;
                    }
                };

                println!("Vous avez retiré {:.2} euros", montant);
            }
            3 => println!("Comptes disponibles : Compte Courant, Compte Épargne"),
            4 => {
                println!("À bientôt !");
                end = true;
            }
            _ => println!("Option invalide"),
        }
    }
}