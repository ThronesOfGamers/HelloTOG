use std::io;
use std::io::Write;
use rand::Rng;

fn main() {
    println!("Bienvenue dans le jeu du 421 !");

    let mut score_total = 0;

    loop {
        let points = jouer_partie();
        score_total += points;
        println!("Points de cette partie : {}", points);
        println!("Score total : {}", score_total);

        if !demander_rejouer() {
            break;
        }
    }

    println!("Partie terminée. Score final : {}", score_total);
    println!("Merci d'avoir joué !");
}

fn jouer_partie() -> u32 {
    let mut des = [0, 0, 0];
    let mut lancers = 0;

    while lancers < 3 {
        lancer_des(&mut des);
        afficher_des(&des);

        let points = calculer_points(&des);
        if points > 1 {
            println!("Vous avez marqué {} points !", points);
            return points;
        }

        if lancers < 2 && demander_relancer() {
            lancers += 1;
        } else {
            break;
        }
    }

    println!("Vous marquez 1 point.");
    1
}

fn lancer_des(des: &mut [u8; 3]) {
    let mut rng = rand::thread_rng();
    for de in des.iter_mut() {
        *de = rng.gen_range(1..=6);
    }
}

fn afficher_des(des: &[u8; 3]) {
    println!("Résultat du lancer : {} {} {}", des[0], des[1], des[2]);
}

fn calculer_points(des: &[u8; 3]) -> u32 {
    let mut sorted_des = des.to_vec();
    sorted_des.sort_unstable();

    match sorted_des[..] {
        [1, 2, 4] => 10, // 421
        [1, 1, 1] => 7,  // 111
        [1, 1, 6] => 6,  // 611
        [6, 6, 6] => 6,  // 666
        [1, 1, 5] => 5,  // 511
        [5, 5, 5] => 5,  // 555
        _ => 1,          // Autres combinaisons
    }
}

fn demander_relancer() -> bool {
    print!("Voulez-vous relancer ? (o/n) : ");
    io::stdout().flush().unwrap();

    let mut reponse = String::new();
    io::stdin().read_line(&mut reponse).unwrap();

    reponse.trim().to_lowercase() == "o"
}

fn demander_rejouer() -> bool {
    print!("Voulez-vous jouer une autre partie ? (o/n) : ");
    io::stdout().flush().unwrap();

    let mut reponse = String::new();
    io::stdin().read_line(&mut reponse).unwrap();

    reponse.trim().to_lowercase() == "o"
}