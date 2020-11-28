mod tents;

fn main() {
    let game = tents::Game::new("src/tents.txt".to_string());
    println!("{:?}", game);
}
