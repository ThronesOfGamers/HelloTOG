use std::collections::VecDeque;
use std::io::{self, Write};

// ====================================================================
// STRUCTURES DE DONNÉES ET LOGIQUE INTERNE
// ====================================================================

/// Gère les opérations d'une File (Queue) selon le principe FIFO (First-In, First-Out).
/// Utilise VecDeque pour garantir des opérations O(1) aux deux extrémités.
struct MaFile { elements: VecDeque<String>, }
impl MaFile {
    pub fn new() -> Self { MaFile { elements: VecDeque::new() } }

    /// Ajout (ENQUEUE). O(1). Ajoute à l'arrière.
    pub fn enqueue(&mut self, element: String) {
        println!("  [ENQUEUE] Ajout de : '{}'", element);
        self.elements.push_back(element);
        self.afficher_etat();
    }

    /// Retrait (DEQUEUE). O(1). Retire au front (début).
    pub fn dequeue(&mut self) -> Option<String> {
        // pop_front() : L'élément le plus ancien (premier entré) est retiré.
        let resultat = self.elements.pop_front();
        match &resultat {
            Some(val) => println!("  [DEQUEUE] Retrait de : '{}' (FIFO)", val),
            None => println!("  [DEQUEUE] File vide. Retrait impossible."),
        }
        self.afficher_etat();
        resultat
    }

    /// Affiche l'état. Le front est à gauche (prochain à sortir).
    pub fn afficher_etat(&self) {
        let contenu: Vec<String> = self.elements.iter().map(|e| format!("'{}'", e)).collect();
        println!("  [ÉTAT] Prochain à sortir -> [{}] <- Dernier arrivé", contenu.join(" | "));
    }
}

/// Gère les opérations d'une Pile (Stack) selon le principe LIFO (Last-In, First-Out).
/// Utilise Vec<T> pour garantir des opérations O(1) à la fin.
struct MaPile { elements: Vec<String>, }
impl MaPile {
    pub fn new() -> Self { MaPile { elements: Vec::new() } }

    /// Ajout (PUSH). O(1). Ajoute au sommet (fin).
    pub fn push(&mut self, element: String) {
        println!("  [PUSH] Ajout de : '{}'", element);
        self.elements.push(element);
        self.afficher_etat();
    }

    /// Retrait (POP). O(1). Retire au sommet (fin).
    pub fn pop(&mut self) -> Option<String> {
        // pop() : L'élément le plus récent (dernier entré) est retiré.
        let resultat = self.elements.pop();
        match &resultat {
            Some(val) => println!("  [POP] Retrait de : '{}' (LIFO)", val),
            None => println!("  [POP] Pile vide. Retrait impossible."),
        }
        self.afficher_etat();
        resultat
    }

    /// Affiche l'état. Le sommet est à droite (dernier élément).
    pub fn afficher_etat(&self) {
        let contenu: Vec<String> = self.elements.iter().map(|e| format!("'{}'", e)).collect();
        println!("  [ÉTAT] Base -> [{}] <- Sommet", contenu.join(" | "));
    }
}

// ====================================================================
// LOGIQUE D'INTERACTION ET DE CONFIGURATION
// ====================================================================

/// Lit une ligne de l'utilisateur pour une commande, formatée en majuscule.
fn lire_choix() -> String {
    let mut choix = String::new();
    // Utilisation de io::stdin pour la robustesse de l'entrée/sortie
    io::stdin().read_line(&mut choix).expect("Erreur de lecture de l'entrée.");
    choix.trim().to_uppercase()
}

/// Permet à l'utilisateur de pré-configurer la Pile (PUSH/POP interactifs).
fn configurer_pile(mut pile: MaPile) {
    loop {
        println!("\n--- CONFIGURATION PILE (LIFO) ---");
        println!("  [A] Ajouter (PUSH) un élément");
        println!("  [R] Retirer (POP) l'élément au sommet");
        println!("  [S] Lancer la Séquence de Démo Automatique");

        pile.afficher_etat();

        print!("\nChoix (A/R/S) : ");
        io::stdout().flush().unwrap(); // Garantit que le prompt est affiché immédiatement

        let choix = lire_choix();

        match choix.as_str() {
            "A" => {
                print!("  Valeur à PUSH : ");
                io::stdout().flush().unwrap();
                let mut valeur = String::new();
                io::stdin().read_line(&mut valeur).unwrap();
                if !valeur.trim().is_empty() {
                    // La Pile est mue dans la fonction push, puis l'ownership est rendu à 'pile'
                    pile.push(valeur.trim().to_string());
                } else {
                    println!("  ⚠️ Valeur non valide. Veuillez entrer une chaîne de caractères.");
                }
            }
            "R" => {
                pile.pop();
            }
            "S" => {
                // MOVE : On transfère l'instance 'pile' configurée vers la fonction d'exécution.
                executer_demo_pile(pile);
                return; // Quitte la boucle de configuration et retourne au menu principal
            }
            _ => println!("  ⚠️ Choix invalide. Veuillez réessayer."),
        }
    }
}

/// Permet à l'utilisateur de pré-configurer la File (ENQUEUE/DEQUEUE interactifs).
fn configurer_file(mut file: MaFile) {
    loop {
        println!("\n--- CONFIGURATION FILE (FIFO) ---");
        println!("  [A] Ajouter (ENQUEUE) un élément");
        println!("  [R] Retirer (DEQUEUE) l'élément au front");
        println!("  [S] Lancer la Séquence de Démo Automatique");

        file.afficher_etat();

        print!("\nChoix (A/R/S) : ");
        io::stdout().flush().unwrap();

        let choix = lire_choix();

        match choix.as_str() {
            "A" => {
                print!("  Valeur à ENQUEUE : ");
                io::stdout().flush().unwrap();
                let mut valeur = String::new();
                io::stdin().read_line(&mut valeur).unwrap();
                if !valeur.trim().is_empty() {
                    file.enqueue(valeur.trim().to_string());
                } else {
                    println!("  ⚠️ Valeur non valide. Veuillez entrer une chaîne de caractères.");
                }
            }
            "R" => {
                file.dequeue();
            }
            "S" => {
                // MOVE : On transfère l'instance 'file' configurée vers la fonction d'exécution.
                executer_demo_file(file);
                return; // Quitte la boucle de configuration
            }
            _ => println!("  ⚠️ Choix invalide. Veuillez réessayer."),
        }
    }
}

// ====================================================================
// FONCTIONS DE DÉMONSTRATION AUTOMATIQUE
// ====================================================================

/// Exécute une séquence prédéfinie de la Pile à partir de l'état configuré par l'utilisateur.
fn executer_demo_pile(mut pile: MaPile) {
    println!("\n--- Début de la Séquence Automatique PILE LIFO ---");
    println!("  Pile initiale :");
    pile.afficher_etat(); // Affichage de l'état transmis par l'utilisateur

    println!("\n[ACTION] Appuyez sur Entrée pour lancer la séquence automatique...");
    let mut _temp = String::new();
    io::stdin().read_line(&mut _temp).unwrap(); // Pause pour contrôle utilisateur

    // Séquence automatique pour bien illustrer LIFO
    println!("\n--- Séquence d'opérations LIFO (PUSH-POP) ---");
    pile.push(String::from("PUSH A (Nouveau)"));
    pile.pop(); // Retire 'PUSH A (Nouveau)' immédiatement (LIFO)
    pile.push(String::from("PUSH B"));
    pile.push(String::from("PUSH C"));

    pile.pop(); // Retire 'PUSH C'
    pile.pop(); // Retire 'PUSH B'
    // Si la pile contenait des éléments de la configuration, ils sortiraient après.

    println!("\n--- Démonstration PILE terminée ---");
}

/// Exécute une séquence prédéfinie de la File à partir de l'état configuré par l'utilisateur.
fn executer_demo_file(mut file: MaFile) {
    println!("\n--- Début de la Séquence Automatique FILE FIFO ---");
    println!("  File initiale :");
    file.afficher_etat(); // Affichage de l'état transmis par l'utilisateur

    println!("\n[ACTION] Appuyez sur Entrée pour lancer la séquence automatique...");
    let mut _temp = String::new();
    io::stdin().read_line(&mut _temp).unwrap(); // Pause pour contrôle utilisateur

    // Séquence automatique pour bien illustrer FIFO
    println!("\n--- Séquence d'opérations FIFO (ENQUEUE-DEQUEUE) ---");
    file.enqueue(String::from("Tâche X (Nouveau)"));
    file.dequeue(); // Retire l'élément qui était le plus au front (FIFO)
    file.enqueue(String::from("Tâche Y"));
    file.enqueue(String::from("Tâche Z"));

    file.dequeue(); // Retire le prochain élément le plus ancien
    file.dequeue(); // Retire le suivant

    println!("\n--- Démonstration FILE terminée ---");
}

// ====================================================================
// ORCHESTRATION ET MENU PRINCIPAL
// ====================================================================

fn menu_principal() {
    loop {
        println!("\n========================================");
        println!("  CHOIX DE L'ALGORITHME À CONFIGURER");
        println!("========================================");
        println!("  [1] Configurer Pile (LIFO) - Last In, First Out");
        println!("  [2] Configurer File (FIFO) - First In, First Out");
        println!("  [Q] Quitter");

        print!("\nVotre choix : ");
        io::stdout().flush().unwrap();

        match lire_choix().as_str() {
            "1" => configurer_pile(MaPile::new()), // Commence la configuration avec une nouvelle pile vide
            "2" => configurer_file(MaFile::new()), // Commence la configuration avec une nouvelle file vide
            "Q" => {
                println!("Fermeture du simulateur. Au revoir !");
                break;
            }
            _ => println!("  ⚠️ Choix invalide. Veuillez réessayer."),
        }
    }
}

fn main() {
    menu_principal();
}