mod tents;
use std::time:: Instant;




fn main() {
    let start = Instant::now();

    let mut sat_maker = tents::SatMaker::new("src/tents.txt");
    let mut duration = start.elapsed();
    //println!("encoding time: {:?}", duration);
    sat_maker.solve_sat();
    //sat_maker.unique_check();
    let duration1 = start.elapsed();
    //println!("solving time: {:?}", duration1-duration);
}

