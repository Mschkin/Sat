use std::time::{Duration, Instant};

#[derive(Debug)]
struct Clause {
    variables: Vec<usize>,
    signs: Vec<bool>,
    sat_by: usize,
    free_variables_qty: usize,
    name:usize,
}

impl Clause{
    fn sat(&mut self,variable:usize,loose_clause_queue:&mut Vec<(usize,bool)>){
        if self.sat_by==FREE{
            self.sat_by=variable;
            for i in 0..self.variables.len(){
                loose_clause_queue.push((self.variable[i],self.signs[i]);
            }
        }
    }

    fn unsat(&mut self,variable:usize,get_clause_queue:&mut Vec<(usize,bool,usize)>){
        if self.sat_by==variable{
            self.sat_by=FREE;
            for i in 0..self.variables.len(){
                get_clause_queue.push((self.variable[i],self.signs[i],self.free_variables_qty));
            }
        }
    }

    fn grow(&mut self,get_neighbor_queue:&mut Vec<(usize,bool,usize)>){
        if self.sat_by==FREE{
            self.free_variables_qty+=1;
            for i in 0..self.variables.len(){
                get_neighbor_queue.push((self.variable[i],self.signs[i],self.free_variables_qty));
            }
        }
    }

    fn shrink(&mut self,variable:usize,loose_neighbor_queue:&mut Vec<(usize,bool,usize)>,unit_prop_queue:&mut Vec<usize>>){
        if self.sat_by==FREE{
            self.free_variables_qty-=1;
            if self.free_variables_qty==1{
                unit_prop_queue.push(&self.name);
            }
            else if self.free_variables_qty==0{
                conflict=true;
            }
            for i in 0..self.variables.len(){
                loose_neighbor_queue.push((self.variable[i],self.signs[i],self.free_variables_qty));
            }
        }
    }
}

#[derive(Debug)]
pub struct Variable {
    pub value: usize,
    name:usize,
    pos_occ: Vec<usize>,
    neg_occ: Vec<usize>,
    pos_occ_not_sat_qty: usize,
    neg_occ_not_sat_qty: usize,
    pos_occ_len: Vec<usize>,
    neg_occ_len: Vec<usize>,
}

impl Variable{
    fn set(&mut self,value:bool,forced:bool,&mut backtracking_stack:Vec<(usize, bool)>,  sat_queue:&mut Vec<(usize,usize)>,shrink_queue:&mut Vec<(usize,usize)>){
        if self.value!=2&&self.value!=value{
            if self.pos_occ_not_sat_qty>0||self.neg_occ_not_sat_qty>0{
                conflict=true;
            }
        }
        if self.value==2{
            self.value=value;
            backtracking_stack.push((self.name,forced));
            if value{
                for i in &self.pos_occ{
                    sat_queue.push((i,self.name));
                }
                for i in &self.neg_occ{
                    shrink_queue.push((i,self.name));
                }
            }
            else{
                for i in &self.neg_occ{
                    sat_queue.push((i,self.name));
                }
                for i in &self.pos_occ{
                    shrink_queue.push((i,self.name));
                }
            }
        }
    }

    fn unset(&mut self,unsat_queue:&mut Vec<(usize,usize)>,grow_queue:&mut Vec<usize>)){
        if self.value{

        
        for i in &self.pos_occ{
            unsat_queue.push((i,self.name));
        }
        for i in &self.neg_occ{
            grow_queue.push(i);
        }


        } else{
                    
        for i in &self.neg_occ{
            unsat_queue.push((i,self.name));
        }
        for i in &self.pos_occ{
            grow_queue.push(i);
        }
        }
        self.value=2;
    }

    fn get_neighbor(&mut self,occ:bool,free_variables_qty:usize){
        if self.value==2{
            if occ{
                if free_variables_qty>2{
                self.pos_occ_len[free_variables_qty-3]-=1;}
                if free_variables_qty>1{
                self.pos_occ_len[free_variables_qty-2]+=1;}
            } else{
                if free_variables_qty>2{
                    self.neg_occ_len[free_variables_qty-3]-=1;}
                    if free_variables_qty>1{
                    self.neg_occ_len[free_variables_qty-2]+=1;}
            }
        }
    }

    fn loose_neighbor(&mut self,occ:bool,free_variables_qty:usize){
        if self.value==2{
            if occ{
                if free_variables_qty>0{
                self.pos_occ_len[free_variables_qty-1]-=1;}
                if free_variables_qty>1{
                self.pos_occ_len[free_variables_qty-2]+=1;}
            } else{
                if free_variables_qty>0{
                    self.neg_occ_len[free_variables_qty-1]-=1;}
                    if free_variables_qty>1{
                    self.neg_occ_len[free_variables_qty-2]+=1;}
            }
        }
    }
    fn get_clause(&mut self,occ:bool,free_variables_qty:usize){
        if self.value==2{
            if occ{
                self.pos_occ_not_sat_qty+=1;
                if free_variables_qty>1{
                self.pos_occ_len[free_variables_qty-2]+=1;}
            }else{
                self.neg_occ_not_sat_qty+=1;
                if free_variables_qty>1{
                self.neg_occ_len[free_variables_qty-2]+=1;}
            }
        }
    }
    fn loose_clause(&mut self,occ:bool,free_variables_qty:usize,&mut set_queue:Vec<(usize,bool)>){
        if self.value==2{
            if occ{
                self.pos_occ_not_sat_qty-=1;
                if self.pos_occ_not_sat_qty==0{
                    set_queue.push((self.name,false));
                }
                if free_variables_qty>1{
                self.pos_occ_len[free_variables_qty-2]-=1;}
            }
            else{
                self.neg_occ_not_sat_qty-=1;
                if self.neg_occ_not_sat_qty==0 && self.pos_occ_not_sat_qty !=0{
                    set_queue.push((self.name,true));
                }
                if free_variables_qty>1{
                self.neg_occ_len[free_variables_qty-2]-=1;}
            }
        }}
}


#[derive(Debug)]
pub struct DPLL {
    clauses: Vec<Clause>,
    pub variables: Vec<Variable>,
    set_queue: Vec<(usize, bool)>,
    backtracking_stack: Vec<(usize, bool)>,
    conflict: bool,
    pub unsat: bool,
    heuristic: usize,
    pub solved: bool,
    sat_queue:Vec<(usize,usize)>,
    unsat_queue:Vec<usize>,
    grow_queue:Vec<usize>,
    shrink_queue:Vec<(usize,usize)>,
    get_neighbor_queue:Vec<usize>,
    loose_neighbor_queue:Vec<(usize,bool)>,
    get_clause_queue:Vec<usize>,
    loose_clause_queue:Vec<(usize,bool)>,
    unit_prop_queue:Vec<usize>
    start: Instant,
    pub duration: Duration,
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
            if line_elem.len() > 1 && line_elem[0] != "c"&& line_elem[0] != "0" {
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
                    for j in 0..line_elem.len() {
                        if line_elem[j]=="0"{
                            break;
                        }
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
        let start = Instant::now();
        DPLL {
            clauses: clauses,
            variables: variables,
            queue: queue,
            backtracking_stack: Vec::<(usize, bool)>::new(),
            conflict: false,
            unsat: false,
            heuristic: heuristic,
            solved: false,
            start: start,
            duration: start.elapsed(),
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

    fn unit_prop(&mut self, clause_index: usize) {
        if self.clauses[clause_index].sat_by == self.variables.len() {
            self.clauses[clause_index].free_variables_qty -= 1;
            if self.clauses[clause_index].free_variables_qty == 0 {
                self.conflict = true;
            } else if self.clauses[clause_index].free_variables_qty == 1 {
                self.set_queue.push(self.get_unit_prop(clause_index));
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
                        self.set_queue.push((variable_index, false));
                    }
                } else {
                    self.variables[variable_index].neg_occ_not_sat_qty -= 1;
                    if self.variables[variable_index].neg_occ_not_sat_qty == 0 &&
                        self.variables[variable_index].pos_occ_not_sat_qty != 0
                    {
                        self.set_queue.push((variable_index, true));
                    }
                }
            }
        }
    }

    fn update_occ_len(&mut self) {
        for i in 0..self.clauses.len() {
            if self.clauses[i].sat_by == self.variables.len() {
                for j in 0..self.clauses[i].variables.len() {
                    if self.clauses[i].free_variables_qty >= 2 {
                        if self.clauses[i].signs[j] {
                            while self.variables[self.clauses[i].variables[j]]
                                .pos_occ_len
                                .len() <=
                                self.clauses[i].variables.len() - 2
                            {
                                self.variables[self.clauses[i].variables[j]]
                                    .pos_occ_len
                                    .push(0);
                            }
                            self.variables[self.clauses[i].variables[j]].pos_occ_len
                                [self.clauses[i].free_variables_qty - 2] += 1;
                        } else {
                            while self.variables[self.clauses[i].variables[j]]
                                .neg_occ_len
                                .len() <=
                                self.clauses[i].variables.len() - 2
                            {
                                self.variables[self.clauses[i].variables[j]]
                                    .neg_occ_len
                                    .push(0);
                            }
                            self.variables[self.clauses[i].variables[j]].neg_occ_len
                                [self.clauses[i].free_variables_qty - 2] += 1;
                        }
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
        let constant = 8;
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
                    momscore = (self.variables[i].pos_occ_len[k] +
                                    self.variables[i].neg_occ_len[k]) *
                        constant +
                        self.variables[i].pos_occ_len[k] * self.variables[i].neg_occ_len[k];
                    variable = i;
                    value = self.variables[i].pos_occ_len[k] > self.variables[i].neg_occ_len[k];
                } else if k == shortest_len &&
                           momscore <
                               (self.variables[i].pos_occ_len[k] +
                                    self.variables[i].neg_occ_len[k]) *
                                   constant +
                                   self.variables[i].pos_occ_len[k] *
                                       self.variables[i].neg_occ_len[k]
                {
                    momscore = (self.variables[i].pos_occ_len[k] +
                                    self.variables[i].neg_occ_len[k]) *
                        constant +
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

    fn jw(&mut self) -> (usize, bool) {
        let mut jwscore = 0.;
        let mut variable = self.variables.len();
        let mut value = false;
        for i in 0..self.variables.len() {
            if self.variables[i].value == 2 {
                let mut new_jw = 0.;
                let mut weight = 1. / 4.;
                for j in 0..self.variables[i].pos_occ_len.len() {
                    new_jw += weight * self.variables[i].pos_occ_len[j] as f64;
                    weight = weight / 2.;
                }
                if new_jw > jwscore {
                    jwscore = new_jw;
                    variable = i;
                    value = true;
                }
                let mut new_jw = 0.;
                let mut weight = 1. / 4.;
                for j in 0..self.variables[i].neg_occ_len.len() {
                    new_jw += weight * self.variables[i].neg_occ_len[j] as f64;
                    weight = weight / 2.;
                }
                if new_jw > jwscore {
                    jwscore = new_jw;
                    variable = i;
                    value = false;
                }
            }
        }
        if variable == self.variables.len() {
            self.solved = true;
            return (variable, true);
        }
        (variable, value)
    }

    fn occurrence_count(&self, variable: usize, occ: usize, pos: bool) -> usize {
        if pos && self.variables[variable].pos_occ_len.len() <= occ {
            return 0;
        } else if !pos && self.variables[variable].neg_occ_len.len() <= occ {
            return 0;
        }
        if pos {
            return self.variables[variable].pos_occ_len[occ];
        } else {
            return self.variables[variable].neg_occ_len[occ];
        }
    }

    fn boehm(&mut self) -> (usize, bool) {
        let mut hs = vec![0; self.variables.len()];
        let mut variable = self.variables.len();
        let mut value = false;
        let mut maxlen = 0;
        let alpha = 1;
        let beta = 2;
        for i in 0..self.variables.len() {
            if self.variables[i].value == 2 {
                if maxlen <
                    std::cmp::max(
                        self.variables[i].pos_occ_len.len(),
                        self.variables[i].neg_occ_len.len(),
                    )
                {
                    maxlen = std::cmp::max(
                        self.variables[i].pos_occ_len.len(),
                        self.variables[i].neg_occ_len.len(),
                    );
                }
                for mut j in 0..hs.len() {
                    let hnew = alpha *
                        std::cmp::max(
                            self.occurrence_count(i, j, true),
                            self.occurrence_count(i, j, false),
                        ) +
                        beta *
                            std::cmp::min(
                                self.occurrence_count(i, j, true),
                                self.occurrence_count(i, j, false),
                            );
                    if hnew > hs[j] {
                        variable = i;
                        value = self.variables[i].pos_occ_not_sat_qty >
                            self.variables[i].neg_occ_not_sat_qty;
                        hs[j] = hnew;
                        while j < maxlen {
                            j += 1;
                            hs[j] = alpha *
                                std::cmp::max(
                                    self.occurrence_count(i, j, true),
                                    self.occurrence_count(i, j, false),
                                ) +
                                beta *
                                    std::cmp::min(
                                        self.occurrence_count(i, j, true),
                                        self.occurrence_count(i, j, false),
                                    );
                        }
                        break;
                    } else if hnew < hs[j] {
                        break;
                    }
                }
            }
        }
        if variable == self.variables.len() {
            self.solved = true;
            return (variable, true);
        }
        (variable, value)
    }

    fn backtrack(&mut self) {
        let (mut variable_index, mut forced) = self.backtracking_stack.pop().unwrap();
        while forced {
            self.variables[variable_index].unset()
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
        self.set_queue.push((variable_index, switch_value));
    }

    pub fn dpll(&mut self) {
        while !self.unsat && !self.solved {
            while self.set_queue.len() > 0 {
                let tup = self.set_queue.pop().unwrap();
                let next_variable = tup.0;
                let next_value = tup.1;
                self.variables[next_variable].set(next_value, true,&self.backtracking_stack,&self.sat_queue,&self.shrink_queue);}
                while self.sat_queue.len()>0&&!self.conflict{
                    let tup = self.sat_queue.pop().unwrap();
                    let next_clause = tup.0;
                    let sat_by = tup.1;
                    self.clauses[next_clause].sat(sat_by,&self.loose_clause_queue);
                }
                while self.unsat_queue.len()>0&&!self.conflict{
                    let tup = self.unsat_queue.pop().unwrap();
                    let next_clause = tup.0;
                    let sat_by = tup.1;
                    self.clauses[next_clause].unsat(sat_by,&self.get_clause_queue);
                }
                while self.grow_queue.len()>0&&!self.conflict{
                    let tup = self.grow_queue.pop().unwrap();
                    let next_clause = tup.0;
                    let sat_by = tup.1;
                    self.clauses[next_clause].unsat(&self.get_neighbor_queue);
                }
                while self.shrink_queue.len()>0&&!self.conflict{
                    let tup = self.shrink_queue.pop().unwrap();
                    let next_clause = tup.0;
                    let next_variable = tup.1;
                    self.clauses[next_clause].unsat(next_variable,&self.loose_neighbor_queue,&self.unit_prop_queue);
                }
                while self.unit_prop_queue.len()>0&&!self.conflict{
                    let next_clause = self.unit_prop_queue.pop().unwrap();
                    for i in 0..self.clauses[next_clause].variables.len(){
                        if self.variables[self.clauses[next_clause].variables[i]].value==2{
                            self.set_queue((self.clauses[next_clause].variables[i],self.clauses[next_clause].signs[i]));
                            break;
                        }
                    }
                }
                while self.get_neighbor_queue.len()>0&&!self.conflict{
                    let tup=self.get_neighbor_queue.pop().unwrap();
                    let next_variable=tup.0
                    let next_occ=tup.1
                    let next_free_v_qty=tup.2
                    self.variables[next_variable].get_neighbor(next_occ,next_free_v_qty);
                }
                while self.loose_neighbor_queue.len()>0&&!self.conflict{
                    let tup=self.loose_neighbor_queue.pop().unwrap();
                    let next_variable=tup.0
                    let next_occ=tup.1
                    let next_free_v_qty=tup.2
                    self.variables[next_variable].loose_neighbor(next_occ,next_free_v_qty);
                }
                while self.get_clause_queue.len()>0&&!self.conflict{
                    let tup=self.get_clause_queue.pop().unwrap();
                    let next_variable=tup.0
                    let next_occ=tup.1
                    let next_free_v_qty=tup.2
                    self.variables[next_variable].get_clause(next_occ,next_free_v_qty);
                }
                while self.loose_clause_queue.len()>0&&!self.conflict{
                    let tup=self.loose_clause_queue.pop().unwrap();
                    let next_variable=tup.0
                    let next_occ=tup.1
                    let next_free_v_qty=tup.2
                    self.variables[next_variable].loose_clause(next_occ,next_free_v_qty,self.set_queue);
                }
                if self.conflict {
                    //println!("conflict!!!!!");
                    self.set_queue.clear();
                    self.sat_queue.clear();
                    self.unsat_queue.clear();
                    self.grow_queue.clear();
                    self.shrink_queue.clear();
                    self.unit_prop_queue.clear();
                    self.get_neighbor_queue.clear();
                    self.loose_neighbor_queue.clear();
                    self.get_clause_queue.clear();
                    self.loose_clause_queue.clear();
                    self.backtrack();
                    self.conflict = false;
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
            } else if self.heuristic == 2 {
                next_choice = self.jw();
            } else if self.heuristic == 3 {
                next_choice = self.moms();
            } else {
                next_choice = self.boehm();
            }
            if !self.solved {
                self.variables[next_choice.0].set(next_choice.1, false,&self.backtracking_stack,&self.sat_queue,&self.shrink_queue);
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
