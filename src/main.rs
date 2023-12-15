use rand::{thread_rng, Rng};
use std::io::stdin;
use std::str::FromStr;

#[derive(Debug)]
struct Player {
    name: String,
    score: u32,
}

trait Printable {
    fn to_string(&self) -> String;
}

impl Printable for Player {
    fn to_string(&self) -> String {
        format!("{} ({})", self.name, self.score)
    }
}

fn collect_input<T: FromStr>(prompt: &str) -> T {
    loop {
        println!("{}", prompt);
        let mut input = String::new();
        stdin().read_line(&mut input).expect("Failed to read input");
        match input.trim().parse() {
            Ok(value) => return value,
            Err(_) => continue,
        }
    }
}

fn collect_players() -> Vec<Player> {
    let mut players = Vec::new();
    let mut num_players;

    loop {
        num_players = collect_input::<u32>("How many players (>= 2)?");
        if num_players < 2 {
            println!("Invalid ❌ no.! Please try again!");
            continue;
        } else {
            break;
        }
    }

    for i in 1..=num_players {
        let name = collect_input(format!("Player {} name:", i).as_str());
        let score = 0;
        players.push(Player { name, score })
    }

    players
}

fn create_max_range(players: &Vec<Player>) -> u32 {
    return players.len() as u32 * 50;
}
fn generate_number(max_range: u32) -> u32 {
    thread_rng().gen_range(1..max_range)
}

fn collect_guesses_into_proximities(players: &Vec<Player>, max_range: u32) -> Vec<(String, u32)> {
    let mut player_proximities = Vec::<(String, u32)>::new();
    let target = generate_number(create_max_range(players));

    for player in players {
        println!("{}'s turn", player.name);
        let guess = collect_input::<u32>(&format!("Guess the number (1-{max_range}):"));
        player_proximities.push((player.name.clone(), guess.abs_diff(target)));
    }

    player_proximities
}

fn get_winner(player_proximities: &Vec<(String, u32)>) -> String {
    player_proximities[0].0.to_owned()
}

fn update_scores(players: &mut Vec<Player>, winner: &str) {
    for player in players {
        if player.name == winner {
            player.score += 1
        }
    }
}

fn print_scores(players: &Vec<Player>) {
    println!("Scores:");
    for player in players {
        println!("- {}", player.to_string());
    }
}

fn main() {
    let players = collect_players();
    println!("{:?}", players);
}
