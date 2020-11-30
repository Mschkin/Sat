mod tents;

fn main() {
    let sat_maker=tents::SatMaker::new("src/tents.txt");
    sat_maker.solve_sat();
}
