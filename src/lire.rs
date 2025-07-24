use std::error::Error;
use std::fs::File;
use csv::Reader;

pub fn lire() -> Result<(), Box<dyn Error>> {
    let file = File::open("donnees.csv")?;
    let mut rdr = Reader::from_reader(file);

    for result in rdr.records() {
        let record = result?;
        println!("{:?}", record);
    }

    Ok(())
}
