mod tents;
//use std::time::Instant;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    //println!("{:?}", args);
    //let start = Instant::now();
    let mut sat_maker = tents::SatMaker::new(&args[1]);
    //let mut duration = start.elapsed();
    //println!("encoding time: {:?}", duration);
    sat_maker.solve_sat();
    //println!("solving time: {:?}", duration1-duration);
    if args.contains(&"unique".to_string()) {
        sat_maker.unique_check();
    }
    //let duration1 = start.elapsed();
}
