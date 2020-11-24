struct Tent{
    position:(usize,usize),
    number:usize,
}



struct Tree{
    position:(usize,usize),
    tents:Vec<Tent>,
}

struct Game{
    trees:Vec<Tree>,
    max_column:usize,
    max_row:usize,
}


impl Game {

    pub fn new(path:String)->Self{
        let mut instance =Self{
            trees:[],
            max_column:0,
            max_row:0,
        }
        
        let input =read_file(path);
        let mut row:usize =0;
        let mut column:usize =0;
        let mut next_number=1;
        for i in input.split_whitepace(){
            if i=='T'{
                let new_tree=Tree{
                    position:(row,column),
                    tents:[],
                }
                next_number+=new_tree.tents.len();
                self.trees.push(new_tree);
                column+=1;
            }
            else if i=='.'{
                column+=1;
            }
            else if i=='\n'{
                column=0;
                row+=1;
            }
        }
        for t in self.trees{
            t.tents=get_tents(&self,(row,column),next_number);
        }
    }

    fn get_tents(&self,position:(usize,usize),next_number:usize)->Vec<Tent>{
        let mut new_tents:Vec<Tent>;
        for pos in get_neighbors(&self,position){
            if is_free(&self,pos){
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
    fn is_free(&self,position)->bool{
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
        if position[0]>0{
            neighbors.push(position[0]-1,position[1]);
        }
        if position[1]>0{
            neighbors.push(position[0],position[1]-1);
        }
        if position[0]+1<self.max_row{
            neighbors.push(position[0]+1,position[1]);
        }
        if postion[1]+1<self.max_column{
            neighbors.push(position[0],position[1]+1);
        }
        neighbors
    }

    fn read_file(path:String)->String{
        use std::io::Read;
        use std;
        let mut file = std::fs::File::open(path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        contents
    }

}
