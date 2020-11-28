struct Tent{
    position:(usize,usize),
    number:usize,
}

struct Tree{
    position:(usize,usize),
    tents:Vec<Tent>,
}

pub struct Game{
    trees:Vec<Tree>,
    max_column:usize,
    max_row:usize,
    tents_in_rows:Vec<usize>,
    tents_in_columns:Vec<usize>,
}


impl Game {

    pub fn new(path:String)->Self{
        let content:String=read_file(path);
        let input:Vec<&str> =content.split_whitespace().collect();
        let mut this =Self{
            trees:Vec::<Tree>::new(),
            max_column:input[1].parse::<usize>().unwrap(),
            max_row:input[0].parse::<usize>().unwrap(),
            tents_in_rows: Vec::<usize>::new(),
            tents_in_columns: Vec::<usize>::new(),
        };
               
        let mut row:usize =0;
        let mut column:usize =0;
        let mut index=0;
        let end=(this.max_column+1)*this.max_row+3;
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
                this.tents_in_rows[row]=i.parse::<usize>().unwrap();
            }            
        }
        index=0;
        for j in &input[end..] {
            this.tents_in_columns[index]=j.parse::<usize>().unwrap();
            index+=1;
        }
        let mut next_number=1;
        for mut tree in this.trees{
            for tent in this.get_tents(tree.position,next_number){
                tree.tents.push(tent);
            }
            next_number+=tree.tents.len();
        }
        this
    }

    fn get_tents(&mut self,position:(usize,usize),next_number:usize)->Vec<Tent>{
        let mut new_tents:Vec<Tent>;
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
        for i in self.trees{
            if i.position==position{
                free=false;
            }
        }
        free
    }
    fn get_neighbors(&self,position:(usize,usize))->Vec<(usize,usize)>{
        let mut neighbors:Vec<(usize,usize)>;
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

fn read_file(path:String)->String{
    use std::io::Read;
    use std;
    let mut file = std::fs::File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}
