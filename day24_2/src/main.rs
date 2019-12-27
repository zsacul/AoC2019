use std::collections::HashSet;

#[derive(Clone,  Debug, PartialEq, Eq)]
struct Universe {
    width: u32,
    height: u32,
    cells: Vec<u8>,
}

//#[derive(Clone,Debug)]
struct Cell
{
    level: i32,
    neighbours:Vec<(i32,i32)>,
    val : char,
}

impl Cell{
    fn new(lev:i32)->Cell{
        Cell {
            level:lev,
            val : ' ',
            neighbours:vec![],
        }
    }

    fn add_neighbour(&mut self,level:i32,num:i32){
        self.neighbours.push((level,num));
    }
}

struct Field{
    cells: Vec<Cell>,
    level: i32,
}

impl<'a> Field{
    fn new(lev:i32)->Field{
        Field{
            cells:vec![],
            level:lev,
        }
    }

    fn fill_val(&mut self,s:String){
        for i in 0..25 {
                let c = s.chars().nth(i).unwrap();
                if c=='#' {
                    self.cells[i].val = '#';
                }
                else {
                    self.cells[i].val = ' ';
                }
            }
    }
}

struct Fields{
    fields: Vec<Field>,
    level: i32,
}

impl<'a> Fields{
    fn new(lev:i32)->Fields
    {
        Fields{
            fields: vec![],
            level : lev,
        }
    }

    fn fill(&mut self,lev:i32,s:&str)
    {        
        for i in -5..=5
        {
            println!("i=={}",i);
            let mut f = Field::new(6+i);
            if i==0 { f.fill_val(s.to_string()); }
            self.fields.push(f);
        }
    }

    fn add(&mut self,orgl:i32 ,cell:i32,l:i32,neig:i32){
        let lev = self.level;
        let cell_n = cell-1;
        let neig_n = neig-1;
        //let mut fields = &
        //let cell = &mut self.fields[(orgl+l) as usize].cells[neig_n as usize];
        self.fields[orgl as usize].cells[cell_n as usize].add_neighbour(l,neig_n);        
        
    }

    fn fill_neighbours(&mut self,orgl:i32)
    {
        let l = self.level;
        self.add(orgl,1, 1,21);
        self.add(orgl,1, 0,2);
        self.add(orgl,1, 0,6);
        self.add(orgl,1, 1,5);

        self.add(orgl,2, 0,1);
        self.add(orgl,2, 0,3);
        self.add(orgl,2, 0,7);
        self.add(orgl,2, 1,22);

        self.add(orgl,3, 0,2);
        self.add(orgl,3, 0,4);
        self.add(orgl,3, 0,8);
        self.add(orgl,3, 1,23);
        
        self.add(orgl,4, 0,3);
        self.add(orgl,4, 0,5);
        self.add(orgl,4, 0,9);
        self.add(orgl,4, 1,24);
        
        self.add(orgl,5, 0,4);
        self.add(orgl,5, 0,10);
        self.add(orgl,5, 1,25);
        self.add(orgl,5, 1,1);
       
        self.add(orgl,6, 0,1);
        self.add(orgl,6, 0,7);
        self.add(orgl,6, 0,11);
        self.add(orgl,6, 1,10);
       
        self.add(orgl,7, 0,2);
        self.add(orgl,7, 0,6);
        self.add(orgl,7, 0,8);
        self.add(orgl,7, 0,12);

        self.add(orgl,8, 0,7);
        self.add(orgl,8, 0,3);
        self.add(orgl,8, 0,9);
        self.add(orgl,8, 1,21);
        self.add(orgl,8, 1,22);
        self.add(orgl,8, 1,23);
        self.add(orgl,8, 1,24);
        self.add(orgl,8, 1,25);

        self.add(orgl,9, 0,4);
        self.add(orgl,9, 0,8);
        self.add(orgl,9, 0,10);
        self.add(orgl,9, 0,14);

        self.add(orgl,10, 0,5);
        self.add(orgl,10, 0,9);
        self.add(orgl,10, 0,15);
        self.add(orgl,10,-1,6);

        self.add(orgl,11, 0,6);
        self.add(orgl,11, 0,12);
        self.add(orgl,11, 0,16);
        self.add(orgl,11,-1,15);

        self.add(orgl,12, 0,7);
        self.add(orgl,12, 0,11);
        self.add(orgl,12, 0,17);
        self.add(orgl,12, 1,1);
        self.add(orgl,12, 1,6);
        self.add(orgl,12, 1,11);
        self.add(orgl,12, 1,16);
        self.add(orgl,12, 1,21);

        self.add(orgl,14,  0,9);
        self.add(orgl,14, 0,15);
        self.add(orgl,14, 0,19);
        self.add(orgl,14, 1,5);
        self.add(orgl,14, 1,10);
        self.add(orgl,14, 1,15);
        self.add(orgl,14, 1,20);
        self.add(orgl,14, 1,25);

        self.add(orgl,15, 0,10);
        self.add(orgl,15, 0,14);
        self.add(orgl,15, 0,20);
        self.add(orgl,15,-1,11);        
        self.add(orgl,16, 0,11);
        self.add(orgl,16, 0,17);
        self.add(orgl,16, 0,21);
        self.add(orgl,16,-1,20);        
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
        self.add(orgl,20,-1,16); 

        self.add(orgl,21, 0,16);
        self.add(orgl,21, 0,22);
        self.add(orgl,21,-1,25);
        self.add(orgl,21,-1,1);   

        self.add(orgl,22, 0,17);
        self.add(orgl,22, 0,21);
        self.add(orgl,22, 0,23);
        self.add(orgl,22,-1,2);    

        self.add(orgl,23, 0,18);
        self.add(orgl,23, 0,22);
        self.add(orgl,23, 0,24);
        self.add(orgl,23,-1,3);     

        self.add(orgl,24, 0,19);
        self.add(orgl,24, 0,23);
        self.add(orgl,24, 0,25);
        self.add(orgl,24,-1,4);    

        self.add(orgl,25, 0,20);
        self.add(orgl,25, 0,24);
        self.add(orgl,25,-1,5);
        self.add(orgl,25,-1,21);        
    }
}


impl Universe {
    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    fn live_neighbor_count(&self, row: i32, column: i32) -> u8 {
        let mut count = 0;
        for delta_row in [-1, 0, 1].iter().cloned() {
            for delta_col in [-1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }
                if delta_row != 0 && delta_col != 0 {
                    continue;
                }

                let mut neighbor_row = row + delta_row;// % self.height;
                let mut neighbor_col = column + delta_col;// % self.width;
                let mut skip = false;
                if neighbor_col<0 { skip = true; }
                if neighbor_row<0 { skip = true; }
                if neighbor_col>=self.width as i32  { skip = true; }
                if neighbor_row>=self.height as i32 { skip = true; }

                if !skip
                {
                    let idx = self.get_index(neighbor_row as u32, neighbor_col as u32);
                    count += self.cells[idx] as u8;
                }
            }
        }
        count
    }

    pub fn tick(&mut self) {
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let idx  = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbors = self.live_neighbor_count(row as i32, col as i32);

                let next_cell = match (cell, live_neighbors) {
                    // Rule 1: Any live cell with fewer than two live neighbours
                    // dies, as if caused by underpopulation.
                    (1, x) if x !=1 => 0,
                    // Rule 2: Any live cell with two or three live neighbours
                    // lives on to the next generation.
                    (0, 1)  => 1,
                    // Rule 3: Any live cell with more than three live
                    // neighbours dies, as if by overpopulation.
                    (0, 2) => 1,
                    // Rule 4: Any dead cell with exactly three live neighbours
                    // becomes a live cell, as if by reproduction.
                    
                    // All other cells remain in the same state.
                    (otherwise, _) => otherwise,
                };

                next[idx] = next_cell;
            }
        }

        self.cells = next;
    }    

    pub fn new() -> Universe {
        let width  = 5;
        let height = 5;

        
//        let cells = "....##..#.#..##..#..#....".chars()
        let cells = "#....#...###.##....##.##.".chars()
            .map(|i| {
                if i=='.' {
                    0
                } else {
                    1
                }
            })
            .collect();

        Universe {
            width,
            height,
            cells,
        }
    }

    pub fn render(&self) -> String {

        let mut res : String = "".to_string();

        for c in self.cells.clone() {
            if c>0 {
                res.push('#');                
            }
              else
            {
                res.push('.');                
            }
        }

        res //self.cells.clone()
    }    

}


use std::fmt;

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == 0 { 1 } else { 0 };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}


fn main() {

    let mut fields = Fields::new(0);

    println!("1");
    fields.fill(0,"#....#...###.##....##.##.");
    println!("2");
    

    /*
    let mut u = Universe::new();
    let mut hash: HashSet<String> = HashSet::new();

    loop {
        let c = u.render();
        if hash.get(&c)!=None
        {
            println!("{:?}",c);

            let mut pow = 1;
            let mut sum = 0i128;

            for i in 0..25 {
                if c.chars().nth(i).unwrap()=='#' { sum+=pow;  }
                pow*=2;
            }
            println!("sum:{}",sum);
            break;
        }
        hash.insert(c);
        u.tick();
    }
    */
    //println!("{:?}",u.render());
   // println!("Hello, world!");
}

//#..#.####.###.###.##.##..