#[derive(Debug)]
struct Tent{
    position:(usize,usize),
    number:usize,
}

#[derive(Debug)]
struct Tree{
    position:(usize,usize),
    tents:Vec<Tent>,
}

#[derive(Debug)]
struct Game{
    trees:Vec<Tree>,
    max_column:usize,
    max_row:usize,
    tents_in_rows:Vec<usize>,
    tents_in_columns:Vec<usize>,
}

#[derive(Debug)]
pub struct SatMaker{
    clauses:String,
    game:Game,
}


impl Game {

    pub fn new(path:&str)->Self{
        let content:String=read_file(path);
        let input:Vec<&str> =content.split_whitespace().collect();
        let mut this =Self{
            trees:Vec::<Tree>::new(),
            max_column:input[1].parse::<usize>().unwrap(),
            max_row:input[0].parse::<usize>().unwrap(),
            tents_in_rows: Vec::<usize>::new(),
            tents_in_columns: Vec::<usize>::new(),
        };
               
        let mut row:usize;
        let mut column:usize;
        let mut index=0;
        let end=(this.max_column+1)*this.max_row+2;
        for i in &input[2..end]{
            row=index/this.max_column;
            column=index%this.max_column;
            if *i=="T" || *i=="." {
                if *i=="T"{
                    let new_tree=Tree{
                        position:(row,column),
                        tents:Vec::<Tent>::new(),
                    };
                    this.trees.push(new_tree);
                }
                index+=1;             
            } else {
                this.tents_in_rows.push(i.parse::<usize>().unwrap());
            }            
        }
        for j in &input[end..] {
            this.tents_in_columns.push(j.parse::<usize>().unwrap());
        }
        let mut next_number=1;
        
        for tree_number in 0..this.trees.len(){
            this.trees[tree_number].tents = this.get_tents(this.trees[tree_number].position,next_number);
            next_number+=this.trees[tree_number].tents.len();
        }
        this
    }

    fn get_tents(&self,position:(usize,usize),mut next_number:usize)->Vec<Tent>{
        let mut new_tents=Vec::<Tent>::new();
        for pos in self.get_neighbors(position){
            if self.is_free(pos){
                let new_tent=Tent{
                    position:pos,
                    number:next_number,
                };
                new_tents.push(new_tent);
                next_number+=1;
            }
        }
        new_tents
    }
    fn is_free(&self,position:(usize,usize))->bool{
        let mut free=true;
        for i in &self.trees{
            if i.position==position{
                free=false;
            }
        }
        free
    }
    fn get_neighbors(&self,position:(usize,usize))->Vec<(usize,usize)>{
        let mut neighbors=Vec::<(usize,usize)>::new();
        if position.0>0{
            neighbors.push((position.0-1,position.1));
        }
        if position.1>0{
            neighbors.push((position.0,position.1-1));
        }
        if position.0+1<self.max_row{
            neighbors.push((position.0+1,position.1));
        }
        if position.1+1<self.max_column{
            neighbors.push((position.0,position.1+1));
        }
        neighbors
    }
}

impl SatMaker {
    pub fn new(path:&str)->Self{
        let this=Self{
            clauses:String::new(),
            game: Game::new(path),
        };
        this
    }

    pub fn n_choose_k(n:usize,mut k:usize)->Vec<Vec<usize>>{
        let mut res_old=Vec::<Vec::<usize>>::new();
        let mut res_new=Vec::<Vec::<usize>>::new();
        while k>0{
            for mut group in &res_old{
                let begin:usize;
                if group.len()>0{
                    begin=group[group.len()-1]+1;
                } else {
                    begin=0;
                }
                for i in begin..n-k{                   
                    res_new.push(group.clone().push(i));
                }
            }
            k-=1;
            res_old=res_new;
            res_new=Vec::<Vec::<usize>>::new();
        }
        res_old
    }

    fn exactly_n(&self,n:usize,tent_indices:Vec<usize>){

    }

    fn none_adjacent(&self){

    }

    fn tent_numbers(&self){

    }
}

fn read_file(path:&str)->String{
    use std::io::Read;
    use std;
    let mut file = std::fs::File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}
