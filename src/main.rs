// mod tents;
// use std::time::Instant;
// use std::env;

// fn main() {
//     let args: Vec<String> = env::args().collect();
//     //println!("{:?}", args);
//     let start = Instant::now();
//     let mut sat_maker:tents::SatMaker;
//     if args.len() == 1 {
//         sat_maker = tents::SatMaker::new("src/tents.txt");
//     } else {
//         sat_maker = tents::SatMaker::new(&args[1]);
//     }
//     let mut duration = start.elapsed();
//     println!("encoding time: {:?}", duration);
//     sat_maker.solve_sat();
//     let duration1 = start.elapsed();
//     println!("solving time: {:?}", duration1-duration);
//     if args.contains(&"unique".to_string()) {
//         sat_maker.unique_check();
//     }
// }


fn main(){
}
struct Clause {
    variables:Vec<usize>,
    signs:Vec<bool>,
    satisfied_by:usize,
    free_variables_qty:usize,
}

struct Variable {
    name:usize,
    value:usize,
    positive_occurrence:Vec<usize>,
    negative_occurrence:Vec<usize>,
    positive_occurrence_not_satisfied_qty:usize,
    negative_occurrence_not_satisfied_qty:usize,
}

struct DPLLP{
    clauses:Vec<Clause>,
    variables:Vec<Variable>,
    queue:Vec<(usize,bool)>,
    back_tracking_stack:Vec<(usize,bool)>,
    conflict:bool,
}

impl DPLLP{



fn insert(&self,variable:Variable,value:bool,decided:bool){    
    if value && variable.value==2{
        variable.value=value as usize;
        for clause_index in variable.positive_occurrence{
            if self.clauses[clause_index].satisfied_by==0{
                self.clauses[clause_index].free_variables_qty-=1;
                if self.clauses[clause_index].free_variables_qty==0{
                    self.conflict=true;
                } else if self.clauses[clause_index].free_variables_qty==1{
                    self.queue.push(self.get_unit_drop(clause_index));
                }
                self.clauses[clause_index].satisfied_by==variable.name;
                for index in 0..self.clauses[clause_index].variables.len(){
                    let variable_index = self.clauses[clause_index].variables[index];
                    if self.variables[variable_index].value==2{
                        if self.clauses[clause_index].signs[index]{
                            self.variables[variable_index].positive_occurrence_not_satisfied_qty-=1;
                            if self.variables[variable_index].positive_occurrence_not_satisfied_qty==0{
                                self.queue.push((variable_index,false));
                            }
                        } else {
                            self.variables[variable_index].negative_occurrence_not_satisfied_qty-=1;
                            if self.variables[variable_index].negative_occurrence_not_satisfied_qty==0{
                                self.queue.push((variable_index,true));
                            }
                        }    
                    }                                                  
                }
            }
        }
        for clause_index in variable.negative_occurrence{
            self.clauses[clause_index].free_variables_qty-=1;
        }
        self.back_tracking_stack.push((variable.name,decided))
    } else if !value && variable.value==2{
        variable.value=value as usize;
        for clause_index in variable.positive_occurrence{
            self.clauses[clause_index].free_variables_qty-=1;
        }
        for clause_index in variable.negative_occurrence{
            self.clauses[clause_index].free_variables_qty-=1;
            if self.clauses[clause_index].satisfied_by==0{
                if self.clauses[clause_index].free_variables_qty==0{
                    self.conflict=true;
                } else if self.clauses[clause_index].free_variables_qty==1{
                    self.queue.push(self.get_unit_drop(clause_index));
                }
                self.clauses[clause_index].satisfied_by==variable.name;
                for index in 0..self.clauses[clause_index].variables.len(){
                    let variable_index = self.clauses[clause_index].variables[index];
                    if self.variables[variable_index].value==2{
                        if self.clauses[clause_index].signs[index]{
                            self.variables[variable_index].positive_occurrence_not_satisfied_qty-=1;
                            if self.variables[variable_index].positive_occurrence_not_satisfied_qty==0{
                                self.queue.push((variable_index,false));
                            }
                        } else {
                            self.variables[variable_index].negative_occurrence_not_satisfied_qty-=1;
                            if self.variables[variable_index].negative_occurrence_not_satisfied_qty==0{
                                self.queue.push((variable_index,true));
                            }
                        }    
                    }                                                  
                }
            }
        }
        self.back_tracking_stack.push((variable.name,decided))
    } else if variable.value!=value as usize{
        self.conflict=true
    }
}

fn get_unit_drop(&self,clause_index:usize)->(usize,bool){
    for index in 0..self.clauses[clause_index].variables.len(){
        let variable_index=self.clauses[clause_index].variables[index];
        if self.variables[variable_index].value==2{
            return (variable_index, self.clauses[clause_index].signs[index])
        }
    }
}

fn dlis(&self)->(usize,bool){
    let mut variable_index=0;
    let mut max_occurrence=0;
    let mut value=true;
    for variable in self.variables{
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
    return (variable_index,value)
}

fn back_track(&self){
    let (mut variable_index, mut decided)=self.back_tracking_stack.pop().unwrap();
    while !decided{
        self.variables[variable_index].value=2;
        for clause_index in self.variables[variable_index].positive_occurrence{
            if self.clauses[clause_index].satisfied_by==variable_index{
                self.clauses[clause_index].free_variables_qty+=1;
                self.clauses[clause_index].satisfied_by=0;
                for index in 0..self.clauses[clause_index].variables.len(){
                    if self.variables[self.clauses[clause_index].variables[index]].value==2{
                        if self.clauses[clause_index].signs[index]{
                            self.variables[self.clauses[clause_index].variables[index]].positive_occurrence_not_satisfied_qty+=1;
                        }else{
                            self.variables[self.clauses[clause_index].variables[index]].negative_occurrence_not_satisfied_qty+=1;
                        }
                    }
                   
                }
            }else if self.clauses[clause_index].satisfied_by==0{
                self.clauses[clause_index].free_variables_qty+=1;
            }
        }
        let tup=self.back_tracking_stack.pop().unwrap();
        variable_index=tup.0;
        decided=tup.1;
        
    }
    self.variables[variable_index].value=2;
    for clause_index in self.variables[variable_index].positive_occurrence{
        if self.clauses[clause_index].satisfied_by==variable_index{
            self.clauses[clause_index].free_variables_qty+=1;
            self.clauses[clause_index].satisfied_by=0;
            for index in 0..self.clauses[clause_index].variables.len(){
                if self.variables[self.clauses[clause_index].variables[index]].value==2{
                    if self.clauses[clause_index].signs[index]{
                        self.variables[self.clauses[clause_index].variables[index]].positive_occurrence_not_satisfied_qty+=1;
                    }else{
                        self.variables[self.clauses[clause_index].variables[index]].negative_occurrence_not_satisfied_qty+=1;
                    }
                }
                
            }
        }else if self.clauses[clause_index].satisfied_by==0{
            self.clauses[clause_index].free_variables_qty+=1;
        }
    }
    self.insert(self.variables[variable_index],self.variables[variable_index].value ==0,false);
}

fn dpll(&self)->Vec<Variable>{
    while true{
        let (mut next_variable,mut next_value)=self.dlis();
        self.insert(self.variables[next_variable],next_value,true);
        while self.queue.len()>0{
            let tup=self.queue.pop().unwrap();
            next_variable=tup.0;
            next_value=tup.1;
            self.insert(self.variables[next_variable],next_value,false);
            while self.conflict{
                self.queue.clear();
                self.back_track();
                self.conflict=false;
            }
        }
        let mut all_set=true;
        for variable in self.variables{
            if variable.value==2{
                all_set=false
            }
        }
        if all_set{
             return self.variables
        }
    }
}
}

