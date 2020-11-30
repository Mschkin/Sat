use std::collections::HashMap;

#[derive(Debug)]
struct Tent {
    position: (usize, usize),
    number: usize,
}

#[derive(Debug)]
struct Tree {
    position: (usize, usize),
    tents: Vec<Tent>,
}

#[derive(Debug)]
struct Game {
    trees: Vec<Tree>,
    max_column: usize,
    max_row: usize,
    tents_in_rows: Vec<usize>,
    tents_in_columns: Vec<usize>,
    tents_map: HashMap<(usize, usize), Vec<usize>>,
    variables_qty: usize,
    content: String,
}

#[derive(Debug)]
pub struct SatMaker {
    clauses: String,
    game: Game,
    clauses_qty: usize,
}

impl Game {
    fn new(path: &str) -> Self {
        let content: String = read_file(path);
        let content_clone = content.clone();
        let input: Vec<&str> = (&content_clone).split_whitespace().collect();
        let mut this = Self {
            trees: Vec::<Tree>::new(),
            max_column: input[1].parse::<usize>().unwrap(),
            max_row: input[0].parse::<usize>().unwrap(),
            tents_in_rows: Vec::<usize>::new(),
            tents_in_columns: Vec::<usize>::new(),
            tents_map: HashMap::new(),
            variables_qty: 1,
            content: content,
        };

        let mut row: usize;
        let mut column: usize;
        let mut index = 0;
        let end = (this.max_column + 1) * this.max_row + 2;
        for i in &input[2..end] {
            row = index / this.max_column;
            column = index % this.max_column;
            if *i == "T" || *i == "." {
                if *i == "T" {
                    let new_tree = Tree {
                        position: (row, column),
                        tents: Vec::<Tent>::new(),
                    };
                    this.trees.push(new_tree);
                }
                index += 1;
            } else {
                this.tents_in_rows.push(i.parse::<usize>().unwrap());
            }
        }
        for j in &input[end..] {
            this.tents_in_columns.push(j.parse::<usize>().unwrap());
        }

        for tree_number in 0..this.trees.len() {
            this.trees[tree_number].tents =
                this.get_tents(this.trees[tree_number].position, this.variables_qty);
            this.variables_qty += this.trees[tree_number].tents.len();
        }

        for i in 0..this.max_row {
            for j in 0..this.max_column {
                this.tents_map.insert((i, j), Vec::<usize>::new());
            }
        }
        for tree in &this.trees {
            for tent in &tree.tents {
                this.tents_map
                    .get_mut(&tent.position)
                    .unwrap()
                    .push(tent.number);
            }
        }
        this
    }

    fn get_tents(&self, position: (usize, usize), mut next_number: usize) -> Vec<Tent> {
        let mut new_tents = Vec::<Tent>::new();
        for pos in self.get_neighbors(position) {
            if self.is_free(pos) {
                let new_tent = Tent {
                    position: pos,
                    number: next_number,
                };
                new_tents.push(new_tent);
                next_number += 1;
            }
        }
        new_tents
    }
    fn is_free(&self, position: (usize, usize)) -> bool {
        let mut free = true;
        for i in &self.trees {
            if i.position == position {
                free = false;
            }
        }
        free
    }
    fn get_neighbors(&self, position: (usize, usize)) -> Vec<(usize, usize)> {
        let mut neighbors = Vec::<(usize, usize)>::new();
        if position.0 > 0 {
            neighbors.push((position.0 - 1, position.1));
        }
        if position.1 > 0 {
            neighbors.push((position.0, position.1 - 1));
        }
        if position.1 + 1 < self.max_column {
            neighbors.push((position.0, position.1 + 1));
        }
        if position.0 + 1 < self.max_row {
            neighbors.push((position.0 + 1, position.1));
        }
        neighbors
    }
}

impl SatMaker {
    pub fn new(path: &str) -> Self {
        let mut this = Self {
            clauses: String::new(),
            game: Game::new(path),
            clauses_qty: 0,
        };
        &this.none_adjacent();
        &this.ensure_tent_qty();
        this.clauses = format!(
            "p cnf {} {}\n{}",
            this.game.variables_qty, this.clauses_qty, this.clauses
        );
        this
    }

    fn n_choose_k(n: usize, mut k: usize) -> Vec<Vec<usize>> {
        let mut res_old = vec![Vec::<usize>::new()];
        let mut res_new = Vec::<Vec<usize>>::new();
        while k > 0 {
            k -= 1;
            for group in &res_old {
                let begin: usize;
                if group.len() > 0 {
                    begin = group[group.len() - 1] + 1;
                } else {
                    begin = 0;
                }
                for i in begin..n - k {
                    let mut clone = group.clone();
                    clone.push(i);
                    res_new.push(clone);
                }
            }
            res_old = res_new;
            res_new = Vec::<Vec<usize>>::new();
        }
        res_old
    }

    fn exactly_n(&mut self, n: usize, tent_numbers: Vec<usize>) {
        let mut combinations = Self::n_choose_k(tent_numbers.len(), n + 1);
        for combi in combinations {
            for i in combi {
                self.clauses.push_str(&format!("-{} ", tent_numbers[i]));
            }
            self.clauses.push_str("0\n");
            self.clauses_qty += 1;
        }
        combinations = Self::n_choose_k(tent_numbers.len(), tent_numbers.len() - n + 1);
        for combi in combinations {
            for i in combi {
                self.clauses.push_str(&format!("{} ", tent_numbers[i]));
            }
            self.clauses.push_str("0\n");
            self.clauses_qty += 1;
        }
    }

    fn none_adjacent(&mut self) {
        for row in 0..self.game.max_row {
            for column in 0..self.game.max_column {
                let tent_numbers = &self.game.tents_map[&(row, column)];
                for tent_number in tent_numbers {
                    for same_position in tent_numbers {
                        if same_position > tent_number {
                            self.clauses
                                .push_str(&format!("-{} -{} 0\n", tent_number, same_position));
                            self.clauses_qty += 1;
                        }
                    }
                    if row + 1 < self.game.max_row {
                        for neighbor in &self.game.tents_map[&(row + 1, column)] {
                            self.clauses
                                .push_str(&format!("-{} -{} 0\n", tent_number, neighbor));
                            self.clauses_qty += 1;
                        }
                    }
                    if column + 1 < self.game.max_column {
                        for neighbor in &self.game.tents_map[&(row, column + 1)] {
                            self.clauses
                                .push_str(&format!("-{} -{} 0\n", tent_number, neighbor));
                            self.clauses_qty += 1;
                        }
                    }
                    if row + 1 < self.game.max_row && column + 1 < self.game.max_column {
                        for neighbor in &self.game.tents_map[&(row + 1, column + 1)] {
                            self.clauses
                                .push_str(&format!("-{} -{} 0\n", tent_number, neighbor));
                            self.clauses_qty += 1;
                        }
                    }
                    if row > 0 && column + 1 < self.game.max_column {
                        for neighbor in &self.game.tents_map[&(row - 1, column + 1)] {
                            self.clauses
                                .push_str(&format!("-{} -{} 0\n", tent_number, neighbor));
                            self.clauses_qty += 1;
                        }
                    }
                }
            }
        }
    }

    fn ensure_tent_qty(&mut self) {
        for tree_number in 0..self.game.trees.len() {
            let mut tent_numbers = Vec::<usize>::new();
            for tent in &self.game.trees[tree_number].tents {
                tent_numbers.push(tent.number);
            }
            &self.exactly_n(1, tent_numbers);
        }
        for row in 0..self.game.max_row {
            let mut tents_in_row = Vec::<usize>::new();
            for column in 0..self.game.max_column {
                for tent_number in &self.game.tents_map[&(row, column)] {
                    tents_in_row.push(*tent_number);
                }
            }
            &self.exactly_n(self.game.tents_in_rows[row], tents_in_row);
        }
        for column in 0..self.game.max_column {
            let mut tents_in_column = Vec::<usize>::new();
            for row in 0..self.game.max_row {
                for tent_number in &self.game.tents_map[&(row, column)] {
                    tents_in_column.push(*tent_number);
                }
            }
            &self.exactly_n(self.game.tents_in_columns[column], tents_in_column);
        }
    }

    pub fn solve_sat(&self) {
        write_file("src/tents_encoded.cnf", &self.clauses);
        let cmd = std::process::Command::new("../cadical-sc2020-45029f8/build/cadical")
            .args(&["-q", "src/tents_encoded.cnf"])
            .output()
            .expect("failed to execute process");
        let sol = cmd.stdout;
        let res = format!("{}", String::from_utf8_lossy(&sol));
        println!("{}", res);
        let truth_values = self.convert_to_true(res);
        let mut game_content: Vec<&str> = self.game.content.split_whitespace().collect();
        let mut sol_content = String::new();
        sol_content.push_str(game_content[0]);
        sol_content.push_str(" ");
        sol_content.push_str(game_content[1]);
        sol_content.push_str("\n");
        for row in 0..self.game.max_row {
            for column in 0..self.game.max_column {
                let mut is_tent = false;
                for tent_number in &self.game.tents_map[&(row, column)] {
                    is_tent = is_tent || truth_values[tent_number - 1];
                }
                if is_tent {
                    game_content[2 + (self.game.max_row + 1) * row + column] = "X";
                }
                sol_content.push_str(game_content[2 + (self.game.max_row + 1) * row + column]);
                sol_content.push_str(" ");
            }
            sol_content.push_str(game_content[1 + (self.game.max_row + 1) * (row + 1)]);
            sol_content.push_str("\n");
        }
        for column in 0..self.game.max_column {
            sol_content
                .push_str(game_content[2 + (self.game.max_row + 1) * self.game.max_row + column]);
            sol_content.push_str(" ");
        }
        println!("{}", sol_content);
    }

    fn convert_to_true(&self, res: String) -> Vec<bool> {
        let mut truth_values = vec![false; self.game.variables_qty];
        let variables = res.split_whitespace();
        for i in variables {
            if i.parse::<i32>().is_ok() {
                let j = i.parse::<i32>().unwrap();
                if j > 0 {
                    truth_values[(j - 1) as usize] = true;
                }
            }
        }
        truth_values
    }
}

fn read_file(path: &str) -> String {
    use std::io::Read;
    let mut file = std::fs::File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}

fn write_file(path: &str, text: &String) {
    use std::io::Write;
    let mut file = std::fs::File::create(path).expect("create failed");
    file.write_all(text.as_bytes()).expect("write failed");
    println!("data written to file");
}
