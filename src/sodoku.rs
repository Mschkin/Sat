use std::process::Command;


pub fn organiser(){
    
    let mut s=String::from("p cnf 729 41010\n");
    for i in 0..9{
        for j in 0..9{
            s.push_str(&cnf_only_one_number(i,j));
        }
        s.push_str(&no_dublicate_row(i));
        s.push_str(&no_dublicate_col(i));
    }
    for i in 0..3{
        for j in 0..3{
            s.push_str(&no_dublicate_block(i,j));
        }
    }
    let sodo=read_in_soduku("src\\test.txt".to_string());
    let mut line=0;
    let mut row=0;
    for i in sodo.chars(){
        if i=='\n'{
            line+=1;
            row=0;
        }
        else if i!='.'&& i!='\r'{
            s.push_str(&index_to_number(line,row,i as i32 - '1' as i32));
            s.push_str(" 0\n");
            row+=1;
        }
        else if i=='.'{
            row+=1;
        }
    }
    write_file(String::from("src\\test_encoded.cnf"),s);
    let cmd = 
    Command::new("cadical").args(&["-q","src\\test_encoded.cnf"]).output().expect("failed to execute process");
    let sol = cmd.stdout;
    //println!("{:?}",String::from_utf8_lossy(&sol));
    s=format!("{}",String::from_utf8_lossy(&sol));
    let mut arr:[bool;729]=[false;729];
    arr=convert_to_true(s);
    s=String::from("");
    for r in 0..9{
        if r%3==0{
            s.push_str("--------------------\n");
        }
        for c in 0..9{
            if c%3==0{
                s.push('|');
            }
            for i in 0..9{
                if arr[index_to_number(r,c,i).parse::<usize>().unwrap()-1]{
                    s.push_str(&format!("{} ",i+1));
                }
            }
            
        }
        s.push_str("|\n");
    }
    s.push_str("--------------------\n");
    println!("{}",s);
}

fn convert_to_true(s:String)->[bool;729]{
    let mut arr:[bool;729]=[false;729];
    let mut num = s.split_whitespace();
    for i in num{
        if i.parse::<i32>().is_ok(){
            let mut j =i.parse::<i32>().unwrap();
            if j>0{
                arr[(j-1) as usize]=true;
            }
        }
    }
    arr
}


fn read_in_soduku(path:String)->String{
    use std::io::Read;
    use std;
    let mut file = std::fs::File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}



fn write_file(path:String,text:String) {
    use std;
    use std::io::Write;
    let mut file = std::fs::File::create(path).expect("create failed");
    file.write_all(text.as_bytes()).expect("write failed");
    println!("data written to file" );
}

fn index_to_number(i1:i32,i2:i32,i3:i32)->String{
    (81*i1+9*i2+i3+1).to_string()
}

fn only_one_number(i1:i32,i2:i32)->String{
    let mut s=String::new();
    for i in 0..9{
        for j in 0..9{
            if i!=j{
                s.push('-');
            }
            s.push_str(&(index_to_number(i1,i2,j)+" "));
        }
        s.push_str("0\n");
    }
    s
}

fn cnf_only_one_number(i1:i32,i2:i32)->String{
    let mut s=String::new();
    let zeros=String::from("000000000");
    for i in 0..512{
        let i =format!("{:b}",i);
        let k = format!("{}{}",&zeros[0..9-i.len()],i);
        let mut one_count=0;
        for i in k.chars(){
            one_count+=i.to_digit(10).unwrap();
        }
        if one_count!=1{
            for (n,j) in k.chars().enumerate(){
                if j=='1'{
                    s.push('-');
                }
                s.push_str(&(index_to_number(i1,i2,n as i32)+" "));
            }
            s.push_str("0\n");
        }
        //println!("{} {}",k,one_count);
    }
    s
}

fn no_dublicate_row(i:i32)->String{
    let mut s= String::new();
    for j in 0..9{
        for k in 0..9{
            s.push_str(&(index_to_number(i,k,j)+" "));
        }
        s.push_str("0\n");
    }
    s
}

fn no_dublicate_col(i:i32)->String{
    let mut s= String::new();
    for j in 0..9{
        for k in 0..9{
            s.push_str(&(index_to_number(k,i,j)+" "));
        }
        s.push_str("0\n");
    }
    s
}

fn no_dublicate_block(i1:i32,i2:i32)->String{
    let mut s= String::new();
    for j in 0..9{
        for k in 0..3{
            for l in 0..3{
                s.push_str(&(index_to_number(i1*3+k,i2*3+l,j)+" "));
            }
        }
        s.push_str("0\n");
    }
    s
}