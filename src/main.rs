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

fn main() {
    let players = collect_players();
    println!("{:?}", players);
}
