use std::io::stdin;
use std::str::FromStr;

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

fn main() {
    let i = collect_input::<u32>("Enter a positive number: ");
    println!("{}", i);
}
