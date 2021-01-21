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
    pos_occ_len: Vec<usize>,
    neg_occ_len: Vec<usize>,
}

#[derive(Debug)]
pub struct DPLL {
    clauses: Vec<Clause>,
    pub variables: Vec<Variable>,
    queue: Vec<(usize, bool)>,
    backtracking_stack: Vec<(usize, bool)>,
    conflict: bool,
    pub unsat: bool,
    heuristic: usize,
    solved: bool,
}

impl DPLL {
    pub fn new(path: &str, heuristic: usize) -> Self {
        let content: String = read_file(path);
        let content_clone = content.clone();
        let input: Vec<&str> = (&content_clone).split("\n").collect();
        let mut clauses = Vec::<Clause>::new();
        let mut variables = Vec::<Variable>::new();
        let mut queue = Vec::<(usize, bool)>::new();
        let mut variables_qty: usize = 0;
        for line_number in 0..input.len() {
            let line_elem: Vec<&str> = input[line_number].split_whitespace().collect();
            if line_elem.len() > 1 && line_elem[0] != "c" {
                if line_elem[0] == "p" {
                    variables_qty = line_elem[2].parse::<usize>().unwrap();
                    for _i in 0..variables_qty {
                        let variable = Variable {
                            value: 2,
                            pos_occ: Vec::<usize>::new(),
                            neg_occ: Vec::<usize>::new(),
                            pos_occ_not_sat_qty: 0,
                            neg_occ_not_sat_qty: 0,
                            pos_occ_len: Vec::<usize>::new(),
                            neg_occ_len: Vec::<usize>::new(),
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
                    let mut push = true;
                    for j in 0..line_elem.len() - 1 {
                        let lit = line_elem[j].parse::<i32>().unwrap();
                        if !clause.variables.contains(&((lit.abs() - 1) as usize)) {
                            clause.variables.push((lit.abs() - 1) as usize);
                            clause.signs.push(lit > 0);
                            clause.free_variables_qty += 1;
                            if lit > 0 {
                                variables[(lit.abs() - 1) as usize].pos_occ.push(
                                    clauses.len(),
                                );
                                variables[(lit.abs() - 1) as usize].pos_occ_not_sat_qty += 1;
                            } else {
                                variables[(lit.abs() - 1) as usize].neg_occ.push(
                                    clauses.len(),
                                );
                                variables[(lit.abs() - 1) as usize].neg_occ_not_sat_qty += 1;
                            }
                        } else {
                            // ignore clauses containing both x and -x
                            if clause.signs[clause
                                                .variables
                                                .iter()
                                                .position(|&x| x == (lit.abs() - 1) as usize)
                                                .unwrap()] !=
                                (lit > 0)
                            {
                                push = false;
                                for i in 0..clause.variables.len() {
                                    if clause.signs[i] {
                                        variables[clause.variables[i]].pos_occ.pop();
                                        variables[clause.variables[i]].pos_occ_not_sat_qty -= 1;
                                    } else {
                                        variables[clause.variables[i]].neg_occ.pop();
                                        variables[clause.variables[i]].neg_occ_not_sat_qty -= 1;
                                    }
                                }
                                break;
                            }
                        }
                    }
                    if push {
                        // unit prop
                        if clause.variables.len() == 1 {
                            queue.push((clause.variables[0], clause.signs[0]));
                        } else if heuristic > 1 {
                            for i in 0..clause.variables.len() {
                                if clause.signs[i] {
                                    while variables[clause.variables[i]].pos_occ_len.len() <=
                                        clause.variables.len() - 2
                                    {
                                        variables[clause.variables[i]].pos_occ_len.push(0);
                                    }
                                    variables[clause.variables[i]].pos_occ_len[clause
                                                                                   .variables
                                                                                   .len() -
                                                                                   2] += 1;
                                } else {
                                    while variables[clause.variables[i]].neg_occ_len.len() <=
                                        clause.variables.len() - 2
                                    {
                                        variables[clause.variables[i]].neg_occ_len.push(0);
                                    }
                                    variables[clause.variables[i]].neg_occ_len[clause
                                                                                   .variables
                                                                                   .len() -
                                                                                   2] += 1;
                                }

                            }
                        }
                        clauses.push(clause);

                    }
                }
            }
        }

        // pure lit
        for i in 0..variables_qty {
            if variables[i].pos_occ_not_sat_qty == 0 {
                queue.push((i, false));
            }
            if variables[i].neg_occ_not_sat_qty == 0 && variables[i].pos_occ_not_sat_qty != 0 {
                queue.push((i, true));
            }
        }
        DPLL {
            clauses: clauses,
            variables: variables,
            queue: queue,
            backtracking_stack: Vec::<(usize, bool)>::new(),
            conflict: false,
            unsat: false,
            heuristic: heuristic,
            solved: false,
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
                self.unit_prop(self.variables[variable_index].neg_occ[j]);
            }
            self.backtracking_stack.push((variable_index, forced))
        } else if !value && self.variables[variable_index].value == 2 {
            self.variables[variable_index].value = value as usize;
            for i in 0..self.variables[variable_index].pos_occ.len() {
                self.unit_prop(self.variables[variable_index].pos_occ[i]);
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
            if self.variables[variable_index].pos_occ_not_sat_qty > 0 ||
                self.variables[variable_index].neg_occ_not_sat_qty > 0
            {
                self.conflict = true
            }
        }
    }

    fn unset_value(&mut self, variable_index: usize) {
        self.variables[variable_index].value = 2;
        for i in 0..self.variables[variable_index].pos_occ.len() {
            let clause_index = self.variables[variable_index].pos_occ[i];
            self.handle_unset_value(clause_index, variable_index);
        }
        for i in 0..self.variables[variable_index].neg_occ.len() {
            let clause_index = self.variables[variable_index].neg_occ[i];
            self.handle_unset_value(clause_index, variable_index);
        }
    }

    fn handle_unset_value(&mut self, clause_index: usize, variable_index: usize) {
        if self.clauses[clause_index].sat_by == variable_index {
            self.clauses[clause_index].sat_by = self.variables.len();
            for j in 0..self.clauses[clause_index].variables.len() {
                if self.variables[self.clauses[clause_index].variables[j]].value == 2 {
                    if self.clauses[clause_index].signs[j] {
                        self.variables[self.clauses[clause_index].variables[j]]
                            .pos_occ_not_sat_qty += 1;
                        if self.heuristic > 1 &&
                            self.clauses[clause_index].free_variables_qty >= 2
                        {
                            self.variables[self.clauses[clause_index].variables[j]].pos_occ_len
                                [self.clauses[clause_index].free_variables_qty - 2] += 1;
                        }
                    } else {
                        self.variables[self.clauses[clause_index].variables[j]]
                            .neg_occ_not_sat_qty += 1;
                        if self.heuristic > 1 &&
                            self.clauses[clause_index].free_variables_qty >= 2
                        {
                            self.variables[self.clauses[clause_index].variables[j]].neg_occ_len
                                [self.clauses[clause_index].free_variables_qty - 2] += 1;
                        }
                    }
                }
            }
        } else if self.clauses[clause_index].sat_by == self.variables.len() {
            self.clauses[clause_index].free_variables_qty += 1;
            if self.heuristic > 1 && self.clauses[clause_index].free_variables_qty >= 2 {
                for j in 0..self.clauses[clause_index].variables.len() {
                    if self.variables[self.clauses[clause_index].variables[j]].value == 2 {
                        if self.clauses[clause_index].signs[j] {
                            if self.clauses[clause_index].free_variables_qty >= 3 {
                                println!(
                                    "{} {} {:?}",self.clauses[clause_index].free_variables_qty,
                                    self.clauses[clause_index].variables[j]+1,
                                    self.variables[self.clauses[clause_index].variables[j]]
                                        .pos_occ_len
                                );
                                println!("{:?}", self.clauses[clause_index]);
                                self.variables[self.clauses[clause_index].variables[j]]
                                    .pos_occ_len
                                    [self.clauses[clause_index].free_variables_qty - 3] -= 1;
                            }
                            self.variables[self.clauses[clause_index].variables[j]].pos_occ_len
                                [self.clauses[clause_index].free_variables_qty - 2] += 1;
                        } else {
                            if self.clauses[clause_index].free_variables_qty >= 3 {
                                self.variables[self.clauses[clause_index].variables[j]]
                                    .neg_occ_len
                                    [self.clauses[clause_index].free_variables_qty - 3] -= 1;
                            }
                            self.variables[self.clauses[clause_index].variables[j]].neg_occ_len
                                [self.clauses[clause_index].free_variables_qty - 2] += 1;
                        }
                    }
                }
            }
        }

    }

    fn unit_prop(&mut self, clause_index: usize) {
        if self.clauses[clause_index].sat_by == self.variables.len() {
            self.clauses[clause_index].free_variables_qty -= 1;
            if self.clauses[clause_index].free_variables_qty > 1 && self.heuristic > 1 {
                for i in 0..self.clauses[clause_index].variables.len() {
                    if self.variables[self.clauses[clause_index].variables[i]].value == 2 {
                        if self.clauses[clause_index].signs[i] {
                            self.variables[self.clauses[clause_index].variables[i]].pos_occ_len
                                [self.clauses[clause_index].free_variables_qty - 2] += 1;
                            self.variables[self.clauses[clause_index].variables[i]].pos_occ_len
                                [self.clauses[clause_index].free_variables_qty - 1] -= 1;
                        } else {
                            self.variables[self.clauses[clause_index].variables[i]].neg_occ_len
                                [self.clauses[clause_index].free_variables_qty - 2] += 1;
                            self.variables[self.clauses[clause_index].variables[i]].neg_occ_len
                                [self.clauses[clause_index].free_variables_qty - 1] -= 1;
                        }
                    }
                }
            } else if self.clauses[clause_index].free_variables_qty == 0 {
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
                    if self.heuristic > 1 {
                        self.variables[variable_index].pos_occ_len[self.clauses[clause_index]
                                                                       .free_variables_qty -
                                                                       2] -= 1;
                    }
                    if self.variables[variable_index].pos_occ_not_sat_qty == 0 {
                        self.queue.push((variable_index, false));
                    }
                } else {
                    self.variables[variable_index].neg_occ_not_sat_qty -= 1;
                    if self.heuristic > 1 {
                        self.variables[variable_index].neg_occ_len[self.clauses[clause_index]
                                                                       .free_variables_qty -
                                                                       2] -= 1;
                    }
                    if self.variables[variable_index].neg_occ_not_sat_qty == 0 &&
                        self.variables[variable_index].pos_occ_not_sat_qty != 0
                    {
                        self.queue.push((variable_index, true));
                    }
                }
            }
        }
    }

    fn dlis(&mut self) -> (usize, bool) {
        let mut variable_index = self.variables.len();
        let mut max_occurrence = 0;
        let mut value = false;
        for index in 0..self.variables.len() {
            if self.variables[index].pos_occ_not_sat_qty > max_occurrence &&
                self.variables[index].value == 2
            {
                variable_index = index;
                max_occurrence = self.variables[index].pos_occ_not_sat_qty;
                value = true;
            }
            if self.variables[index].neg_occ_not_sat_qty > max_occurrence &&
                self.variables[index].value == 2
            {
                variable_index = index;
                max_occurrence = self.variables[index].neg_occ_not_sat_qty;
                value = false;
            }
        }
        if variable_index == self.variables.len() {
            self.solved = true;
            return (variable_index, true);
        }
        (variable_index, value)
    }

    fn dlcs(&mut self) -> (usize, bool) {
        let mut variable_index = self.variables.len();
        let mut max_occurrence = 0;
        let mut value = false;
        for index in 0..self.variables.len() {
            if self.variables[index].pos_occ_not_sat_qty +
                self.variables[index].neg_occ_not_sat_qty > max_occurrence &&
                self.variables[index].value == 2
            {
                variable_index = index;
                max_occurrence = self.variables[index].pos_occ_not_sat_qty +
                    self.variables[index].neg_occ_not_sat_qty;
                if self.variables[index].pos_occ_not_sat_qty >=
                    self.variables[index].neg_occ_not_sat_qty
                {
                    value = true;
                } else {
                    value = false;
                }
            }
        }
        if variable_index == self.variables.len() {
            self.solved = true;
            return (variable_index, true);
        }
        (variable_index, value)
    }

    fn moms(&mut self) -> (usize, bool) {
        let mut momscore = 0;
        let mut shortest_len = self.variables.len();
        let mut variable = self.variables.len();
        let mut value = false;
        let mut k: usize;
        for i in 0..self.variables.len() {
            if self.variables[i].value == 2 {
                k = 0;
                while self.variables[i].pos_occ_len[k] == 0 &&
                    self.variables[i].neg_occ_len[k] == 0
                {
                    k += 1
                }
                if k < shortest_len {
                    shortest_len = k;
                    momscore =
                        (self.variables[i].pos_occ_len[k] + self.variables[i].neg_occ_len[k]) * 8 +
                            self.variables[i].pos_occ_len[k] * self.variables[i].neg_occ_len[k];
                    variable = i;
                    value = self.variables[i].pos_occ_len[k] > self.variables[i].neg_occ_len[k];

                } else if k == shortest_len &&
                           momscore <
                               (self.variables[i].pos_occ_len[k] +
                                    self.variables[i].neg_occ_len[k]) *
                                   8 +
                                   self.variables[i].pos_occ_len[k] *
                                       self.variables[i].neg_occ_len[k]
                {
                    momscore =
                        (self.variables[i].pos_occ_len[k] + self.variables[i].neg_occ_len[k]) * 8 +
                            self.variables[i].pos_occ_len[k] * self.variables[i].neg_occ_len[k];
                    variable = i;
                    value = self.variables[i].pos_occ_len[k] > self.variables[i].neg_occ_len[k];
                }
            }
        }
        if variable == self.variables.len() {
            self.solved = true;
            return (variable, true);
        }
        (variable, value)
    }

    fn boehm(&mut self)

    fn backtrack(&mut self) {
        let (mut variable_index, mut forced) = self.backtracking_stack.pop().unwrap();
        while forced {
            self.unset_value(variable_index);
            if self.backtracking_stack.len() == 0 {
                self.unsat = true;
                return;
            }
            let tup = self.backtracking_stack.pop().unwrap();
            variable_index = tup.0;
            forced = tup.1;
        }
        let switch_value = self.variables[variable_index].value == 0;
        self.unset_value(variable_index);
        self.queue.push((variable_index, switch_value));
    }

    pub fn dpll(&mut self) {
        while !self.unsat && !self.solved {
            while self.queue.len() > 0 {
                let tup = self.queue.pop().unwrap();
                let next_variable = tup.0;
                let next_value = tup.1;
                self.set_value(next_variable, next_value, true);
                if self.conflict {
                    //println!("conflict!!!!!");
                    self.queue.clear();
                    self.backtrack();
                    self.conflict = false;
                }
            }
            if self.heuristic > 1 {
                for i in 0..self.variables.len() {
                    if self.variables[i].pos_occ_not_sat_qty !=
                        self.variables[i].pos_occ_len.iter().fold(0, |a, &b| a + b)
                    {
                        println!(
                            "ERROR, {:?} {}",
                            self.variables[i].pos_occ_len,
                            self.variables[i].pos_occ_not_sat_qty
                        )
                    }
                    if self.variables[i].neg_occ_not_sat_qty !=
                        self.variables[i].neg_occ_len.iter().fold(0, |a, &b| a + b)
                    {
                        println!(
                            "ERROR, {:?} {}",
                            self.variables[i].neg_occ_len,
                            self.variables[i].neg_occ_not_sat_qty
                        )
                    }
                }
            }
            let next_choice: (usize, bool);
            if self.heuristic == 0 {
                next_choice = self.dlis();
            } else if self.heuristic == 1 {
                next_choice = self.dlcs();
            } else {
                next_choice = self.moms();
            }
            if !self.solved {
                self.set_value(next_choice.0, next_choice.1, false);
            }
        }
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
                if self.clauses[i].signs[j] as usize ==
                    self.variables[self.clauses[i].variables[j]].value
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
