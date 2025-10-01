use std::collections::HashMap;

/// Compte la fréquence de chaque mot dans une chaîne de texte.
///
/// Cette fonction utilise une chaîne d'itérateurs et la méthode 'fold'
/// pour une implémentation idiomatique et optimisée.
///
/// # Arguments
/// * `texte` - La chaîne de texte à analyser.
///
/// # Retour
/// Un HashMap (String, usize) des mots et de leurs comptes.
fn compter_frequences_optimise(texte: &str) -> HashMap<String, usize> {
    // La méthode 'fold' remplace la boucle 'for' et gère l'accumulation dans la HashMap.
    texte
        .split_whitespace() // 1. Itérer sur les mots bruts séparés par des espaces.
        .filter_map(|mot_brut| {
            // 2. Nettoyage et normalisation.
            let mot_nettoye = mot_brut
                // Supprime la ponctuation courante ('!', '?', '.', ',', etc.) aux bords.
                .trim_matches(|c: char| c.is_ascii_punctuation())
                .to_lowercase();

            // 3. Filtrer les résultats vides après nettoyage (e.g. un mot était juste "!!!")
            if mot_nettoye.is_empty() {
                None // Équivalent de 'continue' dans une boucle for.
            } else {
                // Retourne la clé (String). L'ownership de cette nouvelle String est prêt à être transféré.
                Some(mot_nettoye)
            }
        })
        // 4. Accumuler les résultats dans une HashMap.
        .fold(HashMap::new(), |mut compteurs, mot_nettoye| {
            // Utilisation idiomatique de l'Entry API : O(1) pour la recherche ET la mise à jour.
            // .entry(mot_nettoye) prend l'ownership de la clé (pas de copie supplémentaire).
            // .or_insert(0) retourne une référence mutable à la valeur (le compte).
            *compteurs.entry(mot_nettoye).or_insert(0) += 1;

            compteurs // Retourne la HashMap mise à jour pour la prochaine itération.
        })
}

fn main() {
    let corpus = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nunc vestibulum imperdiet massa nec facilisis. Donec luctus pulvinar leo sed rhoncus. Etiam bibendum, nibh vel finibus condimentum, quam leo porttitor ligula, sit amet laoreet lorem mauris non massa. Sed euismod eget mi in ultricies. Vivamus facilisis mauris lorem, ut pellentesque ligula porta nec. Curabitur malesuada pretium scelerisque. Pellentesque habitant morbi tristique senectus et netus et malesuada fames ac turpis egestas. Aenean tincidunt, erat nec consectetur imperdiet, lorem nibh dignissim odio, nec rutrum est ipsum nec magna. Maecenas venenatis nulla ut laoreet posuere.

In venenatis aliquam odio finibus semper. Ut ac neque massa. Cras quam magna, placerat non blandit a, viverra sit amet metus. Phasellus arcu lectus, dignissim ac eros a, semper eleifend erat. Proin vehicula blandit sapien, nec elementum arcu vehicula et. Duis non nibh semper, gravida massa id, sollicitudin velit. Aliquam erat volutpat.

Ut vulputate consequat dapibus. Vivamus sit amet orci et nisl vulputate feugiat a in odio. Proin facilisis, nisl sed sodales convallis, ex erat venenatis ipsum, vitae convallis risus dui nec eros. Donec commodo mauris purus. Mauris sit amet fermentum erat, at tempus metus. Curabitur massa leo, sodales et quam sed, convallis scelerisque tellus. Proin vel ornare risus. Pellentesque id urna lorem.

Donec sagittis turpis quis felis commodo suscipit. Integer vehicula mi augue, ut ornare ante iaculis quis. Aenean dolor orci, dapibus ut justo eget, finibus cursus velit. Integer dictum felis lectus, ac vulputate justo egestas vitae. Sed sodales magna non nibh malesuada, tincidunt tristique ipsum pretium. Pellentesque pulvinar varius odio a imperdiet. Vivamus condimentum sem at dictum vehicula. Vivamus interdum mauris libero, a molestie nibh fringilla eget. Nulla sollicitudin erat ac neque interdum, ut elementum orci consequat. In hac habitasse platea dictumst. Aenean posuere semper laoreet. Pellentesque consectetur pellentesque sem, malesuada luctus diam sagittis ut. Sed nec imperdiet ex. Nunc tincidunt bibendum diam, vel sodales dolor feugiat quis. Cras fermentum suscipit nisi porttitor condimentum. Ut mattis justo ut libero cursus consequat.

Proin iaculis vestibulum fermentum. Aliquam sit amet nisi risus. Morbi ultrices lacus sit amet nisl finibus, non auctor risus suscipit. Quisque efficitur rhoncus auctor. Nullam gravida molestie odio, sit amet mattis lectus lobortis non. Etiam vel condimentum nisi, quis rhoncus lorem. Phasellus feugiat a turpis elementum interdum. Fusce sit amet cursus est. Donec vehicula tincidunt turpis, quis blandit neque fringilla at. Sed fringilla pretium porta. Aliquam eget ultrices risus. In non ultricies elit.";

    // Appel de la fonction optimisée
    let frequences = compter_frequences_optimise(corpus);

    println!("==========================================");
    println!("  RÉSULTAT DU COMPTEUR DE FRÉQUENCE OPTIMISÉ");
    println!("==========================================");

    // Affichage final ordonné
    let mut resultat_trie: Vec<(&String, &usize)> = frequences.iter().collect();
    resultat_trie.sort_by_key(|a| a.1);

    for (mot, compte) in resultat_trie {
        println!("{:<10} : {}", mot, compte);
    }
}