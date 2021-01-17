struct Clause {
    variables:Vec<usize>,
    signs:Vec<bool>,
    sat_by:usize,
    free_variables_qty:usize,
}

struct Variable {
    name:usize,
    value:usize,
    pos_occ:Vec<usize>,
    neg_occ:Vec<usize>,
    pos_occ_not_sat_qty:usize,
    neg_occ_not_sat_qty:usize,
}

struct DPLLP{
    clauses:Vec<Clause>,
    variables:Vec<Variable>,
    queue:Vec<(usize,bool)>,
    backtracking_stack:Vec<(usize,bool)>,
    conflict:bool,
}

impl DPLLP{



fn set_value(&mut self,mut variable:Variable,value:bool,forced:bool){    
    if value && variable.value==2{
        variable.value=value as usize;
        for clause_index in variable.pos_occ{
            if self.clauses[clause_index].sat_by==0{
                self.clauses[clause_index].sat_by==variable.name;
                // pure lit
                for index in 0..self.clauses[clause_index].variables.len(){
                    let variable_index = self.clauses[clause_index].variables[index];
                    if self.variables[variable_index].value==2{
                        if self.clauses[clause_index].signs[index]{
                            self.variables[variable_index].pos_occ_not_sat_qty-=1;
                            if self.variables[variable_index].pos_occ_not_sat_qty==0{
                                self.queue.push((variable_index,false));
                            }
                        } else {
                            self.variables[variable_index].neg_occ_not_sat_qty-=1;
                            if self.variables[variable_index].neg_occ_not_sat_qty==0{
                                self.queue.push((variable_index,true));
                            }
                        }    
                    }                                                  
                }
            }
        }
        for clause_index in variable.neg_occ{
            if self.clauses[clause_index].sat_by==0{
                self.clauses[clause_index].free_variables_qty-=1;
                if self.clauses[clause_index].free_variables_qty==0{
                    self.conflict=true;
                } else if self.clauses[clause_index].free_variables_qty==1{
                    self.queue.push(self.get_unit_prop(clause_index));
                }
            }           
        }
        self.backtracking_stack.push((variable.name,forced))
    } else if !value && variable.value==2{
        variable.value=value as usize;
        for clause_index in variable.pos_occ{
            self.clauses[clause_index].free_variables_qty-=1;
        }
        for clause_index in variable.neg_occ{
            self.clauses[clause_index].free_variables_qty-=1;
            if self.clauses[clause_index].sat_by==0{
                if self.clauses[clause_index].free_variables_qty==0{
                    self.conflict=true;
                } else if self.clauses[clause_index].free_variables_qty==1{
                    self.queue.push(self.get_unit_prop(clause_index));
                }
                self.clauses[clause_index].sat_by==variable.name;
                for index in 0..self.clauses[clause_index].variables.len(){
                    let variable_index = self.clauses[clause_index].variables[index];
                    if self.variables[variable_index].value==2{
                        if self.clauses[clause_index].signs[index]{
                            self.variables[variable_index].pos_occ_not_sat_qty-=1;
                            if self.variables[variable_index].pos_occ_not_sat_qty==0{
                                self.queue.push((variable_index,false));
                            }
                        } else {
                            self.variables[variable_index].neg_occ_not_sat_qty-=1;
                            if self.variables[variable_index].neg_occ_not_sat_qty==0{
                                self.queue.push((variable_index,true));
                            }
                        }    
                    }                                                  
                }
            }
        }
        self.backtracking_stack.push((variable.name,forced))
    } else if variable.value!=value as usize{
        self.conflict=true
    }
}

fn get_unit_prop(&self,clause_index:usize)->(usize,bool){
    for index in 0..self.clauses[clause_index].variables.len(){
        let variable_index=self.clauses[clause_index].variables[index];
        if self.variables[variable_index].value==2{
            return (variable_index, self.clauses[clause_index].signs[index]);
        }
    }
    (0,true)
}

fn dlis(&self)->(usize,bool){
    let mut variable_index=0;
    let mut max_occurrence=0;
    let mut value=true;
    for variable in &self.variables{
        if variable.pos_occ_not_sat_qty>max_occurrence{
            variable_index=variable.name;
            max_occurrence=variable.pos_occ_not_sat_qty;
            value=true;
        }
        if variable.neg_occ_not_sat_qty>max_occurrence{
            variable_index=variable.name;
            max_occurrence=variable.neg_occ_not_sat_qty;
            value=false;
        }
    }
    return (variable_index,value)
}

fn back_track(&mut self){
    let (mut variable_index, mut forced)=self.backtracking_stack.pop().unwrap();
    while forced{
        self.variables[variable_index].value=2;
        for i in 0..self.variables[variable_index].pos_occ.len(){
            let clause_index=self.variables[variable_index].pos_occ[i];
            if self.clauses[clause_index].sat_by==variable_index{
                self.clauses[clause_index].sat_by=0;
                for j in 0..self.clauses[clause_index].variables.len(){
                    if self.variables[self.clauses[clause_index].variables[j]].value==2{
                        if self.clauses[clause_index].signs[j]{
                            self.variables[self.clauses[clause_index].variables[j]].pos_occ_not_sat_qty+=1;
                        }else{
                            self.variables[self.clauses[clause_index].variables[j]].neg_occ_not_sat_qty+=1;
                        }
                    }
                   
                }
            }else if self.clauses[clause_index].sat_by==0{
                self.clauses[clause_index].free_variables_qty+=1;
            }
        }
        if self.backtracking_stack.len()==0{
            println!("UNSAT");
            return ();
        }
        let tup=self.backtracking_stack.pop().unwrap();
        variable_index=tup.0;
        forced=tup.1;
        
    }
    self.variables[variable_index].value=2;
    for clause_index in &self.variables[variable_index].pos_occ{
        if self.clauses[*clause_index].sat_by==variable_index{
            self.clauses[*clause_index].free_variables_qty+=1;
            self.clauses[*clause_index].sat_by=0;
            for index in 0..self.clauses[*clause_index].variables.len(){
                if self.variables[self.clauses[*clause_index].variables[index]].value==2{
                    if self.clauses[*clause_index].signs[index]{
                        self.variables[self.clauses[*clause_index].variables[index]].pos_occ_not_sat_qty+=1;
                    }else{
                        self.variables[self.clauses[*clause_index].variables[index]].neg_occ_not_sat_qty+=1;
                    }
                }
                
            }
        }else if self.clauses[*clause_index].sat_by==0{
            self.clauses[*clause_index].free_variables_qty+=1;
        }
    }
    self.set_value(self.variables[variable_index],self.variables[variable_index].value ==0,true);
}

fn dpll(&mut self)->Vec<Variable>{
    while true{
        let (mut next_variable,mut next_value)=self.dlis();
        self.set_value(self.variables[next_variable],next_value,false);
        while self.queue.len()>0{
            let tup=self.queue.pop().unwrap();
            next_variable=tup.0;
            next_value=tup.1;
            self.set_value(self.variables[next_variable],next_value,true);
            while self.conflict{
                self.queue.clear();
                self.back_track();
                self.conflict=false;
            }
        }
        let mut all_set=true;
        for variable in &self.variables{
            if variable.value==2{
                all_set=false
            }
        }
        if all_set{
             return self.variables;
        }
    }
    self.variables
}
}