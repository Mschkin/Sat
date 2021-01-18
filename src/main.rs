mod dpll;

fn main() {
    let paths = std::fs::read_dir("inputs/sat").unwrap();
    for path in paths {
        let path_str = &format!("{}", path.unwrap().path().display());
        println!("{}", path_str);
        solve(path_str);
    }
}
    
fn solve(path: &str) {
    let mut solver = dpll::DPLL::new(path);
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
