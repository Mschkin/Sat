mod dpll;
use plotters::prelude::*;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    //solve("inputs/test/unsat/nop2.cnf", args[1].parse::<usize>().unwrap());
    let mut paths = std::fs::read_dir("inputs/test/sat").unwrap();
    for path in paths {
        let path_str = &format!("{}", path.unwrap().path().display());
        if path_str.ends_with(".cnf") {
            println!("{}", path_str);
            solve(path_str, args[1].parse::<usize>().unwrap());
        }
    }
    paths = std::fs::read_dir("inputs/test/unsat").unwrap();
    for path in paths {
        let path_str = &format!("{}", path.unwrap().path().display());
        if path_str.ends_with(".cnf") {
            println!("{}", path_str);
            solve(path_str, args[1].parse::<usize>().unwrap());
        }
    }
    benchmark(args[1].parse::<usize>().unwrap());
}

fn solve(path: &str, heuristic: usize) -> (bool, u128) {
    let mut solver = dpll::DPLL::new(path, heuristic);
    solver.dpll();
    if solver.unsat {
        println!("s UNSATISFIABLE");
        println!("{:?}", solver.duration);
        (true, solver.duration.as_micros())
    } else if solver.solved {
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
        // if solver.validate() {
        //     println!("Correct!");
        // } else {
        //     println!("Incorrect!");
        // }
        println!("{:?}", solver.duration);
        (true, solver.duration.as_micros())
    } else {
        println!("Timeout!");
        (false, 0)
    }
}

fn benchmark(heuristic: usize) {
    let mut paths = std::fs::read_dir("inputs/sat").unwrap();
    let mut aim_time = Vec::<i32>::new();
    let mut ii_time = Vec::<i32>::new();
    let mut par_time = Vec::<i32>::new();
    let mut ssa_time = Vec::<i32>::new();
    let mut uf50_time = Vec::<i32>::new();
    let mut hole_time = Vec::<i32>::new();
    let mut pret_time = Vec::<i32>::new();
    let mut solved_count = 0;
    let mut timeout_count = 0;
    for path in paths {
        let path_str = &format!("{}", path.unwrap().path().display());
        if path_str.contains("aim") {
            println!("{}", path_str);
            let sol = solve(path_str, heuristic);
            if sol.0 {
                aim_time.push(sol.1 as i32);
                solved_count += 1;
            } else {
                timeout_count += 1;
            }
        } else if path_str.contains("ii") {
            println!("{}", path_str);
            let sol = solve(path_str, heuristic);
            if sol.0 {
                ii_time.push(sol.1 as i32);
                solved_count += 1;
            } else {
                timeout_count += 1;
            }
        } else if path_str.contains("par") {
            println!("{}", path_str);
            let sol = solve(path_str, heuristic);
            if sol.0 {
                par_time.push(sol.1 as i32);
                solved_count += 1;
            } else {
                timeout_count += 1;
            }
        } else if path_str.contains("ssa") {
            println!("{}", path_str);
            let sol = solve(path_str, heuristic);
            if sol.0 {
                ssa_time.push(sol.1 as i32);
                solved_count += 1;
            } else {
                timeout_count += 1;
            }
        } else if path_str.contains("uf50") {
            println!("{}", path_str);
            let sol = solve(path_str, heuristic);
            if sol.0 {
                uf50_time.push(sol.1 as i32);
                solved_count += 1;
            } else {
                timeout_count += 1;
            }
        } else if path_str.contains("hole") {
            println!("{}", path_str);
            let sol = solve(path_str, heuristic);
            if sol.0 {
                hole_time.push(sol.1 as i32);
                solved_count += 1;
            } else {
                timeout_count += 1;
            }
        } else if path_str.contains("pret") {
            println!("{}", path_str);
            let sol = solve(path_str, heuristic);
            if sol.0 {
                pret_time.push(sol.1 as i32);
                solved_count += 1;
            } else {
                timeout_count += 1;
            }
        }
    }
    paths = std::fs::read_dir("inputs/unsat").unwrap();
    for path in paths {
        let path_str = &format!("{}", path.unwrap().path().display());
        if path_str.contains("aim") {
            println!("{}", path_str);
            let sol = solve(path_str, heuristic);
            if sol.0 {
                aim_time.push(sol.1 as i32);
                solved_count += 1;
            } else {
                timeout_count += 1;
            }
        } else if path_str.contains("ii") {
            println!("{}", path_str);
            let sol = solve(path_str, heuristic);
            if sol.0 {
                ii_time.push(sol.1 as i32);
                solved_count += 1;
            } else {
                timeout_count += 1;
            }
        } else if path_str.contains("par") {
            println!("{}", path_str);
            let sol = solve(path_str, heuristic);
            if sol.0 {
                par_time.push(sol.1 as i32);
                solved_count += 1;
            } else {
                timeout_count += 1;
            }
        } else if path_str.contains("ssa") {
            println!("{}", path_str);
            let sol = solve(path_str, heuristic);
            if sol.0 {
                ssa_time.push(sol.1 as i32);
                solved_count += 1;
            } else {
                timeout_count += 1;
            }
        } else if path_str.contains("uf50") {
            println!("{}", path_str);
            let sol = solve(path_str, heuristic);
            if sol.0 {
                uf50_time.push(sol.1 as i32);
                solved_count += 1;
            } else {
                timeout_count += 1;
            }
        } else if path_str.contains("hole") {
            println!("{}", path_str);
            let sol = solve(path_str, heuristic);
            if sol.0 {
                hole_time.push(sol.1 as i32);
                solved_count += 1;
            } else {
                timeout_count += 1;
            }
        } else if path_str.contains("pret") {
            println!("{}", path_str);
            let sol = solve(path_str, heuristic);
            if sol.0 {
                pret_time.push(sol.1 as i32);
                solved_count += 1;
            } else {
                timeout_count += 1;
            }
        }
    }
    aim_time.sort_by(|a, b| a.partial_cmp(b).unwrap());
    ii_time.sort_by(|a, b| a.partial_cmp(b).unwrap());
    par_time.sort_by(|a, b| a.partial_cmp(b).unwrap());
    ssa_time.sort_by(|a, b| a.partial_cmp(b).unwrap());
    uf50_time.sort_by(|a, b| a.partial_cmp(b).unwrap());
    hole_time.sort_by(|a, b| a.partial_cmp(b).unwrap());
    pret_time.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let mut aim_tup = Vec::<(i32, i32)>::new();
    let mut total_time = 0;
    for i in 0..aim_time.len() {
        total_time += aim_time[i];
        aim_tup.push((i as i32, total_time / 1000));
    }
    total_time = 0;
    let mut ii_tup = Vec::<(i32, i32)>::new();
    for i in 0..ii_time.len() {
        total_time += ii_time[i];
        ii_tup.push((i as i32, total_time / 1000));
    }
    total_time = 0;
    let mut par_tup = Vec::<(i32, i32)>::new();
    for i in 0..par_time.len() {
        total_time += par_time[i];
        par_tup.push((i as i32, total_time / 1000));
    }
    total_time = 0;
    let mut ssa_tup = Vec::<(i32, i32)>::new();
    for i in 0..ssa_time.len() {
        total_time += ssa_time[i];
        ssa_tup.push((i as i32, total_time / 1000));
    }
    total_time = 0;
    let mut uf50_tup = Vec::<(i32, i32)>::new();
    for i in 0..uf50_time.len() {
        total_time += uf50_time[i];
        uf50_tup.push((i as i32, total_time / 1000));
    }
    total_time = 0;
    let mut hole_tup = Vec::<(i32, i32)>::new();
    for i in 0..hole_time.len() {
        total_time += hole_time[i];
        hole_tup.push((i as i32, total_time / 1000));
    }
    total_time = 0;
    let mut pret_tup = Vec::<(i32, i32)>::new();
    for i in 0..pret_time.len() {
        total_time += pret_time[i];
        pret_tup.push((i as i32, total_time / 1000));
    }
    //println!("{:?}", aim_tup);
    //println!("{:?}", ii_tup);
    //println!("{:?}", par_tup);
    //println!("{:?}", ssa_tup);
    //println!("{:?}", uf50_tup);
    //println!("{:?}", hole_tup);
    //println!("{:?}", pret_tup);
    ploter(
        aim_tup,
        ii_tup,
        par_tup,
        ssa_tup,
        uf50_tup,
        hole_tup,
        pret_tup,
        heuristic,
        solved_count,
        timeout_count,
    );
    println!(
        "solved problems: {}  timeout: {}",
        solved_count, timeout_count
    );
}

fn ploter(
    aim: Vec<(i32, i32)>,
    ii: Vec<(i32, i32)>,
    par: Vec<(i32, i32)>,
    ssa: Vec<(i32, i32)>,
    uf50: Vec<(i32, i32)>,
    hole: Vec<(i32, i32)>,
    pret: Vec<(i32, i32)>,
    heuristic: usize,
    solved_count: usize,
    timeout_count: usize,
) {
    let heuristics = vec!["DLIS", "DLCS", "Jeroslaw-Wang", "MOM", "Böhm"];
    let caption = format!("{}.png", heuristics[heuristic]);
    let root_area = BitMapBackend::new(&caption, (800, 500)).into_drawing_area();
    root_area.fill(&WHITE).unwrap();

    let mut ctx = ChartBuilder::on(&root_area)
        .caption(
            format!(
                "{} - solved: {}  timeout: {}",
                heuristics[heuristic], solved_count, timeout_count
            ),
            ("sans-serif", 30),
        )
        .margin(10)
        .x_label_area_size(30)
        .y_label_area_size(40)
        .build_cartesian_2d(0..80, 0..150000)
        .unwrap();

    ctx.configure_mesh().draw().unwrap();

    ctx.draw_series(
        aim.iter()
            .map(|point| TriangleMarker::new(*point, 5, &BLUE)),
    )
    .unwrap()
    .label("aim")
    .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));

    ctx.draw_series(ii.iter().map(|point| TriangleMarker::new(*point, 5, &RED)))
        .unwrap()
        .label("ii")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));
    ctx.draw_series(
        par.iter()
            .map(|point| TriangleMarker::new(*point, 5, &GREEN)),
    )
    .unwrap()
    .label("par")
    .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &GREEN));

    ctx.draw_series(
        ssa.iter()
            .map(|point| TriangleMarker::new(*point, 5, &YELLOW)),
    )
    .unwrap()
    .label("ssa")
    .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &YELLOW));

    ctx.draw_series(
        uf50.iter()
            .map(|point| TriangleMarker::new(*point, 5, &BLACK)),
    )
    .unwrap()
    .label("uf50")
    .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLACK));

    ctx.draw_series(
        hole.iter()
            .map(|point| TriangleMarker::new(*point, 5, &CYAN)),
    )
    .unwrap()
    .label("hole")
    .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &CYAN));

    ctx.draw_series(
        pret.iter()
            .map(|point| TriangleMarker::new(*point, 5, &MAGENTA)),
    )
    .unwrap()
    .label("pret")
    .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &MAGENTA));

    ctx.configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()
        .unwrap();
}
