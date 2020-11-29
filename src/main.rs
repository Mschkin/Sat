mod tents;

fn main() {
    let mut test=tents::SatMaker::new("src/tents.txt");
    test.exactly_n(3,vec![1,2,3,4,5,6,7,8,9,10]);
    println!("{}",test.clauses);
}
