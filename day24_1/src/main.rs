use std::collections::HashSet;

#[derive(Clone,  Debug, PartialEq, Eq)]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<u8>,
}

impl Universe {
    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    fn live_neighbor_count(&self, row: i32, column: i32) -> u8 {
        let mut count = 0;
        for delta_row in [-1, 0, 1].iter().cloned() {
            for delta_col in [-1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 { continue; }
                if delta_row != 0 && delta_col != 0 { continue; }

                let neighbor_row = row    + delta_row;
                let neighbor_col = column + delta_col;
                
                if neighbor_col>=0 && 
                   neighbor_row>=0 &&
                   neighbor_col<self.width as i32 &&
                   neighbor_row<self.height as i32
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
                    (1, x) if x!=1 => 0,
                    (0, 1)         => 1,
                    (0, 2)         => 1,
                    (otherwise, _) => otherwise,
                };

                next[idx] = next_cell;
            }
        }

        self.cells = next;
    }    

    pub fn new(s:&str) -> Universe {
        let width  = 5;
        let height = 5;
        let cells = s.chars()
            .map(|i| {
                if i=='.' { 0 } else { 1 }
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

        for c in &self.cells {            
            res.push( if *c>0 {'#'} else {'.'} );                
        }

        res
    }    

}

fn main() {
    let mut u = Universe::new("#....#...###.##....##.##.");
    let mut hash: HashSet<String> = HashSet::new();

    loop {
        let c = u.render();
        if hash.get(&c)!=None
        {
            println!("{:?}",c);

            let mut sum = 0i128;

            for i in 0..25 {
                if c.chars().nth(i).unwrap()=='#' { sum+=1<<i;  }
            }
            println!("sum:{}",sum);
            break;
        }
        hash.insert(c);
        u.tick();  
    }
}