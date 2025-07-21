use std::io;

pub fn tp0() {
    let nom = "issame hassane";
    let age: u32 = 26; //u32 = entier non signé sur 32 bits (valeur positives)
    let age_papa = 60;
    println!("Hello world");
    println!("j'ai {age} ans");
    println!("papa a {age_papa}");

    // il faut utiliser les nakes_cases (par convention de RUST)
    // ne jamais commencer par un chiffre ,pas d'espaces ni tirets

    //2. les fonctions :
    //fn définit une fonction
    //&str est un type de chaine de caractères (référence)
    //on cree une fonction addition() qui retourne une somme et on l'apelle depuis la focntion main

    let resultat: i32 = addition(10, 35);
    println!("{resultat}");

    say_hello("Luiguie");

    // Les conditions les boucles

    let nombre = 16;
    if nombre % 2 == 0 {
        println!("Pair !")
    } else {
        println!("Impair !")
    }

    // Une boucle
    for i in 1..=10 {
        println!("i vaut {i}")
    }

    //Exemple de tableau
    let voitures = ["jeep", "renault", "bmw"];
    for voiture in voitures {
        println!("Voiture : {voiture}")
    }

    // for (index, valeur) in collection.iter().enumerate(){
    // on peut utiliser index et valeur ici}

    //je reprends l'exemple de voiture
    for (i, voiture) in voitures.iter().enumerate() {
        println!("Index :{i} : {voiture}")
    }

    //iter(): crée un iterateur sur la collection sans le consommer
    //enumerate: transform l'itérateur en une séquence d'index , valeur

    let noms = vec![String::from("Kevin"), String::from("Nourdine")];
    for (i, nom) in noms.iter().enumerate() {
        println!("Nom {i} : {nom}")
    }

    //Usage de enumerate dans un cas réel : Afficher un menu avec numéro et choix

    let options = ["Afficher solde", "Retrait", "Liste compte", "Quitter"];
    println!("Menu:");

    for (i, option) in options.iter().enumerate() {
        //Afficher chaque option et on commence par 1
        println!("{},{}", i + 1, option)
    }

    println!("Veuillez saisir le numero de votre choix");

    let mut choix = String::new();
    io::stdin()
        .read_line(&mut choix)
        .expect("Attention erreur de lecture");
    let choix: usize = match choix.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Veuillez saisir un numero valide");
            return;
        }
    };

    if choix == 0 || choix > options.len() {
        println!("choix hors du système")
    } else {
        println!("vous avez selectionné {choix} ")
    }

    // Les tableaux
    let tab: [i32; 4] = [11, 23, 19, 19];
    let _tab2: [i32; 4] = [11, 23, 19, 19];
    // pour eviter le warning d'une variable non utilisée on rajoute _ devant la variable

    //parcourir le tableau
    for i in 0..tab.len() {
        println!("le tableau tab{}", tab[i]);
    }

    for &elt in &tab {
        println!("l'element est {}", elt)
    }
    // &elt => iterer sur des references aux elements du tableau
    //&tab => on passe

    println!("**********************************************");
    //Les loop
    let mut compteur = 0;
    loop {
        println!("Compteur: {}", compteur);
        compteur += 1;
        if compteur == 3 {
            break;
        }
    }

    println!("**********************While********************");

    let mut compteur2 = 0;
    while compteur2 < 4 {
        println!("Compteur: {}", compteur2);
        compteur2 += 1;
    }

    println!("**********************Struct********************");

    struct Salarie{
        nom : String,
        ville : String,
        age : u32,
    }

    //usage struct on crée une instance de la stucture
    let kevin  = Salarie {
        nom:String::from("Kevin"),
        ville:String::from("Lyon"),
        age:34};

    // Pour acceder aux attributs de la structure
    println!("Nom: {},Ville: {},Age: {}",kevin.nom,kevin.ville,kevin.age);  

    //Match

    let nombre = 5;

    match nombre {
        1 => println!("Un"),
        2 => println!("Deux"),
        3 => println!("Trois"),
        4 => println!("Quatre"),
        5 => println!("Cinq"),
        _ => println!("Autre nombre")// cas par defaut
    }


    // Fonctions associés (impl) pour des structures (struct)

    struct Personne{
        nom: String
    }

    impl Personne{
        fn afficher(&self){//emprunt immuable => ne modifie rien
            println!("La personne suivante {} est convoquée ",&self.nom)
        }
    }

    let personne = Personne {
        nom:"Alexandre".to_string()
    };

    personne.afficher();

    //Exemple compteur struct

    struct Compteur{
        value : u32 
    }
    
     // A noter 
    // &self -> lecture seul
    // &mut self -> modifcation possible
    //
    

    impl Compteur {
        fn afficher(&self){
            println!("Valeur actuelle : {}",self.value);
        }      

        fn incrementer(&mut self){
            self.value+=1;
        }

        fn deplacer(self){
            println!("Deplacée : {}",self.value); // Self deplacé ici , plus accessible
        }
    }

    let mut compteur = Compteur{
        value:0
    };


    compteur.afficher();
    compteur.incrementer();
    compteur.deplacer();


}

fn addition(n1: i32, n2: i32) -> i32 {
    n1 + n2
}

fn say_hello(nom: &str) {
    println!("Hello {nom}");
}
