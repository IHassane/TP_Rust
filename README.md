1. Variables et types

    Déclaration de variables avec des types explicites, par exemple age: u32 pour un entier non signé 32 bits.

    Utilisation des chaînes de caractères (&str et String).

    Respect des conventions Rust : snake_case pour les noms de variables.

2. Fonctions

    Définition simple avec fn.

    Fonction addition qui retourne la somme de deux entiers.

    Fonction say_hello prenant une référence à une chaîne (&str) pour afficher un message.

3. Contrôle de flux

    Condition if-else pour tester si un nombre est pair ou impair.

    Boucles for avec différentes syntaxes :

        Plage 1..=10 (inclusif).

        Parcours d’un tableau.

        Utilisation de .iter().enumerate() pour avoir l’index et la valeur.

4. Collections

    Tableaux fixes [type; taille] et vecteurs dynamiques Vec<T>.

    Itération sur les éléments avec références (&elt) pour éviter la copie.

5. Boucles infinies et contrôle

    Utilisation de loop avec break pour sortir.

    Boucle while classique.

6. Structures (struct)

    Définition d’une structure Salarie avec plusieurs champs.

    Création d’instances et accès aux champs avec la syntaxe instance.champ.

7. match

    Structure de contrôle puissante, similaire au switch.

    Exemple simple avec nombres et un cas par défaut (_).

8. Méthodes associées aux structures

    Déclaration d’une impl pour la struct Personne.

    Méthode afficher qui utilise un emprunt immuable (&self).

    Exemple d’une struct Compteur avec méthodes :

        afficher (lecture)

        incrementer (modification avec &mut self)

        deplacer (consommation de la valeur avec self)

Ce que j’ai retenu

    Rust est strict sur la gestion des emprunts et des possessions (ownership).

    Les fonctions peuvent prendre différents types d’emprunts selon si on veut modifier ou non.

    Les structures permettent d’organiser les données et leur comportement.

    Les boucles et conditions sont simples et puissantes.

    match est très utile pour gérer plusieurs cas.

    L’itération avec .iter() et .enumerate() est pratique pour les collections.


9. Gestion de fichiers — principes appliqués

Dans les méthodes de gestion de fichiers qu’on a mises en place (lire, écrire, modifier, supprimer, etc.), on a utilisé plusieurs principes de base de Rust :

    Ownership & Borrowing : on passe des références (&self, &str) pour éviter de déplacer les valeurs, surtout dans les méthodes impl. Ça garantit qu’on garde le contrôle sur les données.

    Match : utilisé pour gérer les erreurs proprement, par exemple lors de l’ouverture d’un fichier avec match File::open(...). On traite les cas Ok et Err sans crasher le programme.

    Struct et impl : on a défini une struct Fichier avec un champ nom, et on a regroupé toutes les méthodes liées dans un bloc impl. Ça rend le code propre et organisé.

    Lecture/écriture : pour lire, on a utilisé BufReader et .lines(). Pour écrire, OpenOptions permet d’ouvrir ou créer un fichier, en mode append ou overwrite.

    Boucles et contrôle de flux : dans modifier_fichier, on utilise une boucle pour parcourir chaque ligne, demander à l'utilisateur s'il veut la changer, et on construit un nouveau contenu.

    Chemin dynamique : on a demandé le nom du fichier à l’utilisateur et ajouté .txt dynamiquement avec format!, ce qui montre l’usage de String.