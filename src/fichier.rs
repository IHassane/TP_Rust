use std::io::Write;
use std::{fs, io};
use std::{
    fs::{File, OpenOptions, remove_file},
    io::{BufRead, BufReader},
};

use chrono::Local;

pub struct Fichier {
    nom: String,
}

impl Fichier {
    pub fn new(nom: &str) -> Self {
        Fichier {
            nom: nom.to_string(),
        }
    }

    pub fn lister_fichier(&self, dossier: &str) {
        println!("Fichiers dans le dossier");

        match fs::read_dir(dossier) {
            Ok(entries) => {
                for entry in entries {
                    if let Ok(entry) = entry {
                        let path = entry.path();
                        if path.is_file() {
                            if let Some(ext) = path.extension() {
                                if ext == "txt" {
                                    if let Some(nom) = path.file_name() {
                                        println!("{}", nom.to_string_lossy());
                                    }
                                }
                            }
                        }
                    }
                }
            }
            Err(e) => println!("Impossible de lire le dossier : {}", e),
        }
    }

    pub fn lire(&self) {
        // Demander le nom du fichier à l'utilisateur
        println!("Entrez le nom du fichier (ex: monfichier.txt) :");
        let mut nom_fichier = String::new();
        io::stdin()
            .read_line(&mut nom_fichier)
            .expect("Erreur lecture");
        let nom_fichier = nom_fichier.trim();
        let nom_fichier_txt = format!("{}.txt", nom_fichier);
        let file = File::open(nom_fichier_txt);
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

    pub fn creer(&self) {
        // Demander le nom du fichier à l'utilisateur
        println!("Entrez le nom du fichier (ex: monfichier.txt) :");
        let mut nom_fichier = String::new();
        io::stdin()
            .read_line(&mut nom_fichier)
            .expect("Erreur lecture");
        let nom_fichier = nom_fichier.trim();
        let nom_fichier_txt = format!("{}.txt", nom_fichier);

        // Ouvrir le fichier en mode écriture (truncate = écrase le fichier)
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true) // Important : écrase le contenu existant
            .open(&nom_fichier_txt)
            .expect("Impossible d'ouvrir le fichier");

        writeln!(file).expect("Erreur d'écriture");
        println!(
            "Écriture réussie dans '{}' ({})",
            nom_fichier_txt,
            Local::now()
        );
    }

    pub fn modifier_fichier(&self) {
        // Demander le nom du fichier à modifier
        println!("Entrez le nom du fichier (sans l'extension .txt) :");
        let mut nom_fichier = String::new();
        io::stdin().read_line(&mut nom_fichier).unwrap();
        let nom_fichier = nom_fichier.trim();
        let chemin = format!("{}.txt", nom_fichier);

        // Ouvrir le fichier en lecture
        let file = File::open(&chemin).expect("Impossible d'ouvrir le fichier");
        let reader = BufReader::new(file);

        // Lire les lignes et proposer la modification
        let mut lignes: Vec<String> = vec![];
        for (i, ligne) in reader.lines().enumerate() {
            let contenu = ligne.unwrap();
            println!("Ligne {} : {}", i + 1, contenu);

            println!("Voulez-vous modifier cette ligne ? (y/n) :");
            let mut reponse = String::new();
            io::stdin().read_line(&mut reponse).unwrap();

            if reponse.trim().eq_ignore_ascii_case("y") {
                println!("Entrez la nouvelle valeur :");
                let mut nouvelle_ligne = String::new();
                io::stdin().read_line(&mut nouvelle_ligne).unwrap();
                lignes.push(nouvelle_ligne.trim().to_string());
            } else {
                lignes.push(contenu);
            }
        }

        // Réécrire le fichier avec les lignes modifiées
        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&chemin)
            .expect("Impossible d'ouvrir le fichier en écriture");

        for ligne in lignes {
            writeln!(file, "{}", ligne).unwrap();
        }

        println!("Fichier mis à jour avec succès !");
    }

    pub fn supprimer(&self) {
        // Demander le nom du fichier à l'utilisateur
        println!("Entrez le nom du fichier (ex: monfichier.txt) :");
        let mut nom_fichier = String::new();
        io::stdin()
            .read_line(&mut nom_fichier)
            .expect("Erreur lecture");
        let nom_fichier = nom_fichier.trim();
        let nom_fichier_txt = format!("{}.txt", nom_fichier);
        match remove_file(nom_fichier_txt) {
            Ok(_) => println!("Fichier supprimé ({})", Local::now()),
            Err(_) => println!("Impossible de supprimer."),
        }
    }
}
