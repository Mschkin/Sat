struct Clause {
    variables:vec<usize>,
    signs:vec<bool>,
    satisfied_by:usize,
    free_variables_qty:usize,
}

struct Variable {
    name:usize,
    value:usize,
    positive_occurrence:vec<usize>,
    negative_occurrence:vec<usize>,
    positive_occurrence_not_satisfied_qty:usize,
    negative_occurrence_not_satisfied_qty:usize,
}

let mut clauses=Vec::<Clause>::new();
let mut variables=Vec::<Variable>::new();
let mut queue=Vec::<(usize,bool)>::new();
let mut back_tracking_stack=Vec::<(usize,bool)>::new();
let mut conflict=false;

fn insert(variable:Variable,value:bool,decided:bool){    
    if value && variable.value==2{
        variable.value=value;
        for clause_index in variable.positive_occurrence{
            if clauses[clause_index].satisfied_by==0{
                clauses[clause_index].free_variables_qty-=1;
                if clauses[clause_index].free_variables_qty==0{
                    conflict=true;
                } else if clauses[clause_index].free_variables_qty==1{
                    queue.push(get_unit_drop(clause_index));
                }
                clauses[clause_index].satisfied_by==variable.name;
                for index in 0..clauses[clause_index].variables.len(){
                    let variable_index = clauses[clause_index].variables[index];
                    if variables[variable_index].value==2{
                        if clauses[clause_index].signs[index]{
                            variables[variable_index].positive_occurrence_not_satisfied_qty-=1;
                            if variables[variable_index].positive_occurrence_not_satisfied_qty==0{
                                queue.push((variable_index,false));
                            }
                        } else {
                            variables[variable_index].negative_occurrence_not_satisfied_qty-=1;
                            if variables[variable_index].negative_occurrence_not_satisfied_qty==0{
                                queue.push((variable_index,true));
                            }
                        }    
                    }                                                  
                }
            }
        }
        for clause_index in variable.negative_occurrence{
            clauses[clause_index].free_variables_qty-=1;
        }
        back_tracking_stack.push((variable.name,decided)
    } else if !value && variable.value==2{
        variable.value=value;
        for clause_index in variable.positive_occurrence{
            clauses[clause_index].free_variables_qty-=1;
        }
        for clause_index in variable.negative_occurrence{
            clauses[clause_index].free_variables_qty-=1;
            if clauses[clause_index].satisfied_by==0{
                if clauses[clause_index].free_variables_qty==0{
                    conflict=true;
                } else if clauses[clause_index].free_variables_qty==1{
                    queue.push(get_unit_drop());
                }
                clauses[clause_index].satisfied_by==variable.name;
                for index in 0..clauses[clause_index].variables.len(){
                    let variable_index = clauses[clause_index].variables[index];
                    if variables[variable_index].value==2{
                        if clauses[clause_index].signs[index]{
                            variables[variable_index].positive_occurrence_not_satisfied_qty-=1;
                            if variables[variable_index].positive_occurrence_not_satisfied_qty==0{
                                queue.push((variable_index,false));
                            }
                        } else {
                            variables[variable_index].negative_occurrence_not_satisfied_qty-=1;
                            if variables[variable_index].negative_occurrence_not_satisfied_qty==0{
                                queue.push((variable_index,true));
                            }
                        }    
                    }                                                  
                }
            }
        }
        back_tracking_stack.push((variable.name,decided))
    } else if variable.value!=value{
        conflict=true
    }
}

fn get_unit_drop(clause_index:usize)->(usize,bool){
    for index in 0..clauses[clause_index].variables.len(){
        let variable_index=clauses[clause_index].variables[index];
        if variables[variable_index].value==2{
            (variable_index, clauses[clause_index].signs[index])
        }
    }
}

fn dlis()->(usize,bool){
    let mut variable_index=0;
    let mut max_occurrence=0;
    let mut value=true;
    for variable in variables{
        if variable.positive_occurrence_not_satisfied_qty>max_occurrence{
            variable_index=variable.name;
            max_occurrence=variable.positive_occurrence_not_satisfied_qty;
            value=true;
        }
        if variable.negative_occurrence_not_satisfied_qty>max_occurrence{
            variable_index=variable.name;
            max_occurrence=variable.negative_occurrence_not_satisfied_qty;
            value=false;
        }
    }
    (variable_index,value)
}

fn back_track(){
    let mut variable_index, decided=back_tracking_stack.pop();
    while !decided{
        variables[variable_index].value=2;
        for clause_index in variables[variable_index].positive_occurrence{
            if clauses[clause_index].satisfied_by==variable_index{
                clauses[clause_index].free_variables_qty++;
                clauses[clause_index].satisfied_by=0;
                for index in 0..clauses[clause_index].variables{
                    if variables[clauses[clause_index].variables[index]].value==2{
                        if clauses[clause_index].signs[index]{
                            variables[clauses[clause_index].variables[index]].positive_occurrence_not_satisfied_qty++;
                        }else{
                            variables[clauses[clause_index].variables[index]].negative_occurrence_not_satisfied_qty++;
                        }
                    }
                   
                }
            }else if clauses[clause_index].satisfied_by==0{
                clauses[clause_index].free_variables_qty++;
            }
        }
        variable_index, decided=back_tracking_stack.pop();
    }
    variables[variable_index].value=2;
    for clause_index in variables[variable_index].positive_occurrence{
        if clauses[clause_index].satisfied_by==variable_index{
            clauses[clause_index].free_variables_qty++;
            clauses[clause_index].satisfied_by=0;
            for index in 0..clauses[clause_index].variables{
                if variables[clauses[clause_index].variables[index]].value==2{
                    if clauses[clause_index].signs[index]{
                        variables[clauses[clause_index].variables[index]].positive_occurrence_not_satisfied_qty++;
                    }else{
                        variables[clauses[clause_index].variables[index]].negative_occurrence_not_satisfied_qty++;
                    }
                }
                
            }
        }else if clauses[clause_index].satisfied_by==0{
            clauses[clause_index].free_variables_qty++;
        }
    }
    insert(variables[variable_index],!variables[variable_index].value,false);
}

fn dpll()->vec<Variables>{
    while true{
        let mut next_variable,next_value=dlis();
        insert(varibles[next_variable],next_value,true);
        while queue.len()>0{
            next_variable,next_value=queue.pop();
            insert(varibles[next_variable],next_value,false);
            while conflict{
                queue.clear();
                back_track();
                conflict=false;
            }
        }
        let mut all_set=true;
        for variable in variables{
            if variable.value==2{
                all_set=false
            }
        }
        if all_set{
            variables
        }
    }
}


