mod tents;
use std::time:: Instant;



fn main() {
    

    let start = Instant::now();

    let mut sat_maker = tents::SatMaker::new("src/tents.txt");
    let mut duration = start.elapsed();
    println!("Time elapsed in expensive_function() is: {:?}", duration);
    sat_maker.solve_sat();
    duration = start.elapsed();
    println!("Time elapsed in expensive_function() is: {:?}", duration);
}

