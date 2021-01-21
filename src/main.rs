mod dpll;
use plotters::prelude::*;


fn main() {
    //solve("inputs/test/unsat/op5.cnf");
    // let paths = std::fs::read_dir("inputs/sat").unwrap();
    // for path in paths {
    //     let path_str = &format!("{}", path.unwrap().path().display());
    //     if path_str.ends_with(".cnf") {
    //         println!("{}", path_str);
    //         solve(path_str);
    //     }
    // }
    benchmark();
}

fn solve(path: &str)->(bool,u128) {
    let mut solver = dpll::DPLL::new(path, 4);
    solver.dpll();
    if solver.unsat {
        println!("s UNSATISFIABLE");
        println!("{:?}", solver.duration);
        (true,solver.duration.as_nanos())
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
        if solver.validate() {
            println!("Correct!");
        } else {
            println!("Incorrect!");
        }
        println!("{:?}", solver.duration);
        (true,solver.duration.as_nanos())
    } else {
        println!("Timeout!");
        (false,0)
    }
}

fn benchmark(){
    let mut paths = std::fs::read_dir("inputs/sat").unwrap();
    //paths.append(&mut std::fs::read_dir("inputs/unsat").unwrap());
    let mut aim_time=Vec::<f64>::new();
    let mut ii_time=Vec::<f64>::new();
    let mut par_time=Vec::<f64>::new();
    let mut ssa_time=Vec::<f64>::new();
    let mut uf50_time=Vec::<f64>::new();
    for path in paths {
        let path_str = &format!("{}", path.unwrap().path().display());
        if path_str.contains("aim") {
            println!("{}", path_str);
            let sol=solve(path_str);
            if sol.0{
                aim_time.push(sol.1 as f64/1000000.);
            }
        } else if path_str.contains("ii") {
            println!("{}", path_str);
            let sol=solve(path_str);
            if sol.0{
                ii_time.push(sol.1 as f64/1000000.);
            }
        }else if path_str.contains("par") {
            println!("{}", path_str);
            let sol=solve(path_str);
            if sol.0{
                par_time.push(sol.1 as f64/1000000.);
            }
        }else if path_str.contains("ssa") {
            println!("{}", path_str);
            let sol=solve(path_str);
            if sol.0{
                ssa_time.push(sol.1 as f64/1000000.);
            }
        }else if path_str.contains("uf50") {
            println!("{}", path_str);
            let sol=solve(path_str);
            if sol.0{
                uf50_time.push(sol.1 as f64/1000000.);
            }
        }
    }
    paths = std::fs::read_dir("inputs/unsat").unwrap();
    for path in paths {
        let path_str = &format!("{}", path.unwrap().path().display());
        if path_str.contains("aim") {
            println!("{}", path_str);
            let sol=solve(path_str);
            if sol.0{
                aim_time.push(sol.1 as f64/1000000.);
            }
        } else if path_str.contains("ii") {
            println!("{}", path_str);
            let sol=solve(path_str);
            if sol.0{
                ii_time.push(sol.1 as f64/1000000.);
            }
        }else if path_str.contains("par") {
            println!("{}", path_str);
            let sol=solve(path_str);
            if sol.0{
                par_time.push(sol.1 as f64/1000000.);
            }
        }else if path_str.contains("ssa") {
            println!("{}", path_str);
            let sol=solve(path_str);
            if sol.0{
                ssa_time.push(sol.1 as f64/1000000.);
            }
        }else if path_str.contains("uf50") {
            println!("{}", path_str);
            let sol=solve(path_str);
            if sol.0{
                uf50_time.push(sol.1 as f64/1000000.);
            }
        }
    }
    aim_time.sort_by(|a,b| a.partial_cmp(b).unwrap());
    ii_time.sort_by(|a,b| a.partial_cmp(b).unwrap());
    par_time.sort_by(|a,b| a.partial_cmp(b).unwrap());
    ssa_time.sort_by(|a,b| a.partial_cmp(b).unwrap());
    uf50_time.sort_by(|a,b| a.partial_cmp(b).unwrap());
    println!("{:?}",aim_time);
    println!("{:?}",ii_time);
    println!("{:?}",par_time);
    println!("{:?}",ssa_time);
    println!("{:?}",uf50_time);
    let mut aim_tup =Vec::<(f64,f64)>::new();
    for i in 0..aim_time.len(){
        aim_tup.push((i as f64,aim_time(i)));
    }
    let mut ii_tup =Vec::<(f64,f64)>::new();
    for i in 0..ii_time.len(){
        ii_tup.push((i as f64,aim_time(i)));
    }
    let mut par_tup =Vec::<(f64,f64)>::new();
    for i in 0..par_time.len(){
        par_tup.push((i as f64,aim_time(i)));
    }
    let mut ssa_tup =Vec::<(f64,f64)>::new();
    for i in 0..ssa_time.len(){
        ssa_tup.push((i as f64,aim_time(i)));
    }
    let mut uf50_tup =Vec::<(f64,f64)>::new();
    for i in 0..uf50_time.len(){
        uf50_tup.push((i as f64,aim_time(i)));
    }
    ploter(aim_tup,ii_tup,par_tup,ssa_tup,uf50_tup);
}

fn ploter(aim:Vec<(f64,f64)>,ii:Vec<(f64,f64)>,par:Vec<(f64,f64)>,ssa:Vec<(f64,f64)>,uf50:Vec<(f64,f64)>) {
    let root_area = BitMapBackend::new("test.png", (600, 400))
    .into_drawing_area();
    root_area.fill(&WHITE).unwrap();

    let mut ctx = ChartBuilder::on(&root_area)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .caption("Benchmarks", ("sans-serif", 40))
        .build_cartesian_2d(0..300, 0..60)
        .unwrap();

    ctx.configure_mesh().draw().unwrap();

    ctx.draw_series(
        aim.iter().map(|point| TriangleMarker::new(*point, 5, &BLUE)),
    )
    .unwrap();

    ctx.draw_series(
        ii.iter().map(|point| TriangleMarker::new(*point, 5, &RED)),
    )
    .unwrap();
    ctx.draw_series(
        par.iter().map(|point| TriangleMarker::new(*point, 5, &GREEN)),
    )
    .unwrap();
    ctx.draw_series(
        ssa.iter().map(|point| TriangleMarker::new(*point, 5, &YELLOW)),
    )
    .unwrap();
    ctx.draw_series(
        uf50.iter().map(|point| TriangleMarker::new(*point, 5, &BLACK)),
    )
    .unwrap();
}