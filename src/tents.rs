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
    content: Vec<String>,
}

#[derive(Debug)]
pub struct SatMaker {
    clauses: String,
    game: Game,
    clauses_qty: usize,
    truth_values: Vec<bool>,
}


impl Game {
    fn new(path: &str) -> Self {
        let content: String = read_file(path);
        let content_clone = content.clone();
        let input: Vec<&str> = (&content_clone).split_whitespace().collect();
        let formated=Game::format_properly(input);
        let mut this = Self {
            trees: Vec::<Tree>::new(),
            max_column: formated[1].parse::<usize>().unwrap(),
            max_row: formated[0].parse::<usize>().unwrap(),
            tents_in_rows: Vec::<usize>::new(),
            tents_in_columns: Vec::<usize>::new(),
            tents_map: HashMap::new(),
            variables_qty: 0,
            content: formated,
        };

        let mut row: usize;
        let mut column: usize;
        let mut index = 0;
        let end = (this.max_column + 1) * this.max_row + 2;
        for i in &this.content[2..end] {
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
        for j in &this.content[end..] {
            this.tents_in_columns.push(j.parse::<usize>().unwrap());
        }

        for tree_number in 0..this.trees.len() {
            this.trees[tree_number].tents =
                this.get_tents(this.trees[tree_number].position, this.variables_qty + 1);
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
    fn format_properly(input:Vec<&str>)->Vec<String>{
        let mut formated=Vec::<String>::new();
        for i in &input{
            let first_char=i.chars().next().unwrap();
            if first_char=='T'||first_char=='.'{
                for j in i.chars(){

                    formated.push(j.to_string());
                }
            }
            else{
                formated.push(i.to_string());
            }
        }
        formated
    }
}

impl SatMaker {
    pub fn new(path: &str) -> Self {
        let mut this = Self {
            clauses: String::new(),
            game: Game::new(path),
            clauses_qty: 0,
            truth_values:Vec::<bool>::new(),
        };
        &this.none_adjacent();
        &this.ensure_tent_qty_pools();
        this.clauses = format!(
            "p cnf {} {}\n{}",
            this.game.variables_qty, this.clauses_qty, this.clauses
        );
        this
    }
    fn encode_dfa(&mut self,input:Vec<Vec<usize>>,max_tents:usize){
        //println!("new dfa with: {:?} {}",input,max_tents);
        let mut number_dict=HashMap::<(usize, usize), usize>::new();
        let mut number_dict_return:usize;
        let mut number_dict_return2:usize;
        for tent_number in &input[0]{
            number_dict_return=self.update_use_dict(&mut number_dict,(1,1));
            &self.clauses.push_str(&format!("-{} {} 0\n",tent_number,number_dict_return));
            self.clauses_qty+=1;
        }
        for tent_number in &input[0]{
            &self.clauses.push_str(&format!("{} ",tent_number));
        }
        number_dict_return=self.update_use_dict(&mut number_dict,(1,0));
        &self.clauses.push_str(&format!("{} 0\n",number_dict_return));
        self.clauses_qty+=1;
       
        for step in 1..input.len()-1{
            for tent_until in 0..max_tents+1{
                if number_dict.contains_key(&(step,tent_until)){
                    if input.len()-step!=max_tents-tent_until{
                        for tent_number in &input[step]{
                            &self.clauses.push_str(&format!("{} ",tent_number));
                        }
                        number_dict_return=self.update_use_dict(&mut number_dict,(step+1,tent_until));
                        number_dict_return2=self.update_use_dict(&mut number_dict,(step,tent_until));
                        &self.clauses.push_str(&format!("{} -{} 0\n",number_dict_return,number_dict_return2));
                        self.clauses_qty+=1;
                        for tent_number in &input[step]{
                            &self.clauses.push_str(&format!("{} ",tent_number));
                        }
                        number_dict_return=self.update_use_dict(&mut number_dict,(step+1,tent_until));
                        number_dict_return2=self.update_use_dict(&mut number_dict,(step,tent_until));
                        &self.clauses.push_str(&format!("-{} {} 0\n",number_dict_return,number_dict_return2));
                        self.clauses_qty+=1;
                    }
                    else{
                        for tent_number in &input[step]{
                            &self.clauses.push_str(&format!("{} ",tent_number));
                        }
                        number_dict_return=self.update_use_dict(&mut number_dict,(step,tent_until));
                        &self.clauses.push_str(&format!("-{} 0\n",number_dict_return));
                        self.clauses_qty+=1;

                    }
                    if tent_until<max_tents{
                        for tent_number in &input[step]{
                            number_dict_return=self.update_use_dict(&mut number_dict,(step+1,tent_until+1));
                            number_dict_return2=self.update_use_dict(&mut number_dict,(step,tent_until));
                            &self.clauses.push_str(&format!("-{} {} -{} 0\n",tent_number,number_dict_return,number_dict_return2));
                            &self.clauses.push_str(&format!("-{} -{} {} 0\n",tent_number,number_dict_return,number_dict_return2));
                            self.clauses_qty+=2;
                        }
                    }
                }
            }
            let mut bools_per_step = Vec::<usize>::new();
            for (pos,bool_number) in number_dict.iter(){
                if pos.0==step{
                    bools_per_step.push(*bool_number);
                }
            }
            self.not_two(bools_per_step);
        }
        &self.clauses.push_str(&format!("-{} ",number_dict.get(&(input.len()-1,max_tents-1)).unwrap()));
        for tent_number in &input[input.len()-1]{
            self.clauses.push_str(&format!("{} ",tent_number));
        }
        &self.clauses.push_str(&"0\n");
        self.clauses_qty+=1;
        
        for tent_number in &input[input.len()-1]{
            &self.clauses.push_str(&format!("-{} -{} 0\n",number_dict.get(&(input.len()-1,max_tents)).unwrap(),tent_number));
            self.clauses_qty+=1;
        }
        &self.clauses.push_str(&format!("-{} -{} 0\n",number_dict.get(&(input.len()-1,max_tents-1)).unwrap(),
                                                     number_dict.get(&(input.len()-1,max_tents)).unwrap()));
        self.clauses_qty+=1;
    }

    fn not_two(&mut self,bools_per_step:Vec<usize>){
        for i in 0..bools_per_step.len(){
            for j in i+1..bools_per_step.len(){
                self.clauses.push_str(&format!("-{} -{} 0\n",bools_per_step[i],bools_per_step[j]));
                self.clauses_qty+=1;
            }
        }
        for i in bools_per_step{
            self.clauses.push_str(&format!("{} ",i));
        }
        self.clauses.push_str("0\n");
        self.clauses_qty+=1;
    }
    fn update_use_dict(& mut self,mut number_dict:&mut HashMap<(usize, usize),usize>,pos:(usize,usize))->usize{
        if number_dict.contains_key(&pos){
            *number_dict.get(&pos).unwrap()
        }
        else{
            self.game.variables_qty+=1;
            number_dict.insert(pos,self.game.variables_qty);
            self.game.variables_qty
        }
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
    
    fn choose_from_pool(&mut self,pools: Vec<Vec<usize>>, mut k: usize,with_not:bool) {
        let mut pool_choice = Vec::<Vec<usize>>::new();
        let combinations = Self::n_choose_k(pools.len(),k);
        let mut teststr=String::new();
        for combi in combinations {
            teststr="".to_string();
            pool_choice.clear();
            for i in &combi {
                pool_choice.push(pools[*i].clone());
            }
            if with_not{
                for claus in &pool_choice{
                    for number in claus{
                        self.clauses.push_str(&format!("-{} ", number));
                        //teststr.push_str(&format!("-{} ", number));
                    }

                }
                self.clauses.push_str("0\n");
                self.clauses_qty += 1;
                //println!("{:?}  {:?}  {}",combi,pool_choice,teststr)
                
            }
            else{
                for claus in &pool_choice{
                    for number in claus{
                        self.clauses.push_str(&format!("{} ", number));
                        //teststr.push_str(&format!("{} ", number));
                    }
                }
                self.clauses.push_str("0\n");
                self.clauses_qty += 1;
                //println!("{:?}  {:?}  {}",combi,pool_choice,teststr)
            }
        }
    }
    fn exactly_n_from_pool(&mut self, n: usize, pools: Vec<Vec<usize>>) {
        self.choose_from_pool(pools.clone(),n+1,true);
        self.choose_from_pool(pools.clone(),pools.len()-n+1,false);
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

    fn ensure_tent_qty_pools(&mut self) {
        for tree_number in 0..self.game.trees.len() {
            let mut tent_numbers = Vec::<usize>::new();
            for tent in &self.game.trees[tree_number].tents {
                tent_numbers.push(tent.number);
            }
            &self.exactly_n(1, tent_numbers);
        }
        for row in 0..self.game.max_row {
            let mut tents_in_row = Vec::<Vec<usize>>::new();
            for column in 0..self.game.max_column {
                if self.game.tents_map[&(row, column)].len()>0{
                    tents_in_row.push(self.game.tents_map[&(row, column)].clone());  
                }         
            }
            //&self.exactly_n_from_pool(self.game.tents_in_rows[row], tents_in_row);
            
            if self.game.tents_in_rows[row]==0{
                for tents in tents_in_row{
                    for tent in tents{
                        self.clauses.push_str(&format!("-{} 0\n",tent));
                        self.clauses_qty+=1;
                    }
                }
            }
            else if self.game.tents_in_rows[row]==tents_in_row.len(){
                for tents in tents_in_row{
                    for tent in tents{
                        self.clauses.push_str(&format!(" {}",tent));    
                    }
                    self.clauses.push_str(&format!(" 0\n"));
                    self.clauses_qty+=1;
                }
            }
            else{
                self.encode_dfa(tents_in_row,self.game.tents_in_rows[row])
            }   
            
        }
        for column in 0..self.game.max_column {
            let mut tents_in_column = Vec::<Vec<usize>>::new();
            for row in 0..self.game.max_row {
                if self.game.tents_map[&(row, column)].len()>0{
                    tents_in_column.push(self.game.tents_map[&(row, column)].clone());  
                }
            }
            //&self.exactly_n_from_pool(self.game.tents_in_columns[column], tents_in_column);
            
            if self.game.tents_in_columns[column]==0{
                for tents in tents_in_column{
                    for tent in tents{
                        self.clauses.push_str(&format!("-{} 0\n",tent));
                        self.clauses_qty+=1;
                    }
                }
            }
            else if self.game.tents_in_columns[column]==tents_in_column.len(){
                for tents in tents_in_column{
                    for tent in tents{
                        self.clauses.push_str(&format!(" {}",tent));    
                    }
                    self.clauses.push_str(&format!(" 0\n"));
                    self.clauses_qty+=1;
                }
            }
            else{
                self.encode_dfa(tents_in_column,self.game.tents_in_columns[column])
            }
            
        }
    }

    pub fn solve_sat(&mut self){
        write_file("src/tents_encoded.cnf", &self.clauses);
        let cmd = std::process::Command::new("cadical-sc2020-45029f8/build/cadical")
            .args(&["-q", "src/tents_encoded.cnf"])
            .output()
            .expect("failed to execute process");
        let sol = cmd.stdout;
        let res = format!("{}", String::from_utf8_lossy(&sol));
        //println!("{}",res);
        self.truth_values = self.convert_to_true(res);
        let game_content = &mut self.game.content;
        let mut sol_content = String::new();
        sol_content.push_str(&game_content[0]);
        sol_content.push_str(" ");
        sol_content.push_str(&game_content[1]);
        sol_content.push_str("\n");
        let mut tent_pos=Vec::<(usize,usize)>::new();
        for row in 0..self.game.max_row {
            for column in 0..self.game.max_column {
                let mut is_tent = false;
                for tent_number in &self.game.tents_map[&(row, column)] {
                    is_tent = is_tent || self.truth_values[tent_number - 1];
                }
                if is_tent {
                    game_content[2 + (self.game.max_column + 1) * row + column] = "X".to_string();
                    tent_pos.push((row,column));
                }
                sol_content.push_str(&game_content[2 + (self.game.max_column + 1) * row + column]);
                sol_content.push_str(" ");
            }
            sol_content.push_str(&game_content[1 + (self.game.max_column + 1) * (row + 1)]);
            sol_content.push_str("\n");
        }
        for column in 0..self.game.max_column {
            sol_content
                .push_str(&game_content[2 + (self.game.max_column + 1) * self.game.max_row + column]);
            sol_content.push_str(" ");
        }
        println!("{:?}", tent_pos);
        //self.find_unsat_clause();
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
    pub fn unique_check(&mut self){
        let mut k=1;
        self.clauses_qty+=1;
        for t in &self.truth_values{
            if *t{
                self.clauses.push_str(&format!(" -{}",k));
            }
            else{
                self.clauses.push_str(&format!(" {}",k));
            }
            //k+=1;
        }
        self.clauses.push_str(" 0\n");
        self.solve_sat();
    }

    fn find_unsat_clause(&self){
        let mut clause_list =self.clauses.split_whitespace();
        let mut k=0;
        let mut is_true=false;
        let mut clause_str=String::new();
        for i in clause_list{
            if 3<k{
                let j = i.parse::<i32>().unwrap();
                clause_str.push_str(&format!("{} ",i));
                if j>0{
                    is_true=is_true||self.truth_values[(j-1) as usize];
                }
                else if j<0{
                    is_true=is_true||!self.truth_values[(-j-1) as usize];
                }
                else{
                    if !is_true{
                        println!("{} {}",is_true,clause_str);
                    }
                    clause_str.clear();
                    is_true=false;
                }
            }
            k+=1;
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

fn write_file(path: &str, text: &String) {
    use std::io::Write;
    let mut file = std::fs::File::create(path).expect("create failed");
    file.write_all(text.as_bytes()).expect("write failed");
    //println!("data written to file");
}
