#[derive(Debug)]
pub struct Clause {
    variables: Vec<usize>,
    signs: Vec<bool>,
    watched: [usize;2],
}

#[derive(Debug)]
pub struct Variable {
    value: usize,
    pos_watched_occ: Vec<usize>,
    neg_watched_occ: Vec<usize>,
    priority:[usize;2], // for + and -
    r:[usize;2], // for + and -
}

pub fn cdcl(path:&str){
    let mut clauses = Vec::<Clause>::new();
    let mut variables = Vec::<Variable>::new();
    let mut queue = Vec::<(usize, bool,bool,usize)>::new(); // var_index,var_val,forced,clause_index(reason)
    let mut backtracking_stack=Vec::<(usize,usize,bool,usize)>::new(); // var_index,depth,forced,clause_index
    let mut cur_depth=0;
    init(path,&mut clauses,&mut variables,&mut queue);
    println!("{:?}", clauses);
    (next_variable,next_value)=vsids();
    queue.push((next_variable,next_value,false,0));
    while queue.len()>0{
        set_value();
    }
}

fn read_file(path: &str) -> String {
    use std::io::Read;
    let mut file = std::fs::File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}

fn init(path:&str,clauses:&mut Vec<Clause>,variables:&mut Vec<Variable>,queue:&mut Vec<(usize, bool,usize,usize)>){
    let dummy_var=Variable {
                        value: 2,
                        pos_watched_occ: Vec::<usize>::new(),
                        neg_watched_occ: Vec::<usize>::new(),
                        priority:[0,0],
                        r:[0,0],
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
                        priority:[0,0],
                        r:[0,0],
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
            watched: [0 as usize,0 as usize],
        };
        for lit_p in chunk {
            let lit = *lit_p;
            clause.variables.push((lit.abs()) as usize);
            clause.signs.push(lit > 0);
            if lit>0 {
                variables[(lit.abs()) as usize].priority[0]+=1;
            } else {
                variables[(lit.abs()) as usize].priority[1]+=1;
            }
            if clause.variables.len()<=2{
                clause.watched[clause.variables.len()-1]=clause.variables[clause.variables.len()-1];
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
        clauses.push(clause);
    }
}

fn set_value(clauses:&mut Vec<Clause>,variables:&mut Vec<Variable>) {
    let tup=queue.pop();
    let variable_index=tup.0;
    let value=tup.1;
    let forced=tup.2;
    let reason=tup.3;
    if variables[variable_index].value == 2 {
        variables[variable_index].value = value as usize;
        let range:usize;
        if value{
            range=variables[variable_index].neg_watched_occ.len()
        }else{
            range=variables[variable_index].pos_watched_occ.len()
        }
        for i in 0..range {
            let clause_index:usize;
            if value{
                clause_index=variables[variable_index].neg_watched_occ[i];
            }else{
                clause_index=variables[variable_index].pos_watched_occ[i];
            }            
            let mut conflict=true;
            let mut sat=false;
            let mut new_watched=0;
            let unit_var:usize;
            let unit_sign:bool;
            for j in 0..clauses[clause_index].variables.len(){
                let other_var=clauses[clause_index].variables[j];
                if variables[other_var].value==clauses[clause_index].signs[j]{
                    sat=true;
                    break; // clause sat
                } else if variables[other_var].value==2 {
                    conflict=false;
                    if !clauses[clause_index].watched.contains(other_var){
                        new_watched=other_var
                    }else if other_var != variable_index{
                        unit_var=other_var;
                        unit_sign=clauses[clause_index].signs[j];
                    }
                }
            }
            if !sat{
                if conflict{
                    resolve_conflict(clause_index);
                }else{
                    if new_watched !=0{ // found new watched
                        if clauses[clause_index].watched[0]==variable_index{
                            clauses[clause_index].watched[1]=new_watched;
                        } else {
                            clauses[clause_index].watched[0]=new_watched;
                        }
                        if clauses[clause_index].signs[j]{
                            variables[new_watched].pos_watched_occ.push(clause_index);
                        }else{
                            variables[new_watched].neg_watched_occ.push(clause_index);
                        }
                        if value{
                            variables[variable_index].neg_watched_occ.retain(|x| *x!=clause_index);
                        }else{
                            variables[variable_index].pos_watched_occ.retain(|x| *x!=clause_index);
                        }                       
                    }else{
                        // unit prop
                        queue.push((unit_var,unit_sign,true,clause_index));
                    }
                }
            }
        }
        backtracking_stack.push((variable_index, cur_depth,forced,reason));
    } else if variables[variable_index].value != value as usize {   
        resolve_conflict(reason);
    }
}

fn resolve_conflict(clause_index:usize){
    last_reason=backtracking_stack.pop().3;
}

fn vsids(variables:&Vec<Variable>) -> (usize, bool){
    let mut variable_index = 0;
    let mut max_priority = 0;
    let mut value = false;
    for i in 1..variables.len() {
        if variables[i].priority[0] > max_priority
            && variables[i].value == 2
        {
            variable_index = i;
            max_priority = variables[i].priority[0];
            value = true;
        }
        if variables[i].priority[1] > max_priority
            && variables[i].value == 2
        {
            variable_index = i;
            max_priority = variables[i].priority[1];
            value = false;
        }
    }
    cur_depth+=1;
    (variable_index, value)
}