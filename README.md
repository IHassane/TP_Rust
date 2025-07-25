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


9. Gestion de fichiers 

Dans les méthodes de gestion de fichiers qu’on a mises en place (lire, écrire, modifier, supprimer), on a utilisé plusieurs principes de base de Rust :

    Ownership & Borrowing : on passe des références (&self, &str) pour éviter de déplacer les valeurs, surtout dans les méthodes impl. Ça garantit qu’on garde le contrôle sur les données.

    Match : utilisé pour gérer les erreurs proprement, par exemple lors de l’ouverture d’un fichier avec match File::open(...). On traite les cas Ok et Err sans crasher le programme.

    Struct et impl : on a défini une struct Fichier avec un champ nom, et on a regroupé toutes les méthodes liées dans un bloc impl. Ça rend le code propre et organisé.

    Lecture/écriture : pour lire, on a utilisé BufReader et .lines(). Pour écrire, OpenOptions permet d’ouvrir ou créer un fichier, en mode append ou overwrite.

    Boucles et contrôle de flux : dans modifier_fichier, on utilise une boucle pour parcourir chaque ligne, demander à l'utilisateur s'il veut la changer, et on construit un nouveau contenu.

    Chemin dynamique : on a demandé le nom du fichier à l’utilisateur et ajouté .txt dynamiquement avec format!, ce qui montre l’usage de String.

10. Serveur de journalisation asynchrone

Projet final : implémentation d’un serveur TCP asynchrone permettant de recevoir des messages clients et de les écrire dans un fichier logs/server.log, avec un timestamp.
 Concepts appliqués :

    tokio pour la gestion asynchrone et la concurrence non bloquante.

    TcpListener et TcpStream pour l'écoute réseau.

    Tâches concurrentes : un client = une tâche (tokio::spawn).

    Arc<Mutex<>> : permet un accès concurrent sécurisé au fichier de log.

    Écriture sécurisée dans un fichier partagé par plusieurs clients.

    Horodatage avec chrono : chaque message logué contient [YYYY-MM-DD HH:MM:SS].

 Comportement :

    Plusieurs clients peuvent se connecter en parallèle.

    Chaque message reçu est affiché dans le terminal et sauvegardé dans server.log.

    Aucun conflit d'écriture grâce à l’usage de Mutex.

Exemple de log :

[2025-07-24 15:20:37] salut
[2025-07-24 15:20:41] je suis un message 

TP7. Client et Serveur DNS Simples

Dans ce TP, on a implémenté un client capable de formuler des requêtes DNS en UDP et un serveur répondant à quelques noms prédéfinis.
Concepts abordés :

    UdpSocket avec Tokio pour envoyer/recevoir des paquets.

    Sérialisation manuelle d’une requête DNS au format RFC 1035.

    Analyse binaire (parsing) d’une réponse DNS.

    Protocole DNS simplifié : en-tête, question, réponse (type A).

Ce que j’ai appris :

    À construire un message binaire à la main (header + question).

    Comment fonctionne une résolution DNS basique (client → serveur).

    Lire proprement une réponse DNS, en extrayant l’adresse IP.

    L’importance du respect du format binaire attendu pour que ça fonctionne.

Exemple :

Client : test.local -> 127.0.0.1
Serveur : réponse avec IP prédéfinie 192.168.1.100

TP8. Chat TCP structuré avec protocoles

Ce TP nous a fait structurer un système de chat textuel TCP entre plusieurs clients, avec un protocole personnalisé (REGISTER, LIST, SEND, DISCONNECT).
Concepts appliqués :

    TcpListener pour accepter des connexions entrantes.

    tokio::spawn pour gérer plusieurs clients en parallèle.

    Struct Message (enum) représentant les types d’échanges.

    Partage d’état global (liste des utilisateurs connectés) avec Arc<Mutex<>>.

    Sérialisation personnalisée : to_string() et from_str() pour notre protocole.

Ce que j’ai appris :

    À concevoir un mini-protocole réseau textuel.

    Comment utiliser les Channels, Arc et Mutex pour partager des données entre tâches.

    Séparer le protocole (src/protocol.rs) de la logique réseau.

Exemple de session :

Client A → REGISTER Alice
Serveur → Ok: Bienvenue Alice
Client A → LIST
Serveur → UserList: [Bob, Charlie]
Client A → SEND Bob:Salut
Serveur → Error: Utilisateur introuvable

TP9. Serveur et Client WebSocket

Ce TP explore les WebSockets avec tokio-tungstenite pour permettre une communication bidirectionnelle persistante, adaptée au temps réel.
Concepts utilisés :

    Handshake WebSocket via HTTP Upgrade.

    Crates : tokio-tungstenite, futures_util, url.

    Full-duplex : lecture/écriture simultanée via split() du WebSocketStream.

    tokio::select pour gérer plusieurs flux d’entrée/sortie.

    Connexion à ws://localhost:8080.

Ce que j’ai appris :

    Différence entre une simple connexion TCP et un WebSocket.

    Comment écouter et répondre à des messages texte en temps réel.

    Gérer plusieurs clients WebSocket avec une boucle principale.

Exemple :

Client A connecté
→ Message : Salut
← Réponse du serveur : [15:30:01] Alice: Salut