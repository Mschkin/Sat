mod dpll;
use plotters::prelude::*;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        compare_heuristics();
    }else if args.len() == 2{
        benchmark(args[1].parse::<usize>().unwrap());
    }else{
        if args[1].ends_with(".cnf"){
            solve(args[1], args[2].parse::<usize>().unwrap());
        }else{ // folder
            solve_all(args[1], args[2].parse::<usize>().unwrap());
        }
    }
    //solve("inputs/test/unsat/nop2.cnf", args[1].parse::<usize>().unwrap());
    //solve_all("inputs/test/sat", args[1].parse::<usize>().unwrap());  
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

fn solve_all(path: &str, heuristic: usize) {
    let paths = std::fs::read_dir(path).unwrap();
    for path in paths {
        let path_str = &format!("{}", path.unwrap().path().display());
        if path_str.ends_with(".cnf") {
            println!("{}", path_str);
            solve(path_str, heuristic);
        }
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
    let mut ii_tup = Vec::<(i32, i32)>::new();
    let mut par_tup = Vec::<(i32, i32)>::new();
    let mut ssa_tup = Vec::<(i32, i32)>::new();
    let mut uf50_tup = Vec::<(i32, i32)>::new();
    let mut hole_tup = Vec::<(i32, i32)>::new();
    let mut pret_tup = Vec::<(i32, i32)>::new();
    sum_time(aim_time, &mut aim_tup);
    sum_time(ii_time, &mut ii_tup);
    sum_time(par_time, &mut par_tup);
    sum_time(ssa_time, &mut ssa_tup);
    sum_time(uf50_time, &mut uf50_tup);
    sum_time(hole_time, &mut hole_tup);
    sum_time(pret_time, &mut pret_tup);
    //println!("{:?}", aim_tup);
    //println!("{:?}", ii_tup);
    //println!("{:?}", par_tup);
    //println!("{:?}", ssa_tup);
    //println!("{:?}", uf50_tup);
    //println!("{:?}", hole_tup);
    //println!("{:?}", pret_tup);

    plot(
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

fn compare_heuristics() {
    let mut dlis_time = Vec::<i32>::new();
    let mut dlcs_time = Vec::<i32>::new();
    let mut moms_time = Vec::<i32>::new();
    let mut jw_time = Vec::<i32>::new();
    let mut boehm_time = Vec::<i32>::new();
    get_time(&mut dlis_time, 0);
    get_time(&mut dlcs_time, 1);
    get_time(&mut moms_time, 2);
    get_time(&mut jw_time, 3);
    get_time(&mut boehm_time, 4);
    dlis_time.sort_by(|a, b| a.partial_cmp(b).unwrap());
    dlcs_time.sort_by(|a, b| a.partial_cmp(b).unwrap());
    moms_time.sort_by(|a, b| a.partial_cmp(b).unwrap());
    jw_time.sort_by(|a, b| a.partial_cmp(b).unwrap());
    boehm_time.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let mut dlis_tup = Vec::<(i32, i32)>::new();
    let mut dlcs_tup = Vec::<(i32, i32)>::new();
    let mut moms_tup = Vec::<(i32, i32)>::new();
    let mut jw_tup = Vec::<(i32, i32)>::new();
    let mut boehm_tup = Vec::<(i32, i32)>::new();
    sum_time(dlis_time, &mut dlis_tup);
    sum_time(dlcs_time, &mut dlcs_tup);
    sum_time(moms_time, &mut moms_tup);
    sum_time(jw_time, &mut jw_tup);
    sum_time(boehm_time, &mut boehm_tup);

    // println!("{:?}", dlis_tup);
    // println!("{:?}", dlcs_tup);
    // println!("{:?}", moms_tup);
    // println!("{:?}", jw_tup);
    // println!("{:?}", boehm_tup);

    plot_compare(
        dlis_tup,
        dlcs_tup,
        moms_tup,
        jw_tup,
        boehm_tup,
    );
}

fn get_time(vec: &mut Vec<i32>, heuristic: usize) {
    let mut paths = std::fs::read_dir("inputs/sat").unwrap();
    for path in paths {
        let path_str = &format!("{}", path.unwrap().path().display());
        if path_str.ends_with(".cnf") {
            println!("{}", path_str);
            let sol = solve(path_str, heuristic);
            if sol.0 {
                vec.push(sol.1 as i32);
            }
        }
    }
    paths = std::fs::read_dir("inputs/unsat").unwrap();
    for path in paths {
        let path_str = &format!("{}", path.unwrap().path().display());
        if path_str.ends_with(".cnf") {
            println!("{}", path_str);
            let sol = solve(path_str, heuristic);
            if sol.0 {
                vec.push(sol.1 as i32);
            }
        }
    }
}

fn sum_time(time_vec: Vec<i32>, tup_vec: &mut Vec<(i32, i32)>) {
    let mut total_time = 0;
    for i in 0..time_vec.len() {
        total_time += time_vec[i];
        tup_vec.push((i as i32, total_time / 1000));
    }
}

fn plot(
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

fn plot_compare(
    dlis: Vec<(i32, i32)>,
    dlcs: Vec<(i32, i32)>,
    moms: Vec<(i32, i32)>,
    jw: Vec<(i32, i32)>,
    boehm: Vec<(i32, i32)>
) {
    let root_area = BitMapBackend::new("compare.png", (800, 500)).into_drawing_area();
    root_area.fill(&WHITE).unwrap();

    let mut ctx = ChartBuilder::on(&root_area)
        .caption(
            "Compare heuristics",
            ("sans-serif", 30),
        )
        .margin(10)
        .x_label_area_size(30)
        .y_label_area_size(40)
        .build_cartesian_2d(0..400, 0..150000)
        .unwrap();

    ctx.configure_mesh().draw().unwrap();

    ctx.draw_series(
        dlis.iter()
            .map(|point| TriangleMarker::new(*point, 5, &BLUE)),
    )
    .unwrap()
    .label("DLIS")
    .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));

    ctx.draw_series(dlcs.iter().map(|point| TriangleMarker::new(*point, 5, &RED)))
        .unwrap()
        .label("DLCS")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    ctx.draw_series(
        moms.iter()
            .map(|point| TriangleMarker::new(*point, 5, &GREEN)),
    )
    .unwrap()
    .label("MOM")
    .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &GREEN));

    ctx.draw_series(
        jw.iter()
            .map(|point| TriangleMarker::new(*point, 5, &BLACK)),
    )
    .unwrap()
    .label("Jeroslaw-Wang")
    .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLACK));

    ctx.draw_series(
        boehm.iter()
            .map(|point| TriangleMarker::new(*point, 5, &CYAN)),
    )
    .unwrap()
    .label("Böhm")
    .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &CYAN));

    ctx.configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()
        .unwrap();
}

