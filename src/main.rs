// mod tents;
// use std::time::Instant;
// use std::env;

// fn main() {
//     let args: Vec<String> = env::args().collect();
//     //println!("{:?}", args);
//     let start = Instant::now();
//     let mut sat_maker:tents::SatMaker;
//     if args.len() == 1 {
//         sat_maker = tents::SatMaker::new("src/tents.txt");
//     } else {
//         sat_maker = tents::SatMaker::new(&args[1]);
//     }
//     let mut duration = start.elapsed();
//     println!("encoding time: {:?}", duration);
//     sat_maker.solve_sat();
//     let duration1 = start.elapsed();
//     println!("solving time: {:?}", duration1-duration);
//     if args.contains(&"unique".to_string()) {
//         sat_maker.unique_check();
//     }
// }

mod dpll;
fn main() {
    let paths = std::fs::read_dir("inputs/test/sat").unwrap();
    for path in paths {
        let path_str = &format!("{}", path.unwrap().path().display());
        println!("{}", path_str);
        let mut solver = dpll::DPLL::new(path_str);
        let res = solver.dpll();
        let mut sol = Vec::<i32>::new();
        for i in 0..res.len() {
            if res[i].value == 1 {
                sol.push(i as i32 + 1);
            } else {
                sol.push(-(i as i32) - 1);
            }
        }
        println!("{:?}", sol);
        if solver.validate() {
            println!("Correct!");
        } else {
            println!("Incorrect!");
        }
    }
}
