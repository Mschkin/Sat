mod tents;
use std::time::Instant;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    //println!("{:?}", args);
    let start = Instant::now();
    let mut sat_maker:tents::SatMaker;
    if args.len() == 1 {
        sat_maker = tents::SatMaker::new("src/tents.txt");
    } else {
        sat_maker = tents::SatMaker::new(&args[1]);
    }
    let mut duration = start.elapsed();
    println!("encoding time: {:?}", duration);
    sat_maker.solve_sat();
    let duration1 = start.elapsed();
    println!("solving time: {:?}", duration1-duration);
    if args.contains(&"unique".to_string()) {
        sat_maker.unique_check();
    }
}
