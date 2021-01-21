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
        let mut jwscore = 0;
        let mut variable = self.variables.len();
        let mut value = false;
        for i in 0..self.variables.len() {
            if self.variables[i].value == 2 {
                let mut new_jw = 0;
                let mut weight = 1 / 4;
                for i in 0..self.variables[i].pos_occ_len.len() {
                    new_jw += weight * self.variables[i].pos_occ_len[i];
                    weight = weight / 2;
                }
                if new_jw > jwscore {
                    jwscore = new_jw;
                    variable = i;
                    value = true;
                }
                let mut new_jw = 0;
                let mut weight = 1 / 4;
                for i in 0..self.variables[i].neg_occ_len.len() {
                    new_jw += weight * self.variables[i].neg_occ_len[i];
                    weight = weight / 2;
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
        if pos && self.variables[variable].pos_occ_len.len() >= occ {
            return 0;
        } else if !pos && self.variables[variable].neg_occ_len.len() >= occ {
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
                        self.variables[variable].pos_occ_len.len(),
                        self.variables[variable].neg_occ_len.len(),
                    )
                {
                    maxlen = std::cmp::max(
                        self.variables[variable].pos_occ_len.len(),
                        self.variables[variable].neg_occ_len.len(),
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
                        value = self.variables[variable].pos_occ_not_sat_qty >
                            self.variables[variable].neg_occ_not_sat_qty;
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
        //ploter();
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
// use plotters::prelude::*;

// fn ploter() {
//     let root_area = BitMapBackend::new("test.png", (600, 400))
//     .into_drawing_area();
//     root_area.fill(&WHITE).unwrap();

//     let mut ctx = ChartBuilder::on(&root_area)
//         .set_label_area_size(LabelAreaPosition::Left, 40)
//         .set_label_area_size(LabelAreaPosition::Bottom, 40)
//         .caption("Scatter Demo", ("sans-serif", 40))
//         .build_cartesian_2d(-10..50, -10..50)
//         .unwrap();

//     ctx.configure_mesh().draw().unwrap();

//     ctx.draw_series(
//         DATA1.iter().map(|point| TriangleMarker::new(*point, 5, &BLUE)),
//     )
//     .unwrap();

//     ctx.draw_series(DATA2.iter().map(|point| Circle::new(*point, 5, &RED)))
//         .unwrap();
// }
// const DATA1: [(i32, i32); 30] = [
//     (-3, 1),
//     (-2, 3),
//     (4, 2),
//     (3, 0),
//     (6, -5),
//     (3, 11),
//     (6, 0),
//     (2, 14),
//     (3, 9),
//     (14, 7),
//     (8, 11),
//     (10, 16),
//     (7, 15),
//     (13, 8),
//     (17, 14),
//     (13, 17),
//     (19, 11),
//     (18, 8),
//     (15, 8),
//     (23, 23),
//     (15, 20),
//     (22, 23),
//     (22, 21),
//     (21, 30),
//     (19, 28),
//     (22, 23),
//     (30, 23),
//     (26, 35),
//     (33, 19),
//     (26, 19),
// ];
// const DATA2: [(i32, i32); 30] = [
//     (1, 22),
//     (0, 22),
//     (1, 20),
//     (2, 24),
//     (4, 26),
//     (6, 24),
//     (5, 27),
//     (6, 27),
//     (7, 27),
//     (8, 30),
//     (10, 30),
//     (10, 33),
//     (12, 34),
//     (13, 31),
//     (15, 35),
//     (14, 33),
//     (17, 36),
//     (16, 35),
//     (17, 39),
//     (19, 38),
//     (21, 38),
//     (22, 39),
//     (23, 43),
//     (24, 44),
//     (24, 46),
//     (26, 47),
//     (27, 48),
//     (26, 49),
//     (28, 47),
//     (28, 50),
// ];
