#[derive(Clone,Debug)]
struct Cell
{
    neighbours:Vec<(i32,i32)>,
    val : char,
}

impl Cell{
    fn new()->Cell{
        Cell {
            val : '.',
            neighbours:vec![],
        }
    }

    fn add_neighbour(&mut self,level:i32,num:i32){
        self.neighbours.push((level,num));
    }
}

#[derive(Clone,Debug)]
struct Field{
    cells: Vec<Cell>,
    level: i32,
}

impl<'a> Field{
    fn new(lev:i32)->Field{
        Field{
            cells:vec![Cell::new();25],
            level:lev,
        }
    }

    fn fill_val(&mut self,s:String){
        for i in 0..25 {
            self.cells[i].val = s.chars().nth(i).unwrap();;
        }
    }

    fn print(&self){
        let mut id = 0;
        for _i in 0..5 {
            println!("{}{}{}{}{}",self.cells[id].val,self.cells[id+1].val,self.cells[id+2].val,self.cells[id+3].val,self.cells[id+4].val);
            id+=5;
        }
    }
}

struct Fields{
    fields: Vec<Field>,
}

impl<'a> Fields{
    fn new()->Fields
    {
        Fields{
            fields: vec![],
        }
    }

    fn print(&self){
        for f in &self.fields {
            println!("level:{}",f.level);
            f.print();
        }
    }

    fn count(&self)->i32 {
        let mut res = 0;
        for f in &self.fields {
            for c in &f.cells {
                if c.val=='#' { res+=1; }
            } 
        }
        res
    }

    fn fill(&mut self,n:i32,s:&str)
    {        
        for i in -n..=n
        {
            let mut f = Field::new(i);
            if i==0 { f.fill_val(s.to_string()); }

            self.fields.push(f);
        }

        for i in -n+1..=n-1
        {
            self.fill_neighbours(n+i);
        }
    }
    
    fn live_neighbor_count(&self,neighbours:&Vec<(i32,i32)>)->i32
    {
        let mut res = 0;
        let levels = self.fields.len();

        //println!("neig:{}",neighbours.len());
        
        for n in neighbours {
            let (lev,num) = *n;
            //println!("{}/{}",lev,num);
            if lev>=0 && lev<levels as i32 && self.fields[lev as usize].cells[num as usize].val=='#' { res+=1; }
        }
        res
    }

    pub fn tick(&mut self) {
        let mut next = self.fields.clone();

        for n in 0..self.fields.len() {
            for num in 0..25 {
                
                let val  = self.fields[n].cells[num].val;
                let live_neighbors = self.live_neighbor_count(&self.fields[n].cells[num].neighbours);                

                let next_val = match (val, live_neighbors) {
                    ('#', x) if x !=1 => '.',
                    ('.', 1)          => '#',
                    ('.', 2)          => '#',
                    (otherwise, _)    => otherwise,
                };

                next[n].cells[num].val = next_val;
            }
        }

        self.fields = next;
    }    

    fn add(&mut self,orgl:i32,cell:i32,l:i32,neig:i32){
        let cell_n = cell-1;
        let neig_n = neig-1;
        self.fields[orgl as usize].cells[cell_n as usize].add_neighbour(orgl+l,neig_n);                
    }

    fn fill_neighbours(&mut self,orgl:i32)
    {
        self.add(orgl,1,-1, 8);
        self.add(orgl,1, 0, 2);
        self.add(orgl,1, 0, 6);
        self.add(orgl,1,-1,12);

        self.add(orgl,2, 0, 1);
        self.add(orgl,2, 0, 3);
        self.add(orgl,2, 0, 7);
        self.add(orgl,2,-1, 8);

        self.add(orgl,3, 0, 2);
        self.add(orgl,3, 0, 4);
        self.add(orgl,3, 0, 8);
        self.add(orgl,3,-1, 8);
        
        self.add(orgl,4, 0, 3);
        self.add(orgl,4, 0, 5);
        self.add(orgl,4, 0, 9);
        self.add(orgl,4,-1, 8);
        
        self.add(orgl,5, 0, 4);
        self.add(orgl,5, 0,10);
        self.add(orgl,5,-1, 8);
        self.add(orgl,5,-1,14);
       
        self.add(orgl,6, 0, 1);
        self.add(orgl,6, 0, 7);
        self.add(orgl,6, 0,11);
        self.add(orgl,6,-1,12);
       
        self.add(orgl,7, 0, 2);
        self.add(orgl,7, 0, 6);
        self.add(orgl,7, 0, 8);
        self.add(orgl,7, 0,12);

        self.add(orgl,8, 0, 7);
        self.add(orgl,8, 0, 3);
        self.add(orgl,8, 0, 9);
        self.add(orgl,8, 1, 1);
        self.add(orgl,8, 1, 2);
        self.add(orgl,8, 1, 3);
        self.add(orgl,8, 1, 4);
        self.add(orgl,8, 1, 5);

        self.add(orgl,9, 0, 4);
        self.add(orgl,9, 0, 8);
        self.add(orgl,9, 0,10);
        self.add(orgl,9, 0,14);

        self.add(orgl,10, 0, 5);
        self.add(orgl,10, 0, 9);
        self.add(orgl,10, 0,15);
        self.add(orgl,10,-1,14);

        self.add(orgl,11, 0, 6);
        self.add(orgl,11, 0,12);
        self.add(orgl,11, 0,16);
        self.add(orgl,11,-1,12);

        self.add(orgl,12, 0, 7);
        self.add(orgl,12, 0,11);
        self.add(orgl,12, 0,17);
        self.add(orgl,12, 1, 1);
        self.add(orgl,12, 1, 6);
        self.add(orgl,12, 1,11);
        self.add(orgl,12, 1,16);
        self.add(orgl,12, 1,21);

        self.add(orgl,14, 0, 9);
        self.add(orgl,14, 0,15);
        self.add(orgl,14, 0,19);
        self.add(orgl,14, 1, 5);
        self.add(orgl,14, 1,10);
        self.add(orgl,14, 1,15);
        self.add(orgl,14, 1,20);
        self.add(orgl,14, 1,25);

        self.add(orgl,15, 0,10);
        self.add(orgl,15, 0,14);
        self.add(orgl,15, 0,20);
        self.add(orgl,15,-1,14);    

        self.add(orgl,16, 0,11);
        self.add(orgl,16, 0,17);
        self.add(orgl,16, 0,21);
        self.add(orgl,16,-1,12);     

        self.add(orgl,17, 0,12);
        self.add(orgl,17, 0,16);
        self.add(orgl,17, 0,18);
        self.add(orgl,17, 0,22);        

        self.add(orgl,18, 0,17);
        self.add(orgl,18, 0,19);
        self.add(orgl,18, 0,23);
        self.add(orgl,18, 1,21);
        self.add(orgl,18, 1,22);
        self.add(orgl,18, 1,23);
        self.add(orgl,18, 1,24);
        self.add(orgl,18, 1,25);

        self.add(orgl,19, 0,14);
        self.add(orgl,19, 0,18);
        self.add(orgl,19, 0,20);
        self.add(orgl,19, 0,24);  

        self.add(orgl,20, 0,15);
        self.add(orgl,20, 0,19);
        self.add(orgl,20, 0,25);
        self.add(orgl,20,-1,14); 

        self.add(orgl,21, 0,16);
        self.add(orgl,21, 0,22);
        self.add(orgl,21,-1,12);
        self.add(orgl,21,-1,18);   

        self.add(orgl,22, 0,17);
        self.add(orgl,22, 0,21);
        self.add(orgl,22, 0,23);
        self.add(orgl,22,-1,18);    

        self.add(orgl,23, 0,18);
        self.add(orgl,23, 0,22);
        self.add(orgl,23, 0,24);
        self.add(orgl,23,-1,18);     

        self.add(orgl,24, 0,19);
        self.add(orgl,24, 0,23);
        self.add(orgl,24, 0,25);
        self.add(orgl,24,-1,18);    

        self.add(orgl,25, 0,20);
        self.add(orgl,25, 0,24);
        self.add(orgl,25,-1,14);
        self.add(orgl,25,-1,18);        
    }
}

fn main() {
    let mut fields = Fields::new();
    //fields.fill(10,"....##..#.#..##..#..#....");
    let minutes = 200;//10;
    fields.fill(minutes/2+1,"#....#...###.##....##.##.");
                     
    for _i in 0..minutes {
        fields.tick();
    }

    fields.print();
    println!("count:{}",fields.count());
}
