mod tents;
use std::time:: Instant;



fn main() {
    let a = ['A', 'B', 'C'];
    let b = [1, 4];
    let c = [true, false];
    let d = ['x', 'y'];

    for (a, b, c, d) in iproduct!(&a, &b, &c, &d) {
        //println!("{} {} {} {}", a, b, c, d);
    }

    let start = Instant::now();

    let mut sat_maker = tents::SatMaker::new("src/tents.txt");
    let mut duration = start.elapsed();
    //println!("Time elapsed in expensive_function() is: {:?}", duration);
    sat_maker.solve_sat();
    duration = start.elapsed();
    //println!("Time elapsed in expensive_function() is: {:?}", duration);
}

