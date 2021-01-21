mod dpll;
//use plotters::prelude::*;

fn main() {
    //solve("inputs/test/unsat/op5.cnf");
    let paths = std::fs::read_dir("inputs/test/unsat").unwrap();
    for path in paths {
        let path_str = &format!("{}", path.unwrap().path().display());
        if path_str.ends_with(".cnf") {
            println!("{}", path_str);
            solve(path_str);
        }
    }
}

fn solve(path: &str) {
    let mut solver = dpll::DPLL::new(path, 4);
    solver.dpll();
    if solver.unsat {
        println!("s UNSATISFIABLE");
    } else {
        let mut sol = Vec::<i32>::new();
        let mut sol_str = String::from("s SATISFIABLE\nv");
        for i in 0..solver.variables.len() {
            if solver.variables[i].value == 1 {
                sol.push(i as i32 + 1);
            } else {
                sol.push(-(i as i32) - 1);
            }
            sol_str.push_str(&format!(" {}", sol[sol.len() - 1]));
        }
        sol_str.push_str(" 0");
        println!("{}", sol_str);
        if solver.validate() {
            println!("Correct!");
        } else {
            println!("Incorrect!");
        }
    }
}
