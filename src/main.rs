//mod tents;

fn main() {
    let s = read_file("src/tents.txt".to_string());
    let k = s.split_whitespace();
    for i in k {
        println!("{}  ", i);
    }
}

fn read_file(path: String) -> String {
    use std::io::Read;
    use std;
    let mut file = std::fs::File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}
