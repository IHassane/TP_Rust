use std::error::Error;
use std::fs::File;
use csv::Writer;

pub fn ecrire() -> Result<(), Box<dyn Error>> {
    let file = File::create("donnees.csv")?;
    let mut wtr = Writer::from_writer(file);

    // Écrire l'en-tête
    wtr.write_record(&["nom", "age", "ville"])?;

    // Écrire des lignes de données
    wtr.write_record(&["Alice", "30", "Paris"])?;
    wtr.write_record(&["Bob", "25", "Lyon"])?;
    wtr.write_record(&["Claire", "28", "Marseille"])?;

    // Terminer l'écriture
    wtr.flush()?;
    println!("Fichier CSV écrit avec succès !");
    Ok(())
}