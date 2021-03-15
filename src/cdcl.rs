#[derive(Debug)]
pub struct Clause {
    variables: Vec<usize>,
    signs: Vec<bool>,
    watched: [usize; 2],
    learned: bool,
}

#[derive(Debug)]
pub struct Variable {
    value: usize,
    pos_watched_occ: Vec<usize>,
    neg_watched_occ: Vec<usize>,
    r: [usize; 2], // for + and -
}

pub fn cdcl(path: &str) {
    test_set_value1();
    test_set_value2();
    test_set_value3();
    test_set_value4();
    test_set_value5();
    let mut clauses = Vec::<Clause>::new();
    let mut variables = Vec::<Variable>::new();
    let mut queue = Vec::<(usize, bool, bool, usize)>::new(); // var_index,var_val,forced,clause_index(reason)
    let mut backtracking_stack = Vec::<(usize, usize, bool, usize)>::new(); // var_index,depth,forced,clause_index
    let mut cur_depth = 0;
    let mut deleted_clauses = Vec::<usize>::new();
    let mut priority_queue = Vec::<(usize, usize, bool)>::new();
    let mut phase_saving = true;
    init(path, &mut clauses, &mut variables);
    let mut phases = vec![2; variables.len()];
    // print_clauses(&clauses, &deleted_clauses, &variables);
    // print_variables(&variables);
    let mut unsat = !preprocessing(
        &mut clauses,
        &mut deleted_clauses,
        &mut variables,
        &mut priority_queue,
    );
    if !unsat {
        set_priority_queue(&clauses, &deleted_clauses, &variables, &mut priority_queue);
    }
    let mut heuristic_count = 0;
    while !unsat {
        let (next_variable, next_value) = vsids(
            &variables,
            &priority_queue,
            &mut cur_depth,
            phase_saving,
            &phases,
        );
        heuristic_count += 1;
        if heuristic_count == 255 {
            //println!("boom");
            update_priority(&mut variables, &mut priority_queue);
            delete_clauses(&mut clauses, &mut deleted_clauses, &mut variables);
            restart(
                &mut variables,
                &mut backtracking_stack,
                &mut queue,
                phase_saving,
                &mut phases,
                &mut cur_depth,
            );
            heuristic_count = 0;
        }
        if next_variable == 0 {
            break;
        }
        queue.push((next_variable, next_value, false, 0));
        while queue.len() > 0 && !unsat {
            if !set_value(
                &mut clauses,
                &mut variables,
                &mut queue,
                &mut backtracking_stack,
                &mut deleted_clauses,
                &mut cur_depth,
                &mut priority_queue,
            ) {
                unsat = true;
            }
            // print_clauses(&clauses, &deleted_clauses, &variables);
            // print_variables(&variables);
            // println!("Queue: {:?}", queue);
            // println!(
            //     "Backtracking stack: {:?} {}",
            //     backtracking_stack,
            //     backtracking_stack.len()
            // );
            //check_watched(&clauses, &deleted_clauses,&variables);
        }
    }
    if !unsat {
        assert!(validate(&clauses, &variables));
        let mut sol = Vec::<i32>::new();
        let mut sol_str = String::from("s SATISFIABLE\nv");
        for i in 0..variables.len() {
            if variables[i].value == 1 {
                sol.push(i as i32 + 1);
            } else {
                sol.push(-(i as i32) - 1);
            }
            sol_str.push_str(&format!(" {}", sol[sol.len() - 1]));
        }
        sol_str.push_str(" 0");
        println!("{}", sol_str);
    } else {
        println!("s UNSATISFIABLE");
    }
}

fn read_file(path: &str) -> String {
    use std::io::Read;
    let mut file = std::fs::File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}

fn init(path: &str, mut clauses: &mut Vec<Clause>, mut variables: &mut Vec<Variable>) {
    let dummy_var = Variable {
        value: 2,
        pos_watched_occ: Vec::<usize>::new(),
        neg_watched_occ: Vec::<usize>::new(),
        r: [0, 0],
    };
    variables.push(dummy_var);
    let content: String = read_file(path);
    let content_clone = content.clone();
    let input: Vec<&str> = (&content_clone).split("\n").collect();
    let mut variables_qty: usize;
    let mut lits = Vec::<i32>::new();
    for line_number in 0..input.len() {
        let line_elem: Vec<&str> = input[line_number].split_whitespace().collect();
        if line_elem.len() > 0 && line_elem[0] != "c" {
            if line_elem[0] == "p" {
                variables_qty = line_elem[2].parse::<usize>().unwrap();
                for _i in 0..variables_qty {
                    let variable = Variable {
                        value: 2,
                        pos_watched_occ: Vec::<usize>::new(),
                        neg_watched_occ: Vec::<usize>::new(),
                        r: [0, 0],
                    };
                    variables.push(variable);
                }
            } else {
                for j in 0..line_elem.len() {
                    let lit = line_elem[j].parse::<i32>().unwrap();
                    lits.push(lit);
                }
            }
        }
    }

    let lits_chunk = lits[..lits.len() - 1].split(|x| *x == 0);
    for chunk in lits_chunk {
        let mut clause = Clause {
            variables: Vec::<usize>::new(),
            signs: Vec::<bool>::new(),
            watched: [0 as usize, 0 as usize],
            learned: false,
        };
        if remove_duplicate_vars(&mut chunk.to_vec()) {
            for lit_p in chunk {
                let lit = *lit_p;
                clause.variables.push((lit.abs()) as usize);
                clause.signs.push(lit > 0);
            }
            clauses.push(clause);
            let cl_index = clauses.len() - 1;
            if clauses[cl_index].variables.len() > 1 {
                let var1 = clauses[cl_index].variables[0];
                let var2 = clauses[cl_index].variables[1];
                double_watch(&mut clauses, &mut variables, cl_index, var1, var2);
            }
        }
    }
}

fn update_priority(variables: &mut Vec<Variable>, priority_queue: &mut Vec<(usize, usize, bool)>) {
    for i in 0..priority_queue.len() {
        if priority_queue[i].2 {
            priority_queue[i].0 = priority_queue[i].0 / 2 + variables[priority_queue[i].1].r[0];
            variables[priority_queue[i].1].r[0] = 0;
        } else {
            priority_queue[i].0 = priority_queue[i].0 / 2 + variables[priority_queue[i].1].r[1];
            variables[priority_queue[i].1].r[1] = 0;
        }
    }
    priority_queue.sort();
}

fn delete_clauses(
    clauses: &mut Vec<Clause>,
    deleted_clauses: &mut Vec<usize>,
    mut variables: &mut Vec<Variable>,
) {
    for i in 0..clauses.len() {
        if clauses[i].learned && !deleted_clauses.contains(&i) && clauses[i].variables.len() > 6 {
            let mut free_count = 0;
            for j in &clauses[i].variables {
                if variables[*j].value == 2 {
                    free_count += 1;
                }
            }
            if free_count > 3 {
                deleted_clauses.push(i);
                double_unwatch(&clauses, &mut variables, i);
            }
        }
    }
}

fn restart(
    variables: &mut Vec<Variable>,
    backtracking_stack: &mut Vec<(usize, usize, bool, usize)>,
    queue: &mut Vec<(usize, bool, bool, usize)>,
    phase_saving: bool,
    phases: &mut Vec<usize>,
    cur_depth: &mut usize,
) {
    if phase_saving {
        for i in 1..variables.len() {
            phases[i] = variables[i].value;
        }
    }
    queue.clear();
    while backtracking_stack.len() > 0 {
        variables[backtracking_stack.pop().unwrap().0].value = 2;
    }
    *cur_depth = 0 as usize;
}

fn preprocessing(
    mut clauses: &mut Vec<Clause>,
    deleted_clauses: &mut Vec<usize>,
    mut variables: &mut Vec<Variable>,
    priority_queue: &mut Vec<(usize, usize, bool)>,
) -> bool {
    //println!("Preprocessing");
    let mut i = 0;
    while i < clauses.len() {
        if clauses[i].variables.len() == 1 && !deleted_clauses.contains(&i) {
            let var_index = clauses[i].variables[0];
            variables[var_index].value = clauses[i].signs[0] as usize;
            let mut k = 0;
            while k < priority_queue.len() {
                if priority_queue[k].1 == var_index {
                    priority_queue.remove(k);
                } else {
                    k += 1;
                }
            }
            for j in 0..clauses.len() {
                if !deleted_clauses.contains(&j) {
                    let found = clauses[j].variables.iter().position(|x| *x == var_index);
                    match found {
                        Some(x) => {
                            if variables[var_index].value == clauses[j].signs[x] as usize {
                                // remove clause
                                deleted_clauses.push(j);
                                if clauses[j].variables.len() > 1 {
                                    double_unwatch(&clauses, &mut variables, j);
                                }
                            } else {
                                // remove var from clause
                                if clauses[j].variables.len() == 1 {
                                    // unsat
                                    return false;
                                } else if clauses[j].variables.len() == 2 {
                                    double_unwatch(&clauses, &mut variables, j);
                                } else {
                                    if clauses[j].watched.contains(&var_index) {
                                        let mut new_watch = 0;
                                        for v in &clauses[j].variables {
                                            if !clauses[j].watched.contains(v)
                                                && variables[*v].value == 2
                                            {
                                                new_watch = *v;
                                                break;
                                            }
                                        }
                                        switch_watch(
                                            &mut clauses,
                                            &mut variables,
                                            j,
                                            var_index,
                                            new_watch,
                                        );
                                    }
                                }
                                clauses[j].variables.remove(x);
                                clauses[j].signs.remove(x);
                            }
                        }
                        None => {}
                    }
                }
            }
            i = 0;
        } else {
            i += 1;
        }
    }
    // print_clauses(&clauses, &deleted_clauses, &variables);
    // print_variables(&variables);
    // check_watched(&clauses, &deleted_clauses, &variables);
    return true;
}

fn set_priority_queue(
    clauses: &Vec<Clause>,
    deleted_clauses: &Vec<usize>,
    variables: &Vec<Variable>,
    priority_queue: &mut Vec<(usize, usize, bool)>,
) {
    for i in 1..variables.len() {
        if variables[i].value == 2 {
            priority_queue.push((0, i, true));
            priority_queue.push((0, i, false));
        }
    }
    for i in 0..clauses.len() {
        if !deleted_clauses.contains(&i) {
            for j in 0..clauses[i].variables.len() {
                let mut k = 0;
                while priority_queue[k].1 != clauses[i].variables[j]
                    || priority_queue[k].2 != clauses[i].signs[j]
                {
                    k += 1;
                }
                priority_queue[k].0 += 1;
            }
        }
    }
    priority_queue.sort();
}

fn remove_duplicate_vars(chunk: &mut Vec<i32>) -> bool {
    chunk.sort();
    let mut new_chunk = vec![chunk[0]];
    for i in 0..chunk.len() {
        if chunk[i] != new_chunk[new_chunk.len() - 1] {
            new_chunk.push(chunk[i]);
        }
    }
    for i in &new_chunk {
        if new_chunk.contains(&-i) {
            return false;
        }
    }
    *chunk = new_chunk;
    return true;
}

fn set_value(
    mut clauses: &mut Vec<Clause>,
    mut variables: &mut Vec<Variable>,
    mut queue: &mut Vec<(usize, bool, bool, usize)>,
    mut backtracking_stack: &mut Vec<(usize, usize, bool, usize)>,
    mut deleted_clauses: &mut Vec<usize>,
    mut cur_depth: &mut usize,
    mut priority_queue: &mut Vec<(usize, usize, bool)>,
) -> bool {
    let tup = queue.pop().unwrap();
    let variable_index = tup.0;
    let value = tup.1;
    let forced = tup.2;
    let reason = tup.3;
    if variables[variable_index].value == 2 {
        variables[variable_index].value = value as usize;
        backtracking_stack.push((variable_index, *cur_depth, forced, reason));
        let mut range: usize;
        if value {
            range = variables[variable_index].neg_watched_occ.len()
        } else {
            range = variables[variable_index].pos_watched_occ.len()
        }
        let mut i = 0;
        while i < range {
            let clause_index: usize;
            if value {
                clause_index = variables[variable_index].neg_watched_occ[i];
            } else {
                clause_index = variables[variable_index].pos_watched_occ[i];
            }
            i += 1;
            let mut conflict = true;
            let mut sat = false;
            let mut new_watched = 0;
            let mut new_watched_index = 0;
            let mut unit_var = 0;
            let mut unit_sign = true;
            for j in 0..clauses[clause_index].variables.len() {
                let other_var = clauses[clause_index].variables[j];
                if variables[other_var].value == clauses[clause_index].signs[j] as usize {
                    sat = true;
                    break; // clause sat
                } else if variables[other_var].value == 2 {
                    conflict = false;
                    if !clauses[clause_index].watched.contains(&other_var) {
                        new_watched = other_var;
                        new_watched_index = j;
                    } else if other_var != variable_index {
                        unit_var = other_var;
                        unit_sign = clauses[clause_index].signs[j];
                    }
                }
            }
            if !sat {
                if conflict {
                    //println!("Conflict 1!");
                    return resolve_conflict(
                        &mut clauses,
                        &mut variables,
                        &mut queue,
                        &mut backtracking_stack,
                        &mut deleted_clauses,
                        &mut cur_depth,
                        clause_index,
                        &mut priority_queue,
                    );
                } else {
                    if new_watched != 0 {
                        // found new watched
                        switch_watch(
                            &mut clauses,
                            &mut variables,
                            clause_index,
                            variable_index,
                            new_watched,
                        );
                        range -= 1;
                        i -= 1;
                    } else {
                        // unit prop
                        queue.push((unit_var, unit_sign, true, clause_index));
                    }
                }
            }
        }
    } else if variables[variable_index].value != value as usize {
        println!("Conflict 2!");
        return resolve_conflict(
            &mut clauses,
            &mut variables,
            &mut queue,
            &mut backtracking_stack,
            &mut deleted_clauses,
            &mut cur_depth,
            reason,
            &mut priority_queue,
        );
    }
    return true;
}

fn resolve_conflict(
    mut clauses: &mut Vec<Clause>,
    mut variables: &mut Vec<Variable>,
    queue: &mut Vec<(usize, bool, bool, usize)>,
    mut backtracking_stack: &mut Vec<(usize, usize, bool, usize)>,
    mut deleted_clauses: &mut Vec<usize>,
    cur_depth: &mut usize,
    clause_index: usize,
    mut priority_queue: &mut Vec<(usize, usize, bool)>,
) -> bool {
    // print_clauses(&clauses, &deleted_clauses, &variables);
    // print_variables(&variables);
    // println!("Queue: {:?}", queue);
    // println!("Backtracking stack: {:?} {}", backtracking_stack,backtracking_stack.len());
    // println!("Deleted clauses: {:?}", deleted_clauses);
    queue.clear();
    if backtracking_stack[backtracking_stack.len() - 1].1 == 1 {
        let first_var = backtracking_stack[0].0;
        let unit_clause = Clause {
            variables: vec![first_var],
            signs: vec![variables[first_var].value == 0], // first value wrong due to conflict
            watched: [0 as usize, 0 as usize],
            learned: false,
        };
        clause_learning(
            &mut clauses,
            &mut deleted_clauses,
            &mut variables,
            &unit_clause,
            0,
            0,
        );
        while backtracking_stack.len() > 0 {
            variables[backtracking_stack.pop().unwrap().0].value = 2;
        }
        *cur_depth = 0 as usize;
        return preprocessing(
            &mut clauses,
            &mut deleted_clauses,
            &mut variables,
            &mut priority_queue,
        );
    }
    let mut max_depth_var = Vec::<usize>::new();
    let mut index = backtracking_stack.len() - 2; // remove the last one
    while backtracking_stack[index].1 == *cur_depth {
        max_depth_var.push(backtracking_stack[index].0);
        index -= 1;
    }
    max_depth_var.reverse();
    let mut tup: (usize, usize, bool, usize);
    let mut var_index: usize;
    let mut last_reason: usize;
    let mut resolvent = &mut copy_clause(&clauses[clause_index]);
    let mut result: Clause;
    while intersection(&max_depth_var, &resolvent.variables) {
        // resolvent is not asserting
        tup = backtracking_stack.pop().unwrap();
        var_index = tup.0;
        last_reason = tup.3;
        result = get_resolvent(resolvent, &clauses[last_reason], var_index);
        resolvent = &mut result;
        max_depth_var.pop();
        variables[var_index].value = 2;
    }
    if resolvent.variables.len() == 1 {
        resolvent.signs = vec![variables[resolvent.variables[0]].value == 0];
        clause_learning(
            &mut clauses,
            &mut deleted_clauses,
            &mut variables,
            &resolvent,
            0,
            0,
        );
        while backtracking_stack.len() > 0 {
            variables[backtracking_stack.pop().unwrap().0].value = 2;
        }
        *cur_depth = 0 as usize;
        return preprocessing(
            &mut clauses,
            &mut deleted_clauses,
            &mut variables,
            &mut priority_queue,
        );
    }
    let (uip_index, new_uip_value, new_clause_index) = non_chronological_backtracking(
        &mut clauses,
        &mut deleted_clauses,
        &mut variables,
        resolvent,
        &mut backtracking_stack,
    );
    *cur_depth = backtracking_stack[backtracking_stack.len() - 1].1;
    queue.push((uip_index, new_uip_value, true, new_clause_index));
    return true;
}

fn intersection(vec1: &Vec<usize>, vec2: &Vec<usize>) -> bool {
    for i in vec1 {
        for j in vec2 {
            if *i == *j {
                return true;
            }
        }
    }
    return false;
}

fn get_resolvent(clause1: &Clause, clause2: &Clause, var_index: usize) -> Clause {
    let mut resolvent = Clause {
        variables: Vec::<usize>::new(),
        signs: Vec::<bool>::new(),
        watched: [0 as usize, 0 as usize],
        learned: false,
    };
    for i in 0..clause1.variables.len() {
        if clause1.variables[i] != var_index {
            resolvent.variables.push(clause1.variables[i]);
            resolvent.signs.push(clause1.signs[i]);
        }
    }
    for i in 0..clause2.variables.len() {
        if clause2.variables[i] != var_index && !resolvent.variables.contains(&clause2.variables[i])
        {
            resolvent.variables.push(clause2.variables[i]);
            resolvent.signs.push(clause2.signs[i]);
        }
    }
    resolvent
}

fn non_chronological_backtracking(
    mut clauses: &mut Vec<Clause>,
    mut deleted_clauses: &mut Vec<usize>,
    mut variables: &mut Vec<Variable>,
    resolvent: &mut Clause,
    backtracking_stack: &mut Vec<(usize, usize, bool, usize)>,
) -> (usize, bool, usize) {
    let uip_index = backtracking_stack.pop().unwrap().0;
    let new_uip_value = variables[uip_index].value == 0;
    variables[uip_index].value = 2; // set the uip free
    let mut i = backtracking_stack.len() - 1;
    while !resolvent.variables.contains(&backtracking_stack[i].0) {
        i -= 1;
    }
    let assertion_level = backtracking_stack[i].1;
    while backtracking_stack[backtracking_stack.len() - 1].1 > assertion_level {
        variables[backtracking_stack.pop().unwrap().0].value = 2;
    }
    let new_clause_index = clause_learning(
        &mut clauses,
        &mut deleted_clauses,
        &mut variables,
        resolvent,
        uip_index,
        backtracking_stack[i].0,
    );
    (uip_index, new_uip_value, new_clause_index)
}

fn clause_learning(
    mut clauses: &mut Vec<Clause>,
    deleted_clauses: &mut Vec<usize>,
    mut variables: &mut Vec<Variable>,
    new_clause: &Clause,
    var1: usize,
    var2: usize,
) -> usize {
    let mut copy = copy_clause(new_clause);
    copy.learned = true;
    let new_clause_index: usize;
    if deleted_clauses.len() > 0 {
        new_clause_index = deleted_clauses.pop().unwrap();
        clauses[new_clause_index] = copy; // replace one deleted clause with the new clause
    } else {
        clauses.push(copy);
        new_clause_index = clauses.len() - 1;
    }
    double_watch(&mut clauses, &mut variables, new_clause_index, var1, var2);
    update_r(&clauses, &mut variables, new_clause_index);
    new_clause_index
}

fn copy_clause(clause: &Clause) -> Clause {
    let mut copy = Clause {
        variables: Vec::<usize>::new(),
        signs: Vec::<bool>::new(),
        watched: [0 as usize, 0 as usize],
        learned: false,
    };
    for i in 0..clause.variables.len() {
        copy.variables.push(clause.variables[i]);
        copy.signs.push(clause.signs[i]);
    }
    copy.watched[0] = clause.watched[0];
    copy.watched[1] = clause.watched[1];
    copy.learned = clause.learned;
    copy
}

fn update_r(clauses: &Vec<Clause>, variables: &mut Vec<Variable>, clause_index: usize) {
    for i in 0..clauses[clause_index].variables.len() {
        if clauses[clause_index].signs[i] {
            variables[clauses[clause_index].variables[i]].r[0] += 1;
        } else {
            variables[clauses[clause_index].variables[i]].r[1] += 1;
        }
    }
}

fn double_unwatch(clauses: &Vec<Clause>, mut variables: &mut Vec<Variable>, clause_index: usize) {
    let watched0_pos = clauses[clause_index]
        .variables
        .iter()
        .position(|&x| x == clauses[clause_index].watched[0])
        .unwrap();
    let watched1_pos = clauses[clause_index]
        .variables
        .iter()
        .position(|&x| x == clauses[clause_index].watched[1])
        .unwrap();
    unwatch(
        &mut variables,
        clause_index,
        clauses[clause_index].watched[0],
        clauses[clause_index].signs[watched0_pos],
    );
    unwatch(
        &mut variables,
        clause_index,
        clauses[clause_index].watched[1],
        clauses[clause_index].signs[watched1_pos],
    );
}

fn double_watch(
    clauses: &mut Vec<Clause>,
    mut variables: &mut Vec<Variable>,
    clause_index: usize,
    var1: usize,
    var2: usize,
) {
    if (clauses[clause_index].variables.len() > 1) {
        clauses[clause_index].watched[0] = var1;
        clauses[clause_index].watched[1] = var2;
        let var1_pos = clauses[clause_index]
            .variables
            .iter()
            .position(|&x| x == var1)
            .unwrap();
        let var2_pos = clauses[clause_index]
            .variables
            .iter()
            .position(|&x| x == var2)
            .unwrap();
        watch(
            &mut variables,
            clause_index,
            var1,
            clauses[clause_index].signs[var1_pos],
        );
        watch(
            &mut variables,
            clause_index,
            var2,
            clauses[clause_index].signs[var2_pos],
        );
    }
}

fn watch(variables: &mut Vec<Variable>, clause_index: usize, variable_index: usize, sign: bool) {
    if sign {
        variables[variable_index].pos_watched_occ.push(clause_index);
    } else {
        variables[variable_index].neg_watched_occ.push(clause_index);
    }
}

fn unwatch(variables: &mut Vec<Variable>, clause_index: usize, variable_index: usize, sign: bool) {
    if sign {
        variables[variable_index]
            .pos_watched_occ
            .retain(|x| *x != clause_index);
    } else {
        variables[variable_index]
            .neg_watched_occ
            .retain(|x| *x != clause_index);
    }
    //println!("{} unwatched for {}", clause_index, variable_index);
}

fn switch_watch(
    clauses: &mut Vec<Clause>,
    mut variables: &mut Vec<Variable>,
    clause_index: usize,
    old_watch: usize,
    new_watch: usize,
) {
    let old_watch_pos = clauses[clause_index]
        .variables
        .iter()
        .position(|&x| x == old_watch)
        .unwrap();
    let new_watch_pos = clauses[clause_index]
        .variables
        .iter()
        .position(|&x| x == new_watch)
        .unwrap();
    if clauses[clause_index].watched[0] == old_watch {
        clauses[clause_index].watched[0] = new_watch;
    } else {
        clauses[clause_index].watched[1] = new_watch;
    }
    unwatch(
        &mut variables,
        clause_index,
        old_watch,
        clauses[clause_index].signs[old_watch_pos],
    );
    watch(
        &mut variables,
        clause_index,
        new_watch,
        clauses[clause_index].signs[new_watch_pos],
    );
}

fn vsids(
    variables: &Vec<Variable>,
    priority_queue: &Vec<(usize, usize, bool)>,
    cur_depth: &mut usize,
    phase_saving: bool,
    phases: &Vec<usize>,
) -> (usize, bool) {
    if priority_queue.len() == 0 {
        return (0, true);
    }
    let mut i = priority_queue.len() - 1;
    while variables[priority_queue[i].1].value != 2 && i >= 1 {
        i -= 1;
    }
    if i > 0 || variables[priority_queue[i].1].value == 2 {
        *cur_depth += 1;
        if phase_saving && phases[priority_queue[i].1] != 2 {
            return (priority_queue[i].1, phases[priority_queue[i].1] != 0);
        }
        (priority_queue[i].1, priority_queue[i].2)
    } else {
        (0, true)
    }
}

fn print_variables(variables: &Vec<Variable>) {
    for i in 1..variables.len() {
        if variables[i].value == 0 {
            println!(
                "{}: false {:?} {:?}",
                i, variables[i].pos_watched_occ, variables[i].neg_watched_occ
            );
        } else if variables[i].value == 1 {
            println!(
                "{}: true {:?} {:?}",
                i, variables[i].pos_watched_occ, variables[i].neg_watched_occ
            );
        } else {
            println!(
                "{}: free {:?} {:?}",
                i, variables[i].pos_watched_occ, variables[i].neg_watched_occ
            );
        }
    }
}

fn print_clauses(clauses: &Vec<Clause>, deleted_clauses: &Vec<usize>, variables: &Vec<Variable>) {
    for i in 0..clauses.len() {
        let mut sat = false;
        let mut var_str = String::new();
        let mut val_str = String::new();
        for j in 0..clauses[i].variables.len() {
            if clauses[i].signs[j] {
                var_str = format!("{} {}", var_str, clauses[i].variables[j]);
            } else {
                var_str = format!("{} -{}", var_str, clauses[i].variables[j]);
            }
            val_str = format!("{} {}", val_str, variables[clauses[i].variables[j]].value);
            if clauses[i].signs[j] as usize == variables[clauses[i].variables[j]].value {
                sat = true;
            }
        }
        var_str = format!(
            "{} watched:{} {}",
            var_str, clauses[i].watched[0], clauses[i].watched[1]
        );
        if deleted_clauses.contains(&i) {
            var_str = format!("{} deleted", var_str);
        }
        if sat {
            val_str = format!("{} sat", val_str);
        }
        println!("{}, {}", i, var_str);
        println!("{}, {}", i, val_str);
    }
}

fn validate(clauses: &Vec<Clause>, variables: &Vec<Variable>) -> bool {
    // check if there is still free variables
    for i in 1..variables.len() {
        if variables[i].value == 2 {
            return false;
        }
    }

    // logical check if all clauses are really sat
    for i in 0..clauses.len() {
        let mut sat = false;
        for j in 0..clauses[i].variables.len() {
            if clauses[i].signs[j] as usize == variables[clauses[i].variables[j]].value {
                sat = true;
                break;
            }
        }
        if !sat {
            return false;
        }
    }
    true
}

fn check_watched(clauses: &Vec<Clause>, deleted_clauses: &Vec<usize>, variables: &Vec<Variable>) {
    for i in 0..clauses.len() {
        if !deleted_clauses.contains(&i) {
            for j in 0..2 {
                if clauses[i].watched[j] != 0 as usize {
                    let watched_index = clauses[i]
                        .variables
                        .iter()
                        .position(|&x| x == clauses[i].watched[j])
                        .unwrap();
                    if clauses[i].signs[watched_index] {
                        assert!(variables[clauses[i].watched[j]]
                            .pos_watched_occ
                            .contains(&i));
                    } else {
                        assert!(variables[clauses[i].watched[j]]
                            .neg_watched_occ
                            .contains(&i));
                    }
                }
            }
        }
    }
    for i in 1..variables.len() {
        for j in &variables[i].pos_watched_occ {
            assert!(clauses[*j].watched.contains(&i));
            assert!(!deleted_clauses.contains(j));
        }
        for j in &variables[i].neg_watched_occ {
            assert!(clauses[*j].watched.contains(&i));
            assert!(!deleted_clauses.contains(j));
        }
    }
}

fn check_uniqueness(vec: &mut Vec<usize>) {
    vec.sort();
    for i in 0..vec.len() - 1 {
        assert!(vec[i + 1] != vec[i]);
    }
}

fn test_set_value1() {
    let mut cl = Clause {
        variables: vec![1, 2, 3],
        signs: vec![true, true, true],
        watched: [1 as usize, 2 as usize],
        learned: false,
    };
    let dummy = Variable {
        value: 2,
        pos_watched_occ: vec![0],
        neg_watched_occ: Vec::<usize>::new(),
        r: [0, 0],
    };
    let var1 = Variable {
        value: 2,
        pos_watched_occ: vec![0],
        neg_watched_occ: Vec::<usize>::new(),
        r: [0, 0],
    };
    let var2 = Variable {
        value: 2,
        pos_watched_occ: vec![0],
        neg_watched_occ: Vec::<usize>::new(),
        r: [0, 0],
    };
    let var3 = Variable {
        value: 2,
        pos_watched_occ: Vec::<usize>::new(),
        neg_watched_occ: Vec::<usize>::new(),
        r: [0, 0],
    };
    let mut variables = vec![dummy, var1, var2, var3];
    let mut clauses = vec![cl];
    let mut queue = vec![(1, true, false, 0)];
    let mut backtracking_stack = Vec::<(usize, usize, bool, usize)>::new();
    let mut deleted_clauses = Vec::<usize>::new();
    let mut cur_depth = 1;
    let mut priority_queue = Vec::<(usize, usize, bool)>::new();
    set_priority_queue(&clauses, &deleted_clauses, &variables, &mut priority_queue);
    set_value(
        &mut clauses,
        &mut variables,
        &mut queue,
        &mut backtracking_stack,
        &mut deleted_clauses,
        &mut cur_depth,
        &mut priority_queue,
    );
    assert!(clauses[0].watched == [1, 2]);
    assert!(variables[1].value == 1);
    assert!(variables[2].value == 2);
    assert!(variables[3].value == 2);
    assert!(variables[1].pos_watched_occ == [0]);
    assert!(variables[2].pos_watched_occ == [0]);
}

fn test_set_value2() {
    let mut cl = Clause {
        variables: vec![1, 2, 3],
        signs: vec![false, true, true],
        watched: [1 as usize, 2 as usize],
        learned: false,
    };
    let dummy = Variable {
        value: 2,
        pos_watched_occ: Vec::<usize>::new(),
        neg_watched_occ: Vec::<usize>::new(),
        r: [0, 0],
    };
    let var1 = Variable {
        value: 2,
        pos_watched_occ: Vec::<usize>::new(),
        neg_watched_occ: vec![0],
        r: [0, 0],
    };
    let var2 = Variable {
        value: 2,
        pos_watched_occ: vec![0],
        neg_watched_occ: Vec::<usize>::new(),
        r: [0, 0],
    };
    let var3 = Variable {
        value: 2,
        pos_watched_occ: Vec::<usize>::new(),
        neg_watched_occ: Vec::<usize>::new(),
        r: [0, 0],
    };
    let mut variables = vec![dummy, var1, var2, var3];
    let mut clauses = vec![cl];
    let mut queue = vec![(1, true, false, 0)];
    let mut backtracking_stack = Vec::<(usize, usize, bool, usize)>::new();
    let mut deleted_clauses = Vec::<usize>::new();
    let mut cur_depth = 1;
    let mut priority_queue = Vec::<(usize, usize, bool)>::new();
    set_priority_queue(&clauses, &deleted_clauses, &variables, &mut priority_queue);
    set_value(
        &mut clauses,
        &mut variables,
        &mut queue,
        &mut backtracking_stack,
        &mut deleted_clauses,
        &mut cur_depth,
        &mut priority_queue,
    );
    assert!(clauses[0].watched == [3, 2]);
    assert!(variables[1].value == 1);
    assert!(variables[2].value == 2);
    assert!(variables[3].value == 2);
    assert!(variables[1].neg_watched_occ == []);
    assert!(variables[2].pos_watched_occ == [0]);
    assert!(variables[3].pos_watched_occ == [0]);
}

fn test_set_value3() {
    let mut cl = Clause {
        variables: vec![1, 2],
        signs: vec![false, true],
        watched: [1 as usize, 2 as usize],
        learned: false,
    };
    let dummy = Variable {
        value: 2,
        pos_watched_occ: Vec::<usize>::new(),
        neg_watched_occ: Vec::<usize>::new(),
        r: [0, 0],
    };
    let var1 = Variable {
        value: 2,
        pos_watched_occ: Vec::<usize>::new(),
        neg_watched_occ: vec![0],
        r: [0, 0],
    };
    let var2 = Variable {
        value: 2,
        pos_watched_occ: vec![0],
        neg_watched_occ: Vec::<usize>::new(),
        r: [0, 0],
    };
    let mut variables = vec![dummy, var1, var2];
    let mut clauses = vec![cl];
    let mut queue = vec![(1, true, false, 0)];
    let mut backtracking_stack = Vec::<(usize, usize, bool, usize)>::new();
    let mut deleted_clauses = Vec::<usize>::new();
    let mut cur_depth = 1;
    let mut priority_queue = Vec::<(usize, usize, bool)>::new();
    set_priority_queue(&clauses, &deleted_clauses, &variables, &mut priority_queue);
    set_value(
        &mut clauses,
        &mut variables,
        &mut queue,
        &mut backtracking_stack,
        &mut deleted_clauses,
        &mut cur_depth,
        &mut priority_queue,
    );
    assert!(clauses[0].watched == [1, 2]);
    assert!(variables[1].value == 1);
    assert!(variables[2].value == 2);
    assert!(variables[1].neg_watched_occ == [0]);
    assert!(variables[2].pos_watched_occ == [0]);
    assert!(queue == [(2, true, true, 0)]);
}

fn test_set_value4() {
    let mut cl1 = Clause {
        variables: vec![1, 2],
        signs: vec![false, true],
        watched: [1 as usize, 2 as usize],
        learned: false,
    };
    let mut cl2 = Clause {
        variables: vec![1, 2],
        signs: vec![false, false],
        watched: [1 as usize, 2 as usize],
        learned: false,
    };
    let dummy = Variable {
        value: 2,
        pos_watched_occ: Vec::<usize>::new(),
        neg_watched_occ: Vec::<usize>::new(),
        r: [0, 0],
    };
    let var1 = Variable {
        value: 2,
        pos_watched_occ: Vec::<usize>::new(),
        neg_watched_occ: vec![0, 1],
        r: [0, 0],
    };
    let var2 = Variable {
        value: 2,
        pos_watched_occ: vec![0],
        neg_watched_occ: vec![1],
        r: [0, 0],
    };
    let mut variables = vec![dummy, var1, var2];
    let mut clauses = vec![cl1, cl2];
    let mut queue = vec![(1, true, false, 0)];
    let mut backtracking_stack = Vec::<(usize, usize, bool, usize)>::new();
    let mut deleted_clauses = Vec::<usize>::new();
    let mut cur_depth = 1;
    let mut priority_queue = Vec::<(usize, usize, bool)>::new();
    set_priority_queue(&clauses, &deleted_clauses, &variables, &mut priority_queue);
    set_value(
        &mut clauses,
        &mut variables,
        &mut queue,
        &mut backtracking_stack,
        &mut deleted_clauses,
        &mut cur_depth,
        &mut priority_queue,
    );
    set_value(
        &mut clauses,
        &mut variables,
        &mut queue,
        &mut backtracking_stack,
        &mut deleted_clauses,
        &mut cur_depth,
        &mut priority_queue,
    );
    assert!(clauses[0].watched == [1, 2]);
    assert!(variables[1].value == 0);
    assert!(variables[2].value == 2);
    assert!(variables[1].neg_watched_occ == []);
    assert!(variables[1].pos_watched_occ == []);
    assert!(variables[2].neg_watched_occ == []);
    assert!(variables[2].pos_watched_occ == []);
    assert!(clauses[2].variables == [1]);
    assert!(clauses[2].signs == [false]);
    assert!(deleted_clauses == [0, 1, 2]);
}

fn test_set_value5() {
    let mut cl1 = Clause {
        variables: vec![1],
        signs: vec![false],
        watched: [0 as usize, 0 as usize],
        learned: false,
    };
    let mut cl2 = Clause {
        variables: vec![1, 2],
        signs: vec![true, false],
        watched: [1 as usize, 2 as usize],
        learned: false,
    };
    let mut cl3 = Clause {
        variables: vec![1, 2, 3],
        signs: vec![true, true, true],
        watched: [1 as usize, 2 as usize],
        learned: false,
    };
    let mut cl4 = Clause {
        variables: vec![1, 2, 3],
        signs: vec![true, true, false],
        watched: [1 as usize, 2 as usize],
        learned: false,
    };
    let dummy = Variable {
        value: 2,
        pos_watched_occ: Vec::<usize>::new(),
        neg_watched_occ: Vec::<usize>::new(),
        r: [0, 0],
    };
    let var1 = Variable {
        value: 2,
        pos_watched_occ: vec![1, 2, 3],
        neg_watched_occ: Vec::<usize>::new(),
        r: [0, 0],
    };
    let var2 = Variable {
        value: 2,
        pos_watched_occ: vec![2, 3],
        neg_watched_occ: vec![1],
        r: [0, 0],
    };
    let var3 = Variable {
        value: 2,
        pos_watched_occ: Vec::<usize>::new(),
        neg_watched_occ: Vec::<usize>::new(),
        r: [0, 0],
    };
    let mut variables = vec![dummy, var1, var2, var3];
    let mut clauses = vec![cl1, cl2, cl3, cl4];
    let mut queue = vec![(1, true, false, 0)];
    let mut backtracking_stack = Vec::<(usize, usize, bool, usize)>::new();
    let mut deleted_clauses = Vec::<usize>::new();
    let mut cur_depth = 1;
    let mut priority_queue = Vec::<(usize, usize, bool)>::new();
    let res = preprocessing(
        &mut clauses,
        &mut deleted_clauses,
        &mut variables,
        &mut priority_queue,
    );
    assert!(variables[1].value == 0);
    assert!(variables[2].value == 0);
    assert!(variables[3].value == 1);
    assert!(variables[1].neg_watched_occ == []);
    assert!(variables[1].pos_watched_occ == []);
    assert!(variables[2].neg_watched_occ == []);
    assert!(variables[2].pos_watched_occ == []);
    assert!(variables[3].neg_watched_occ == []);
    assert!(variables[3].pos_watched_occ == []);
    assert!(deleted_clauses == [0, 1, 2]);
    assert!(res == false);
}
