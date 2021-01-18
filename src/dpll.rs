#[derive(Debug)]
struct Clause {
    variables: Vec<usize>,
    signs: Vec<bool>,
    sat_by: usize,
    free_variables_qty: usize,
}

#[derive(Debug)]
pub struct Variable {
    pub value: usize,
    pos_occ: Vec<usize>,
    neg_occ: Vec<usize>,
    pos_occ_not_sat_qty: usize,
    neg_occ_not_sat_qty: usize,
}

#[derive(Debug)]
pub struct DPLL {
    clauses: Vec<Clause>,
    variables: Vec<Variable>,
    queue: Vec<(usize, bool)>,
    backtracking_stack: Vec<(usize, bool)>,
    conflict: bool,
}

impl DPLL {
    pub fn new(path: &str) -> Self {
        let content: String = read_file(path);
        let content_clone = content.clone();
        let input: Vec<&str> = (&content_clone).split("\n").collect();
        let mut clauses = Vec::<Clause>::new();
        let mut variables = Vec::<Variable>::new();
        let mut queue = Vec::<(usize, bool)>::new();
        let mut clauses_qty:usize=0;
        let mut variables_qty:usize=0;
        for line_number in 0..input.len() {
            let line_elem:Vec<&str> = input[line_number].split_whitespace().collect();
            if line_elem.len()>1 && line_elem[0] != "c" {
                if line_elem[0] == "p" {
                    clauses_qty = line_elem[3].parse::<usize>().unwrap();
                    variables_qty = line_elem[2].parse::<usize>().unwrap();
                    for i in 0..variables_qty {
                        let variable = Variable {
                            value: 2,
                            pos_occ: Vec::<usize>::new(),
                            neg_occ: Vec::<usize>::new(),
                            pos_occ_not_sat_qty: 0,
                            neg_occ_not_sat_qty: 0,
                        };
                        variables.push(variable);
                    }
                } else {
                    // clauses
                    let mut clause = Clause {
                        variables: Vec::<usize>::new(),
                        signs: Vec::<bool>::new(),
                        sat_by: variables_qty,
                        free_variables_qty: 0,
                    };
                    for j in 0..line_elem.len()-1 {
                        let lit = line_elem[j].parse::<i32>().unwrap();
                        clause.variables.push((lit.abs() - 1) as usize);
                        clause.signs.push(lit > 0);
                        clause.free_variables_qty += 1;
                        if lit > 0 {
                            variables[(lit.abs() - 1) as usize]
                                .pos_occ
                                .push(clauses.len());
                            variables[(lit.abs() - 1) as usize].pos_occ_not_sat_qty += 1;
                        } else {
                            variables[(lit.abs() - 1) as usize]
                                .neg_occ
                                .push(clauses.len());
                            variables[(lit.abs() - 1) as usize].neg_occ_not_sat_qty += 1;
                        }
                    }
                    // unit prop
                    if clause.variables.len() == 1 { 
                        queue.push((clause.variables[0], clause.signs[0]));
                    }
                    clauses.push(clause);
                }
            }
        }

        // pure lit
        for i in 0..variables_qty {
            if variables[i].pos_occ_not_sat_qty == 0 {
                queue.push((i, false));
            }
            if variables[i].neg_occ_not_sat_qty == 0 {
                queue.push((i, true));
            }
        }
        DPLL {
            clauses: clauses,
            variables: variables,
            queue: queue,
            backtracking_stack: Vec::<(usize, bool)>::new(),
            conflict: false,
        }
    }

    fn set_value(&mut self, variable_index: usize, value: bool, forced: bool) {
        if value && self.variables[variable_index].value == 2 {
            self.variables[variable_index].value = value as usize;
            for i in 0..self.variables[variable_index].pos_occ.len() {
                let clause_index = self.variables[variable_index].pos_occ[i];
                if self.clauses[clause_index].sat_by == self.variables.len() {
                    self.clauses[clause_index].sat_by = variable_index;
                    self.pure_lit(clause_index);
                }
            }
            for j in 0..self.variables[variable_index].neg_occ.len() {
                self.xxx(self.variables[variable_index].neg_occ[j]);
            }
            self.backtracking_stack.push((variable_index, forced))
        } else if !value && self.variables[variable_index].value == 2 {
            self.variables[variable_index].value = value as usize;
            for i in 0..self.variables[variable_index].pos_occ.len() {
                self.xxx(self.variables[variable_index].pos_occ[i]);
            }
            for j in 0..self.variables[variable_index].neg_occ.len() {
                let clause_index = self.variables[variable_index].neg_occ[j];
                if self.clauses[clause_index].sat_by == self.variables.len() {
                    self.clauses[clause_index].sat_by = variable_index;
                    self.pure_lit(clause_index);
                }
            }
            self.backtracking_stack.push((variable_index, forced))
        } else if self.variables[variable_index].value != value as usize {
            self.conflict = true
        }
    }

    fn unset_value(&mut self, variable_index: usize) {
        self.variables[variable_index].value = 2;
        for i in 0..self.variables[variable_index].pos_occ.len() {
            let clause_index = self.variables[variable_index].pos_occ[i];
            if self.clauses[clause_index].sat_by == variable_index {
                self.clauses[clause_index].sat_by = self.variables.len();
                for j in 0..self.clauses[clause_index].variables.len() {
                    if self.variables[self.clauses[clause_index].variables[j]].value == 2 {
                        if self.clauses[clause_index].signs[j] {
                            self.variables[self.clauses[clause_index].variables[j]]
                                .pos_occ_not_sat_qty += 1;
                        } else {
                            self.variables[self.clauses[clause_index].variables[j]]
                                .neg_occ_not_sat_qty += 1;
                        }
                    }
                }
            } else if self.clauses[clause_index].sat_by == self.variables.len() {
                self.clauses[clause_index].free_variables_qty += 1;
            }
        }
        for i in 0..self.variables[variable_index].neg_occ.len() {
            let clause_index = self.variables[variable_index].neg_occ[i];
            if self.clauses[clause_index].sat_by == variable_index {
                self.clauses[clause_index].sat_by = self.variables.len();
                for j in 0..self.clauses[clause_index].variables.len() {
                    if self.variables[self.clauses[clause_index].variables[j]].value == 2 {
                        if self.clauses[clause_index].signs[j] {
                            self.variables[self.clauses[clause_index].variables[j]]
                                .pos_occ_not_sat_qty += 1;
                        } else {
                            self.variables[self.clauses[clause_index].variables[j]]
                                .neg_occ_not_sat_qty += 1;
                        }
                    }
                }
            } else if self.clauses[clause_index].sat_by == self.variables.len() {
                self.clauses[clause_index].free_variables_qty += 1;
            }
        }
    }

    fn xxx(&mut self, clause_index: usize) {
        if self.clauses[clause_index].sat_by == self.variables.len() {
            self.clauses[clause_index].free_variables_qty -= 1;
            let mut vals = 0;
            for i in 0..self.clauses[clause_index].variables.len() {
                if self.variables[self.clauses[clause_index].variables[i]].value != 2 {
                    vals += 1;
                }
            }
            //println!("{:?} and should be {}",self.clauses[clause_index],self.clauses[clause_index].variables.len()-vals);
            if self.clauses[clause_index].free_variables_qty == 0 {
                self.conflict = true;
            } else if self.clauses[clause_index].free_variables_qty == 1 {
                self.queue.push(self.get_unit_prop(clause_index));
            }
        }
    }

    fn get_unit_prop(&self, clause_index: usize) -> (usize, bool) {
        for index in 0..self.clauses[clause_index].variables.len() {
            let variable_index = self.clauses[clause_index].variables[index];
            if self.variables[variable_index].value == 2 {
                return (variable_index, self.clauses[clause_index].signs[index]);
            }
        }
        (0, true)
    }

    fn pure_lit(&mut self, clause_index: usize) {
        for index in 0..self.clauses[clause_index].variables.len() {
            let variable_index = self.clauses[clause_index].variables[index];
            if self.variables[variable_index].value == 2 {
                if self.clauses[clause_index].signs[index] {
                    self.variables[variable_index].pos_occ_not_sat_qty -= 1;
                    if self.variables[variable_index].pos_occ_not_sat_qty == 0 {
                        self.queue.push((variable_index, false));
                    }
                } else {
                    self.variables[variable_index].neg_occ_not_sat_qty -= 1;
                    if self.variables[variable_index].neg_occ_not_sat_qty == 0 {
                        self.queue.push((variable_index, true));
                    }
                }
            }
        }
    }

    fn dlis(&self) -> (usize, bool) {
        let mut variable_index = 0;
        let mut max_occurrence = 0;
        let mut value = true;
        for index in 0..self.variables.len() {
            if self.variables[index].pos_occ_not_sat_qty > max_occurrence
                && self.variables[index].value == 2
            {
                variable_index = index;
                max_occurrence = self.variables[index].pos_occ_not_sat_qty;
                value = true;
            }
            if self.variables[index].neg_occ_not_sat_qty > max_occurrence
                && self.variables[index].value == 2
            {
                variable_index = index;
                max_occurrence = self.variables[index].neg_occ_not_sat_qty;
                value = false;
            }
        }
        (variable_index, value)
    }

    fn backtrack(&mut self) {
        let (mut variable_index, mut forced) = self.backtracking_stack.pop().unwrap();
        while forced {
            self.unset_value(variable_index);
            if self.backtracking_stack.len() == 0 {
                println!("UNSAT");
                std::process::exit(0);
            }
            let tup = self.backtracking_stack.pop().unwrap();
            variable_index = tup.0;
            forced = tup.1;
        }
        let switch_value = self.variables[variable_index].value == 0;
        self.unset_value(variable_index);
        self.set_value(variable_index, switch_value, true);
    }

    pub fn dpll(&mut self) -> &Vec<Variable> {
        while true {
            let mut free = 0;
            let mut sats = 0;
            for i in 0..self.clauses.len() {
                if self.clauses[i].sat_by < self.variables.len() {
                    sats += 1;
                }
            }
            for i in 0..self.backtracking_stack.len() {
                if !self.backtracking_stack[i].1 {
                    free += 1;
                }
            }
            //println!("{} {}",sats,free);
            //println!("{:?} {:?}",self.backtracking_stack,self.queue);
            while self.queue.len() > 0 {
                let tup = self.queue.pop().unwrap();
                let next_variable = tup.0;
                let next_value = tup.1;
                self.set_value(next_variable, next_value, true);
                while self.conflict {
                    //println!("conflict!!!!!");
                    self.queue.clear();
                    self.backtrack();
                    self.conflict = false;
                }
            }
            let mut all_set = true;
            for variable in &self.variables {
                if variable.value == 2 {
                    all_set = false
                }
            }
            if all_set {
                return &self.variables;
            }
            let (mut next_variable, mut next_value) = self.dlis();
            self.set_value(next_variable, next_value, false);
        }
        &self.variables
    }

    pub fn validate(&self) -> bool {
        // check if there is still free variables
        for i in 0..self.variables.len() {
            if self.variables[i].value == 2 {
                return false;
            }
        }
        // check if there is still not satisfied clauses
        for i in 0..self.clauses.len() {
            if self.clauses[i].sat_by == self.variables.len() {
                return false;
            }
        }
        // logical check if all clauses are really sat
        for i in 0..self.clauses.len() {
            let mut sat = false;
            for j in 0..self.clauses[i].variables.len() {
                if self.clauses[i].signs[j] as usize
                    == self.variables[self.clauses[i].variables[j]].value
                {
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
}

fn read_file(path: &str) -> String {
    use std::io::Read;
    let mut file = std::fs::File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}
