use std::{fs::{remove_file, File, OpenOptions}, io::{BufRead, BufReader}};
use std::io::Write;

use chrono::Local;

pub struct Fichier {
    nom: String,
}

impl Fichier {
    pub fn new(nom: &str) -> Self {
        Fichier { nom: nom.to_string() }
    }

    pub fn lire(&self) {
        let file = File::open(&self.nom);
        match file {
            Ok(f) => {
                println!("Lecture du fichier ({}):", Local::now());
                let reader = BufReader::new(f);
                for line in reader.lines() {
                    if let Ok(ligne) = line {
                        println!("{}", ligne);
                    }
                }
            }
            Err(_) => println!("Impossible de lire le fichier."),
        }
    }

    pub fn ecrire(&self, texte: &str) {
        let mut file = OpenOptions::new().create(true).append(true).open(&self.nom).unwrap();
        writeln!(file, "{}", texte).unwrap();
        println!("Écriture réussie ({})", Local::now());
    }

    pub fn supprimer(&self) {
        match remove_file(&self.nom) {
            Ok(_) => println!("Fichier supprimé ({})", Local::now()),
            Err(_) => println!("Impossible de supprimer."),
        }
    }
}