use std::io;
use std::io::Write;
use rand::Rng;

struct Score {
    partie: u32,
    points: u32,
    combinaison: String,
    lancers: u32,
}

fn main() {
    println!("Bienvenue dans le jeu du 421 !");

    let max_lancers = demander_nombre_lancers();
    println!("Vous avez choisi {} lancer(s) par partie.", max_lancers);

    let mut scores: Vec<Score> = Vec::new();
    let mut score_total = 0;
    let mut numero_partie = 1;
    let mut total_lancers = 0;

    loop {
        println!("\nPartie {} :", numero_partie);
        let (points, combinaison, lancers) = jouer_partie(max_lancers);  // Modifiez cette ligne
        score_total += points;
        total_lancers += lancers;

        scores.push(Score {
            partie: numero_partie,
            points,
            combinaison,
            lancers,
        });

        println!("Points de cette partie : {}", points);
        println!("Score total : {}", score_total);

        if !demander_rejouer() {
            break;
        }

        numero_partie += 1;
    }

    afficher_tableau_scores(&scores, score_total, total_lancers);
    println!("Merci d'avoir joué !");
}
fn demander_nombre_lancers() -> u32 {
    loop {
        print!("Combien de lancers voulez-vous par partie ? (1-15) : ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim().parse() {
            Ok(n) if n >= 1 && n <= 15 => return n,
            _ => println!("Veuillez entrer un nombre entre 1 et 15."),
        }
    }
}
fn jouer_partie(max_lancers: u32) -> (u32, String, u32) {
    let mut des = [0, 0, 0];
    let mut meilleur_score = 0;
    let mut meilleure_combinaison = String::new();
    let mut lancers = 0;

    while lancers < max_lancers {
        lancers += 1;
        lancer_des(&mut des);
        afficher_des(&des);

        let points = calculer_points(&des);
        let combinaison = format!("{} {} {}", des[0], des[1], des[2]);

        println!("Lancer {} : {} points", lancers, points);

        if points > meilleur_score {
            meilleur_score = points;
            meilleure_combinaison = combinaison.clone();
        }

        if points == 10 {  // Si on obtient 421, on arrête immédiatement
            println!("Bravo ! Vous avez obtenu 421 !");
            return (points, combinaison, lancers);
        }

        if lancers < max_lancers && !demander_relancer(lancers, max_lancers) {
            break;
        }
    }

    println!("Votre meilleur score : {} points avec la combinaison {}", meilleur_score, meilleure_combinaison);
    (meilleur_score, meilleure_combinaison, lancers)
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

fn demander_relancer(lancers_actuels: u32, max_lancers: u32) -> bool {
    print!("Voulez-vous relancer ? (o/n) [Il vous reste {} lancer(s)] : ", max_lancers - lancers_actuels);
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

fn afficher_tableau_scores(scores: &[Score], score_total: u32, total_lancers: u32) {
println!("\nTableau des scores :");
println!("+---------+--------+-------------+----------+");
println!("| Partie  | Points | Combinaison | Lancers  |");
println!("+---------+--------+-------------+----------+");

for score in scores {
println!("| {: <7} | {: <6} | {: <11} | {: <8} |", score.partie, score.points, score.combinaison, score.lancers);
}

println!("+---------+--------+-------------+----------+");
println!("| Total   | {: <6} |             | {: <8} |", score_total, total_lancers);
println!("+---------+--------+-------------+----------+");

let nombre_parties = scores.len() as f32;
let moyenne_points = score_total as f32 / nombre_parties;
let moyenne_lancers = total_lancers as f32 / nombre_parties;

println!("\nStatistiques :");
println!("Nombre de parties : {}", scores.len());
println!("Moyenne de points par partie : {:.2}", moyenne_points);
println!("Moyenne de lancers par partie : {:.2}", moyenne_lancers);

if let Some(meilleur_score) = scores.iter().max_by_key(|s| s.points) {
println!("Meilleur score : {} points (partie {})", meilleur_score.points, meilleur_score.partie);
}
}