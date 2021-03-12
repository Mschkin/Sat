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
    let mut deleted_clauses=Vec::<usize>::new();
    init(path,&mut clauses,&mut variables,&mut queue);
    while !unsat{
        (next_variable,next_value)=vsids();
        if next_variable==0{
            break;
        }
        queue.push((next_variable,next_value,false,0));
        while queue.len()>0{
            set_value();
        }
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
        if remove_duplicate_vars(chunk) {
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
        }}
        clauses.push(clause);
    }
}

fn preprocessing(){
    let mut i=0;
    while i<clauses.len(){
        if clauses[i].variables.len()==1 && !deleted_clauses.contains(i){
            let var_index=clauses[i].variables[0];
            variables[var_index].value=clauses[i].signs[0];
            for j in 0..clauses.len(){
                let found=clauses[j].variables.iter().position(|x| *x==var_index);
                match found{
                    Some(x)=>if clauses[j].signs[x]==variables[var_index].value{
                        deleted_clauses.push(j);
                        unwatch(j,clauses[j].watched[0],);
                        unwatch(j,clauses[j].watched[1],);
                    }else{

                    }
                }
            }
        }
    }
}

fn remove_duplicate_vars(chunk:Vec<i32>)->bool{
    chunk.sort();
    let mut new_chunk=vec![chunk[0]];
    for i in chunk{
        if i != new_chunk(new_chunk.len()-1){
            new_chunk.push(i);
        }
    }
    for i in new_chunk{
        if new_chunk.contains(-i){
            return false;
        }
    }
    chunk=new_chunk;
    return true;
}

fn set_value() {
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
                    return;
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
                        unwatch(clause_index,variable_index,!value);                   
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
    let mut tup:(usize,usize,bool,usize);
    let mut max_depth_var=Vec::<usize>::new();
    let mut index=backtracking_stack.len()-2; // remove the last one
    while index>=0 && backtracking_stack[index].1==cur_depth{
        max_depth_var.push(backtracking_stack[index].0);
        index-=1;
    }
    max_depth_var.rev();
    let mut resolvent=clauses[clause_index];
    while intersection(max_depth_var,resolvent.variables){ // resolvent is not asserting
        tup=backtracking_stack.pop();
        var_index=tup.0;
        last_reason=tup.3;
        resolvent=get_resolvent(resolvent,clauses[last_reason],var_index);
        max_depth_var.pop();
        variables[var_index].value=2;
    }
    let uip_index=backtracking_stack.pop().0;
    let new_uip_value=!variables[uip_index].value;
    variables[uip_index].value=2; // set the uip free
    while backtracking_stack.len()>0 && !resolvent.variables.contains(backtracking_stack[backtracking_stack.len()-1].0){ // non-chronological backtracking
        variables[backtracking_stack.pop().0].value=2;
    }
    resolvent.watched[0]=uip_index;
    resolvent.watched[1]=backtracking_stack[backtracking_stack.len()-1].0;
    let new_clause_index:usize;
    if deleted_clauses.len()>0{
        new_clause_index=deleted_clauses.pop();
        clauses[new_clause_index]=resolvent; // replace one deleted clause with the new clause
    }else{
        clauses.push(resolvent); // clause learning
        new_clause_index=clauses.len()-1;
    }
    if new_uip_value{ // uip must be true with unit prop, so sign = value
        variables[uip_index].pos_watched_occ(new_clause_index);
    }else{
        variables[uip_index].neg_watched_occ(new_clause_index);
    }
    if !variables[resolvent.watched[1]].value{ // because all literals except uip in the clause are false, the sign = !value
       variables[resolvent.watched[1]].pos_watched_occ(new_clause_index);
    }else{
        variables[resolvent.watched[1]].neg_watched_occ(new_clause_index);
    }
    update_r(new_clause_index);
    cur_depth=backtracking_stack[backtracking_stack.len()-1].1;
    queue.clear();
    queue.push((uip_index,new_uip_value,true,new_clause_index));
}

fn intersection(vec1:Vec<usize>,vec2:Vec<usize>)->bool{
    for i in vec1{
        for j in vec2{
            if vec1[i]==vec2[j]{
                return true;
            }
        }
    }
    return false;
}

fn get_resolvent(clause1:&Clause,clause2:&Clause,var_index:usize)->Clause{
    let mut resolvent=Clause{
        variables: Vec::<usize>::new(),
        signs: Vec::<bool>::new(),
        watched: [0 as usize,0 as usize],
    };
    for i in 0..clause1.variables.len(){
        if clause1.variables[i] != var_index{
            resolvent.variables.push(clause1.variables[i]);
            resolvent.signs.push(clause1.signs[i]);
        }
    }
    for i in 0..clause2.variables.len(){
        if clause2.variables[i] != var_index && !resolvent.variables.contains(clause2.variables[i]){
            resolvent.variables.push(clause2.variables[i]);
            resolvent.signs.push(clause2.signs[i]);
        }
    } 
    resolvent
}

fn update_r(clause_index:usize){
    for i in 0..clauses[clause_index].variables{
        if clauses[clause_index].signs[i]{
            clauses[clause_index].variables[i].r[0]+=1;
        }else{
            clauses[clause_index].variables[i].r[1]+=1;
        }
    }
}

fn unwatch(clause_index:usize,variable_index:usize,sign:bool){
    if sign{
        variables[variable_index].pos_watched_occ.retain(|x| *x!=clause_index);
    }else{
        variables[variable_index].neg_watched_occ.retain(|x| *x!=clause_index);
    }
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