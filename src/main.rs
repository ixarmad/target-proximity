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

fn main() {}
