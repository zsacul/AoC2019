struct Calc {
    bases : Vec<Vec<i8>>,
    max_n : usize,
}

impl Calc
{
    fn new(l:usize)->Calc{        
        Calc{
            bases : vec![vec![0]],
            max_n : l,
        }
    }

    fn init(&mut self,i:i128){        
        for num in 1..=i {
            self.bases.push(self.get_base(num));
        }
    }        

    fn get_base(&self,i:i128)->Vec<i8>
    {
        let mut res = vec![];
        for _n in 0..i { res.push( 0); if res.len()>self.max_n { return res; } }
        for _n in 0..i { res.push( 1); if res.len()>self.max_n { return res; } }
        for _n in 0..i { res.push( 0); if res.len()>self.max_n { return res; } }
        for _n in 0..i { res.push(-1); if res.len()>self.max_n { return res; } }
        res
    }

    fn get_pattern(&self,i:i128,n:i128)->i8
    {
        let l = self.bases[i as usize].len() as i128;
        let id = ( ((n+1).abs())%l ) as usize;
    
        return self.bases[i as usize][id];
    }
    
    fn comp(&self,data:Vec<i128>)->Vec<i128>
    {
        let mut res : Vec<i128> = vec![];
        let mut i=1;

        for d in &data {    
            let mut sum = 0;
            let mut id = 0;
    
            for j in &data {
                sum+=*j*(self.get_pattern(i, id) as i128);
                id+=1;
            }
            res.push(sum.abs()%10i128);
            i+=1;
        }
        res
    }
}

fn main() {
    let mut data = vec![5,9,7,7,3,5,9,0,4,3,1,0,0,3,1,3,4,1,0,9,9,5,0,4,8,2,1,5,9,5,3,2,1,2,1,8,3,8,4,6,8,3,0,6,5,2,5,5,0,5,7,9,7,6,6,2,1,4,2,6,9,1,0,0,7,4,4,8,4,5,8,4,3,6,4,5,2,1,3,7,4,0,3,4,5,9,1,4,5,5,7,6,0,1,9,7,8,5,0,4,8,2,5,4,0,4,5,9,3,6,0,3,9,8,7,8,7,9,9,6,3,8,0,2,0,9,1,7,0,7,1,0,7,9,4,2,3,1,4,7,9,5,6,6,4,8,6,7,4,7,0,3,0,9,3,8,6,3,3,8,0,2,8,4,5,1,0,4,3,6,9,1,9,2,4,5,8,7,6,3,2,2,5,3,7,6,7,1,0,6,9,4,6,0,1,7,5,2,3,8,2,6,0,7,5,8,2,8,9,7,7,9,6,7,7,7,5,8,6,0,7,1,5,6,5,0,2,7,5,6,1,8,2,5,4,1,9,9,6,3,8,4,7,4,5,6,5,4,2,1,5,3,4,8,8,6,8,6,9,5,1,1,2,6,7,3,8,4,2,8,6,6,5,3,0,6,3,7,2,3,1,3,1,6,8,3,6,1,0,4,2,6,7,0,3,8,9,1,9,1,8,8,0,5,3,6,2,3,2,3,3,2,8,5,1,0,8,4,9,3,2,9,6,0,2,4,4,9,9,4,0,5,3,6,0,6,5,2,8,4,6,8,2,2,1,8,3,6,4,7,1,3,5,5,1,7,3,8,7,2,1,1,4,2,3,4,2,7,7,6,3,8,9,2,6,2,4,5,5,8,1,2,2,5,6,4,5,7,0,2,3,7,8,5,0,9,0,6,6,3,7,5,2,2,8,4,8,5,4,7,8,6,9,6,7,9,8,4,9,3,8,8,3,7,1,8,1,6,8,2,9,1,4,3,8,7,8,6,7,1,9,8,4,1,4,8,5,0,1,3,1,9,0,2,2,9,7,4,5,3,5,5,2,7,9,0,7,5,7,3,1,8,0,8,5,2,4,1,5,7,4,1,4,5,8,9,9,1,5,9,4,5,5,6,6,3,6,0,6,4,7,3,7,1,7,9,1,4,8,1,5,9,4,7,4,2,8,2,6,9,6,7,7,7,1,6,8,9,7,8,5,9,1,0,3,6,5,8,2,1,7,5,1,3,4,2,5,7,5,4,7,1,2,7,3,0,8,4,0,2,7,9,3,3,5,9,9,8,1,9,9,6,7,1,7,6,0,9,7,0,0,3,8,1,3,2,0,3,5,5,0,3,8,2,2,4,9,0,6,9,6,7,5,7,4,4,3,4,9,8,5,2,9,3,9,4,8,1,4,9,9,7,7,6,4,3,1,7,1,4,1,0,2,3,7,9,6,0,4,1,3,1,6,4,6,6,9,9,3,0];

    let mut comp = Calc::new(data.len()+2);
    comp.init(data.len() as i128);
    println!("start");

    for i in 0..100 {
        data = comp.comp(data);        
    }
    println!("{:?}",&data[0..8]);
    
}



/*
84970726
[8, 4, 9, 7, 0, 7, 2, 6, 4, 7, 1, 7, 2, 8, 7, 8, 6, 7, 8, 2, 6, 9, 1, 9, 1, 4, 4, 8, 7, 0, 0, 7, 6, 5, 2, 1, 9, 5, 9, 4, 0, 0, 9, 5, 0, 6, 2, 7, 5, 7, 3, 0, 1, 1, 7, 3, 3, 7, 5, 0, 6, 5, 8, 5, 8, 5, 7, 7, 1, 7, 0, 4, 1, 2, 2, 1, 3, 7, 8, 8, 3, 2, 0, 9, 5, 6, 9, 1, 3, 7, 7, 8, 5, 4, 4, 0, 1, 1, 7, 6, 1, 
8, 6, 4, 3, 7, 7, 0, 9, 0, 6, 6, 5, 6, 0, 0, 0, 3, 0, 2, 2, 9, 8, 0, 9, 3, 0, 4, 4, 8, 0, 4, 2, 0, 3, 5, 7, 1, 0, 0, 3, 7, 9, 1, 8, 9, 7, 5, 8, 2, 8, 8, 5, 8, 2, 2, 2, 1, 3, 9, 2, 7, 1, 2, 6, 4, 5, 7, 6, 2, 6, 0, 2, 2, 9, 5, 7, 7, 2, 0, 9, 7, 8, 0, 1, 8, 0, 7, 9, 1, 6, 0, 2, 7, 7, 7, 8, 5, 6, 3, 9, 9, 6, 3, 6, 3, 6, 6, 1, 9, 6, 2, 4, 1, 0, 4, 9, 5, 9, 9, 2, 7, 9, 6, 6, 5, 9, 8, 8, 3, 8, 3, 4, 9, 4, 1, 4, 1, 8, 7, 4, 3, 5, 4, 9, 9, 3, 6, 3, 8, 0, 9, 0, 
0, 9, 7, 6, 2, 7, 3, 9, 6, 8, 3, 0, 8, 0, 7, 9, 1, 4, 4, 3, 7, 6, 9, 0, 2, 5, 3, 5, 0, 3, 2, 1, 2, 1, 9, 2, 1, 6, 7, 3, 1, 6, 7, 7, 9, 0, 1, 1, 5, 5, 4, 7, 1, 5, 0, 5, 2, 9, 1, 5, 0, 4, 7, 7, 6, 0, 5, 6, 0, 2, 4, 1, 7, 9, 7, 1, 4, 4, 3, 4, 1, 3, 8, 8, 8, 0, 2, 9, 3, 7, 4, 0, 3, 4, 0, 4, 7, 2, 6, 7, 0, 8, 9, 4, 8, 5, 4, 2, 2, 8, 4, 9, 9, 9, 2, 7, 6, 4, 0, 4, 8, 5, 4, 0, 3, 8, 8, 2, 1, 7, 2, 9, 9, 4, 9, 8, 9, 7, 8, 1, 3, 3, 1, 3, 2, 4, 9, 8, 6, 3, 4, 1, 
5, 3, 3, 1, 0, 0, 9, 8, 8, 7, 3, 1, 8, 5, 9, 8, 2, 3, 0, 5, 3, 3, 4, 6, 8, 0, 4, 6, 2, 9, 7, 2, 4, 3, 2, 3, 2, 4, 8, 3, 2, 9, 2, 8, 6, 9, 3, 8, 7, 6, 1, 1, 4, 2, 0, 8, 4, 5, 4, 6, 7, 2, 7, 9, 0, 8, 9, 3, 2, 1, 3, 2, 7, 5, 5, 8, 3, 2, 8, 8, 0, 1, 1, 7, 8, 8, 4, 3, 5, 5, 5, 2, 6, 2, 7, 9, 1, 0, 5, 8, 0, 7, 0, 6, 0, 3, 0, 1, 4, 9, 3, 4, 2, 2, 6, 5, 6, 4, 5, 2, 4, 2, 2, 5, 0, 0, 1, 7, 5, 6, 9, 1, 0, 4, 1, 3, 1, 3, 2, 5, 3, 9, 5, 9, 2, 1, 6, 8, 5, 8, 0, 5, 
6, 9, 9, 0, 9, 5, 8, 5, 4, 6, 5, 9, 7, 5, 2, 2, 5, 3, 5, 3, 4, 5, 1, 6, 5, 9, 7, 5, 7, 8, 4, 2, 6, 6, 0, 2, 8, 3, 5, 2, 3, 0, 0, 8, 1, 3, 1, 0, 3, 3, 7, 6, 1, 6, 6, 4, 4, 3, 6, 7, 5, 5, 3, 8, 8, 3, 8, 9, 3, 6, 2, 1, 4, 1, 5, 2, 8, 2, 9, 6, 5, 9, 6, 8, 1, 1, 9, 1, 6, 9, 9, 3, 0]

*/