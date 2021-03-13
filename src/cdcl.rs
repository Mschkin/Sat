#[derive(Debug)]
pub struct Clause {
    variables: Vec<usize>,
    signs: Vec<bool>,
    watched: [usize; 2],
}

#[derive(Debug)]
pub struct Variable {
    value: usize,
    pos_watched_occ: Vec<usize>,
    neg_watched_occ: Vec<usize>,
    priority: [usize; 2], // for + and -
    r: [usize; 2],        // for + and -
}

pub fn cdcl(path: &str) {
    let mut clauses = Vec::<Clause>::new();
    let mut variables = Vec::<Variable>::new();
    let mut queue = Vec::<(usize, bool, bool, usize)>::new(); // var_index,var_val,forced,clause_index(reason)
    let mut backtracking_stack = Vec::<(usize, usize, bool, usize)>::new(); // var_index,depth,forced,clause_index
    let mut cur_depth = 0;
    let mut deleted_clauses = Vec::<usize>::new();
    init(path, &mut clauses, &mut variables);
    let mut unsat = !preprocessing(&mut clauses, &mut deleted_clauses, &mut variables);
    while !unsat {
        let (next_variable, next_value) = vsids(&variables, &mut cur_depth);
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
        println!("solved");
    } else {
        println!("unsat");
    }
    println!("Validation: {}", validate(&clauses, &variables));
}

fn read_file(path: &str) -> String {
    use std::io::Read;
    let mut file = std::fs::File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}

fn init(path: &str, clauses: &mut Vec<Clause>, variables: &mut Vec<Variable>) {
    let dummy_var = Variable {
        value: 2,
        pos_watched_occ: Vec::<usize>::new(),
        neg_watched_occ: Vec::<usize>::new(),
        priority: [0, 0],
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
        if line_elem.len() > 0 && line_elem[0] != "c" && line_elem[0] != "0" {
            if line_elem[0] == "p" {
                variables_qty = line_elem[2].parse::<usize>().unwrap();
                for _i in 0..variables_qty {
                    let variable = Variable {
                        value: 2,
                        pos_watched_occ: Vec::<usize>::new(),
                        neg_watched_occ: Vec::<usize>::new(),
                        priority: [0, 0],
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
        };
        if remove_duplicate_vars(&mut chunk.to_vec()) {
            for lit_p in chunk {
                let lit = *lit_p;
                clause.variables.push((lit.abs()) as usize);
                clause.signs.push(lit > 0);
                if lit > 0 {
                    variables[(lit.abs()) as usize].priority[0] += 1;
                } else {
                    variables[(lit.abs()) as usize].priority[1] += 1;
                }
                if clause.variables.len() <= 2 {
                    clause.watched[clause.variables.len() - 1] =
                        clause.variables[clause.variables.len() - 1];
                    if lit > 0 {
                        variables[(lit.abs()) as usize]
                            .pos_watched_occ
                            .push(clauses.len());
                    } else {
                        variables[(lit.abs()) as usize]
                            .neg_watched_occ
                            .push(clauses.len());
                    }
                }
            }
        }
        clauses.push(clause);
    }
}

fn preprocessing(
    clauses: &mut Vec<Clause>,
    deleted_clauses: &mut Vec<usize>,
    mut variables: &mut Vec<Variable>,
) -> bool {
    println!("Preprocessing");
    let mut i = 0;
    while i < clauses.len() {
        if clauses[i].variables.len() == 1 && !deleted_clauses.contains(&i) {
            let var_index = clauses[i].variables[0];
            variables[var_index].value = clauses[i].signs[0] as usize;
            for j in 0..clauses.len() {
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
                                clauses[j].variables.remove(x);
                                clauses[j].signs.remove(x);
                            } else {
                                double_unwatch(&clauses, &mut variables, j);
                                clauses[j].variables.swap_remove(x);
                                clauses[j].signs.swap_remove(x);
                                clauses[j].watched[0] = clauses[j].variables[0];
                                clauses[j].watched[1] = clauses[j].variables[1];
                                if clauses[j].signs[0] {
                                    variables[clauses[j].watched[0]].pos_watched_occ.push(j);
                                } else {
                                    variables[clauses[j].watched[0]].neg_watched_occ.push(j);
                                }
                                if clauses[j].signs[1] {
                                    variables[clauses[j].watched[1]].pos_watched_occ.push(j);
                                } else {
                                    variables[clauses[j].watched[1]].neg_watched_occ.push(j);
                                }
                            }
                        }
                    }
                    None => {}
                }
            }
            i = 0;
        } else {
            i += 1;
        }
    }
    return true;
}

fn double_unwatch(clauses: &Vec<Clause>, mut variables: &mut Vec<Variable>, clause_index: usize) {
    println!("{:?}",clauses[clause_index]);
    let watched0_index = clauses[clause_index]
        .variables
        .iter()
        .position(|&x| x == clauses[clause_index].watched[0])
        .unwrap();
    let watched1_index = clauses[clause_index]
        .variables
        .iter()
        .position(|&x| x == clauses[clause_index].watched[1])
        .unwrap();
    unwatch(
        &mut variables,
        clause_index,
        clauses[clause_index].watched[0],
        clauses[clause_index].signs[watched0_index],
    );
    unwatch(
        &mut variables,
        clause_index,
        clauses[clause_index].watched[1],
        clauses[clause_index].signs[watched1_index],
    );
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
                    println!("Conflict 1!");
                    return resolve_conflict(
                        &mut clauses,
                        &mut variables,
                        &mut queue,
                        &mut backtracking_stack,
                        &mut deleted_clauses,
                        &mut cur_depth,
                        clause_index,
                    );
                } else {
                    if new_watched != 0 {
                        // found new watched
                        if clauses[clause_index].watched[0] == variable_index {
                            clauses[clause_index].watched[0] = new_watched;
                        } else {
                            clauses[clause_index].watched[1] = new_watched;
                        }
                        if clauses[clause_index].signs[new_watched_index] {
                            variables[new_watched].pos_watched_occ.push(clause_index);
                        } else {
                            variables[new_watched].neg_watched_occ.push(clause_index);
                        }
                        unwatch(&mut variables, clause_index, variable_index, !value);
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
        );
    }
    return true;
}

fn resolve_conflict(
    mut clauses: &mut Vec<Clause>,
    mut variables: &mut Vec<Variable>,
    queue: &mut Vec<(usize, bool, bool, usize)>,
    backtracking_stack: &mut Vec<(usize, usize, bool, usize)>,
    mut deleted_clauses: &mut Vec<usize>,
    cur_depth: &mut usize,
    clause_index: usize,
) -> bool {
    // print_clauses(&clauses, &deleted_clauses, &variables);
    // print_variables(&variables);
    // println!("Queue: {:?}", queue);
    // println!("Backtracking stack: {:?} {}", backtracking_stack,backtracking_stack.len());
    // println!("Deleted clauses: {:?}", deleted_clauses);
    if backtracking_stack[backtracking_stack.len() - 1].1 == 1 {
        let first_var = backtracking_stack[0].0;
        let unit_clause = Clause {
            variables: vec![first_var],
            signs: vec![variables[first_var].value == 0], // first value wrong due to conflict
            watched: [0 as usize, 0 as usize],
        };
        clause_learning(&mut clauses, &mut deleted_clauses, &unit_clause);
        while backtracking_stack.len() > 0 {
            variables[backtracking_stack.pop().unwrap().0].value = 2;
        }
        *cur_depth = 0 as usize;
        return preprocessing(&mut clauses, &mut deleted_clauses, &mut variables);
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
    let uip_index = backtracking_stack.pop().unwrap().0;
    let new_uip_value = variables[uip_index].value == 0;
    variables[uip_index].value = 2; // set the uip free
    while !resolvent
        .variables
        .contains(&backtracking_stack[backtracking_stack.len() - 1].0)
    {
        // non-chronological backtracking
        variables[backtracking_stack.pop().unwrap().0].value = 2;
    }
    resolvent.watched[0] = uip_index;
    resolvent.watched[1] = backtracking_stack[backtracking_stack.len() - 1].0;

    let new_clause_index = clause_learning(&mut clauses, &mut deleted_clauses, resolvent);
    if new_uip_value {
        // uip must be true with unit prop, so sign = value
        variables[uip_index].pos_watched_occ.push(new_clause_index);
    } else {
        variables[uip_index].neg_watched_occ.push(new_clause_index);
    }
    if variables[clauses[new_clause_index].watched[1]].value == 0 {
        // because all literals except uip in the clause are false, the sign = !value
        variables[clauses[new_clause_index].watched[1]]
            .pos_watched_occ
            .push(new_clause_index);
    } else {
        variables[clauses[new_clause_index].watched[1]]
            .neg_watched_occ
            .push(new_clause_index);
    }
    update_r(&clauses, &mut variables, new_clause_index);
    *cur_depth = backtracking_stack[backtracking_stack.len() - 1].1;
    queue.clear();
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

fn clause_learning(
    clauses: &mut Vec<Clause>,
    deleted_clauses: &mut Vec<usize>,
    new_clause: &Clause,
) -> usize {
    let copy = copy_clause(new_clause);
    let new_clause_index: usize;
    if deleted_clauses.len() > 0 {
        new_clause_index = deleted_clauses.pop().unwrap();
        clauses[new_clause_index] = copy; // replace one deleted clause with the new clause
    } else {
        clauses.push(copy);
        new_clause_index = clauses.len() - 1;
    }
    new_clause_index
}

fn copy_clause(clause: &Clause) -> Clause {
    let mut copy = Clause {
        variables: Vec::<usize>::new(),
        signs: Vec::<bool>::new(),
        watched: [0 as usize, 0 as usize],
    };
    for i in 0..clause.variables.len() {
        copy.variables.push(clause.variables[i]);
        copy.signs.push(clause.signs[i]);
    }
    copy.watched[0] = clause.watched[0];
    copy.watched[1] = clause.watched[1];
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
    println!("{} unwatched for {}", clause_index, variable_index);
}

fn vsids(variables: &Vec<Variable>, cur_depth: &mut usize) -> (usize, bool) {
    let mut variable_index = 0;
    let mut max_priority = 0;
    let mut value = false;
    for i in 1..variables.len() {
        if variables[i].priority[0] > max_priority && variables[i].value == 2 {
            variable_index = i;
            max_priority = variables[i].priority[0];
            value = true;
        }
        if variables[i].priority[1] > max_priority && variables[i].value == 2 {
            variable_index = i;
            max_priority = variables[i].priority[1];
            value = false;
        }
    }
    *cur_depth += 1;
    (variable_index, value)
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

fn check_watched(clauses: &Vec<Clause>, deleted_clauses:&Vec<usize>,variables: &Vec<Variable>) {
    for i in 0..clauses.len() {
        println!("{} {:?}",i,clauses[i]);
        if !deleted_clauses.contains(&i){
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
}
