use std::fmt::{self, Debug};
use std::mem;

// --- DÉFINITIONS DE STRUCTURES ET TYPES (Identiques) ---

type Lien<T> = Option<Box<Noeud<T>>>;

struct Noeud<T> {
    valeur: T,
    prochain: Lien<T>,
}

pub struct ListeChainee<T> {
    tete: Lien<T>,
}

// --- IMPLEMENTATION DES FONCTIONNALITÉS (Avec Débogage) ---

impl<T: Debug> ListeChainee<T> {
    pub fn new() -> Self {
        ListeChainee { tete: None }
    }

    /// Ajoute un élément au début de la liste (opération O(1)).
    pub fn push(&mut self, element: T) {
        println!("  [PUSH - Début] Élément: {:?}", element);
        println!("  [PUSH - Étape 1] Tête actuelle (avant remplacement): {:?}", self.tete.as_ref().map(|b| &b.valeur));

        // 1. Prendre l'ancienne tête et la remplacer par 'None'.
        let ancienne_tete = mem::replace(&mut self.tete, None);

        println!("  [PUSH - Étape 2] Tête ancienne extraite (Ownership déplacé). Tête de la liste est maintenant 'None'.");

        // 2. Créer le nouveau nœud.
        let nouveau_noeud = Box::new(Noeud {
            valeur: element,
            prochain: ancienne_tete, // L'ownership de l'ancienne liste est transféré ici
        });

        println!("  [PUSH - Étape 3] Nouveau Nœud créé. Son 'prochain' pointe vers l'ancienne tête.");

        // 3. Le nouveau nœud devient la nouvelle tête.
        self.tete = Some(nouveau_noeud);

        println!("  [PUSH - Fin] Nouvelle tête établie: {:?}", self.tete.as_ref().map(|b| &b.valeur));
    }

    /// Retire et retourne l'élément au début de la liste (opération O(1)).
    pub fn pop(&mut self) -> Option<T> {
        println!("  [POP - Début] Tête actuelle: {:?}", self.tete.as_ref().map(|b| &b.valeur));

        // 1. Prendre l'ownership de la tête.
        let resultat = self.tete.take().map(|mut noeud| {
            println!("  [POP - Étape 1] Tête extraite (Ownership pris). Tête de la liste est maintenant 'None'.");

            // 2. Le prochain nœud devient la nouvelle tête.
            self.tete = noeud.prochain.take();

            println!("  [POP - Étape 2] Le 'prochain' du nœud retiré devient la nouvelle tête.");
            println!("  [POP - Étape 3] Nouvelle tête établie: {:?}", self.tete.as_ref().map(|b| &b.valeur));

            // 3. Retourner la valeur du nœud retiré.
            noeud.valeur
        });

        println!("  [POP - Fin] Résultat du POP: {:?}", resultat.as_ref());
        resultat
    }
}

// --- IMPLÉMENTATIONS DE TRAITS (Identiques pour l'affichage et le nettoyage) ---

impl<T> Drop for ListeChainee<T> {
    fn drop(&mut self) {
        let mut courant = self.tete.take();
        while let Some(mut noeud) = courant {
            courant = noeud.prochain.take();
        }
    }
}

impl<T: Debug> Debug for ListeChainee<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ListeChainee [")?;
        let mut courant = self.tete.as_ref();

        while let Some(noeud) = courant {
            write!(f, "{:?}", noeud.valeur)?;
            courant = noeud.prochain.as_ref();
            if courant.is_some() {
                write!(f, " -> ")?;
            }
        }
        write!(f, "]")
    }
}

// --- DÉMONSTRATION ---

// ... (Toutes les définitions de structures et implémentations de fonctions restent ici) ...
// ... (y compris push, pop, Drop, et Debug) ...

fn main() {
    println!("\n==================================================");
    println!("  DÉMONSTRATION VISUELLE : Contenu de la Liste");
    println!("==================================================");

    let mut liste = ListeChainee::new();
    println!("\n[ÉTAT INITIAL] -> {:?}", liste);

    // --- PUSH DÉTAILLÉ ---
    println!("\n--- PUSH 42 ---");
    liste.push(42);
    // Affichage de l'état complet après l'opération
    println!("Contenu COMPLET : {:?}", liste);

    println!("\n--- PUSH 99 ---");
    liste.push(99);
    // Affichage de l'état complet après l'opération
    println!("Contenu COMPLET : {:?}", liste);

    println!("\n--- PUSH 10 ---");
    liste.push(10);
    // Affichage de l'état complet après l'opération
    println!("Contenu COMPLET : {:?}", liste);


    // --- POP DÉTAILLÉ ---
    println!("\n--- POP 1 (Retrait du 10) ---");
    if let Some(valeur) = liste.pop() {
        println!("  Résultat POP : Valeur retirée: {}", valeur);
    }
    // Affichage de l'état complet après l'opération
    println!("Contenu COMPLET : {:?}", liste);

    println!("\n--- POP 2 (Retrait du 99) ---");
    if let Some(valeur) = liste.pop() {
        println!("  Résultat POP : Valeur retirée: {}", valeur);
    }
    // Affichage de l'état complet après l'opération
    println!("Contenu COMPLET : {:?}", liste);

    // --- POP sur liste vide ---
    println!("\n--- POP 3 (Retrait du 42) ---");
    if let Some(valeur) = liste.pop() {
        println!("  Résultat POP : Valeur retirée: {}", valeur);
    }
    println!("Contenu COMPLET : {:?}", liste);

    println!("\n--- POP sur Liste Vide ---");
    match liste.pop() {
        Some(valeur) => println!("Erreur : Popped {} sur liste vide!", valeur),
        None => println!("  Résultat POP : Liste vide. Retourne None."),
    }
    println!("Contenu COMPLET : {:?}", liste);

    println!("\n==================================================");
}