struct Calc {
    bases : Vec<i8>,
    max_n : usize,
}

impl Calc
{
    fn new(l:usize)->Calc{        
        Calc{
            bases : vec![0i8],
            max_n : l,
        }
    }

    fn init(&mut self,i:i128){        
        for num in 1..=i {
            //self.bases.push(self.get_base(num));
            //if num%100000==0 { println!("iii:{}/6500000",num); }
        }
    }        

    fn get_base(&mut self,i:i128)->Vec<(usize,usize,i8)>
    {
        let mut res = vec![];
        //self.bases.clear();
        if i as usize>self.max_n { return vec![(0,0,0)]; }
        let mut id : usize = 0;


        if i-1>0 {
            id+=(i-1) as usize;
            if id>self.max_n { return res; }
            //res.push((id,id+i as usize,1));   
        }

        loop {
            res.push((id,id+(i-1) as usize,1));     id+=i as usize;
            if id>self.max_n { return res; }        id+=i as usize;            
            res.push((id,id+(i-1) as usize,-1));    id+=i as usize;
            if id>self.max_n { return res; }        id+=i as usize;    
        }
        res


        //for _n in 0..i { self.bases.push( 0); if self.bases.len()>self.max_n { return; } }
        //for _n in 0..i { self.bases.push( 1); if self.bases.len()>self.max_n { return; } }
        //for _n in 0..i { self.bases.push( 0); if self.bases.len()>self.max_n { return; } }
        //for _n in 0..i { self.bases.push(-1); if self.bases.len()>self.max_n { return; } }
        //res
    }

    fn get_pattern(&self,i:i128,n:i128)->i8
    {
        let l = self.bases.len() as i128;
        let id = ( ((n+1).abs())%l ) as usize;
    
        return self.bases[id];
    }

    //let v:Vec<i128> = vec![1,2,3,4,6];
    //1..2 = 5

    fn get_sum_val(&self,sum_vec:&Vec<i128>,i:i128)->i128
    {
        if i<0 || i>=sum_vec.len() as i128 { return 0; }
        sum_vec[i as usize]
    }

    fn sum(&self,sum_vec:&Vec<i128>,f:i128,t:i128)->i128{
        
        let l = sum_vec.len() as i128;
        if f>=l as i128 || t<0 { return 0; }

        let to   = std::cmp::min(l as i128-1,t);
        let from = std::cmp::max(f,0);

        self.get_sum_val(sum_vec,to) - self.get_sum_val(sum_vec,from-1)
    }

    fn compute_sum_vector(&self,data:&Vec<i128>)->Vec<i128>{
        let mut sum_vec = vec![];
        let mut id = 0;
        let mut sum = 0;
        for d in data {    
            sum+=d;
            sum_vec.push(sum); 
            id+=1;
        }
        sum_vec
    }
    
    fn comp(&mut self,data:Vec<i128>)->Vec<i128>
    {
        let mut res : Vec<i128> = vec![];
        let mut i=1;    
        let sum_vec = self.compute_sum_vector(&data);

        for d in &data {    
            let mut sum = 0;
            //let mut id = 0;
            let spans = self.get_base(i);
    
            for j in &spans {
                let (f,t,v) = j;
                sum+=(*v as i128)*self.sum(&sum_vec,*f as i128,*t as i128);
                //sum+=*j*(self.get_pattern(i, id) as i128);
                //id+=1;
            }
            res.push(sum.abs()%10i128);
            i+=1;
        }
        res
    }
}

fn main() {

    
    let mut data_org = vec![5,9,7,7,3,5,9,0,4,3,1,0,0,3,1,3,4,1,0,9,9,5,0,4,8,2,1,5,9,5,3,2,1,2,1,8,3,8,4,6,8,3,0,6,5,2,5,5,0,5,7,9,7,6,6,2,1,4,2,6,9,1,0,0,7,4,4,8,4,5,8,4,3,6,4,5,2,1,3,7,4,0,3,4,5,9,1,4,5,5,7,6,0,1,9,7,8,5,0,4,8,2,5,4,0,4,5,9,3,6,0,3,9,8,7,8,7,9,9,6,3,8,0,2,0,9,1,7,0,7,1,0,7,9,4,2,3,1,4,7,9,5,6,6,4,8,6,7,4,7,0,3,0,9,3,8,6,3,3,8,0,2,8,4,5,1,0,4,3,6,9,1,9,2,4,5,8,7,6,3,2,2,5,3,7,6,7,1,0,6,9,4,6,0,1,7,5,2,3,8,2,6,0,7,5,8,2,8,9,7,7,9,6,7,7,7,5,8,6,0,7,1,5,6,5,0,2,7,5,6,1,8,2,5,4,1,9,9,6,3,8,4,7,4,5,6,5,4,2,1,5,3,4,8,8,6,8,6,9,5,1,1,2,6,7,3,8,4,2,8,6,6,5,3,0,6,3,7,2,3,1,3,1,6,8,3,6,1,0,4,2,6,7,0,3,8,9,1,9,1,8,8,0,5,3,6,2,3,2,3,3,2,8,5,1,0,8,4,9,3,2,9,6,0,2,4,4,9,9,4,0,5,3,6,0,6,5,2,8,4,6,8,2,2,1,8,3,6,4,7,1,3,5,5,1,7,3,8,7,2,1,1,4,2,3,4,2,7,7,6,3,8,9,2,6,2,4,5,5,8,1,2,2,5,6,4,5,7,0,2,3,7,8,5,0,9,0,6,6,3,7,5,2,2,8,4,8,5,4,7,8,6,9,6,7,9,8,4,9,3,8,8,3,7,1,8,1,6,8,2,9,1,4,3,8,7,8,6,7,1,9,8,4,1,4,8,5,0,1,3,1,9,0,2,2,9,7,4,5,3,5,5,2,7,9,0,7,5,7,3,1,8,0,8,5,2,4,1,5,7,4,1,4,5,8,9,9,1,5,9,4,5,5,6,6,3,6,0,6,4,7,3,7,1,7,9,1,4,8,1,5,9,4,7,4,2,8,2,6,9,6,7,7,7,1,6,8,9,7,8,5,9,1,0,3,6,5,8,2,1,7,5,1,3,4,2,5,7,5,4,7,1,2,7,3,0,8,4,0,2,7,9,3,3,5,9,9,8,1,9,9,6,7,1,7,6,0,9,7,0,0,3,8,1,3,2,0,3,5,5,0,3,8,2,2,4,9,0,6,9,6,7,5,7,4,4,3,4,9,8,5,2,9,3,9,4,8,1,4,9,9,7,7,6,4,3,1,7,1,4,1,0,2,3,7,9,6,0,4,1,3,1,6,4,6,6,9,9,3,0];
    let mut data     = vec![];
    //0000
    for i in 0..10000 {
        data.append(&mut data_org.clone());
    }

    println!("go {}",data.len() );
    let mut comp = Calc::new(data.len()+2);

//    println!("1:{:?}", comp.get_base(1));
//    println!("2:{:?}", comp.get_base(2));
//    println!("3:{:?}", comp.get_base(3));
//    return;

    comp.init(data.len() as i128);
    println!("start");

    for i in 0..100 {
        println!("i:{}",i);
        data = comp.comp(data);        
    }
    //println!("{:?}",&data[..8]);
    println!("{:?}",&data[5977359..5977359+8]);
    
}

#[test]
fn sum_tests()
{
    let v:Vec<i128> = vec![1,2,3,4,6];
    let mut c = Calc::new(0);
    let sum_vec = c.compute_sum_vector(&v);
    assert_eq!(c.sum(&sum_vec, 1,  1), 2);
    assert_eq!(c.sum(&sum_vec, 0,  0), 1);
    assert_eq!(c.sum(&sum_vec,-5,  0), 1);
    assert_eq!(c.sum(&sum_vec, 1,  2), 5);
    assert_eq!(c.sum(&sum_vec, 5,  6), 0);
    assert_eq!(c.sum(&sum_vec,-5,600),16);
    assert_eq!(c.sum(&sum_vec, 1,  3), 9);
}

/*
84970726
[8, 4, 9, 7, 0, 7, 2, 6, 4, 7, 1, 7, 2, 8, 7, 8, 6, 7, 8, 2, 6, 9, 1, 9, 1, 4, 4, 8, 7, 0, 0, 7, 6, 5, 2, 1, 9, 5, 9, 4, 0, 0, 9, 5, 0, 6, 2, 7, 5, 7, 3, 0, 1, 1, 7, 3, 3, 7, 5, 0, 6, 5, 8, 5, 8, 5, 7, 7, 1, 7, 0, 4, 1, 2, 2, 1, 3, 7, 8, 8, 3, 2, 0, 9, 5, 6, 9, 1, 3, 7, 7, 8, 5, 4, 4, 0, 1, 1, 7, 6, 1, 
8, 6, 4, 3, 7, 7, 0, 9, 0, 6, 6, 5, 6, 0, 0, 0, 3, 0, 2, 2, 9, 8, 0, 9, 3, 0, 4, 4, 8, 0, 4, 2, 0, 3, 5, 7, 1, 0, 0, 3, 7, 9, 1, 8, 9, 7, 5, 8, 2, 8, 8, 5, 8, 2, 2, 2, 1, 3, 9, 2, 7, 1, 2, 6, 4, 5, 7, 6, 2, 6, 0, 2, 2, 9, 5, 7, 7, 2, 0, 9, 7, 8, 0, 1, 8, 0, 7, 9, 1, 6, 0, 2, 7, 7, 7, 8, 5, 6, 3, 9, 9, 6, 3, 6, 3, 6, 6, 1, 9, 6, 2, 4, 1, 0, 4, 9, 5, 9, 9, 2, 7, 9, 6, 6, 5, 9, 8, 8, 3, 8, 3, 4, 9, 4, 1, 4, 1, 8, 7, 4, 3, 5, 4, 9, 9, 3, 6, 3, 8, 0, 9, 0, 
0, 9, 7, 6, 2, 7, 3, 9, 6, 8, 3, 0, 8, 0, 7, 9, 1, 4, 4, 3, 7, 6, 9, 0, 2, 5, 3, 5, 0, 3, 2, 1, 2, 1, 9, 2, 1, 6, 7, 3, 1, 6, 7, 7, 9, 0, 1, 1, 5, 5, 4, 7, 1, 5, 0, 5, 2, 9, 1, 5, 0, 4, 7, 7, 6, 0, 5, 6, 0, 2, 4, 1, 7, 9, 7, 1, 4, 4, 3, 4, 1, 3, 8, 8, 8, 0, 2, 9, 3, 7, 4, 0, 3, 4, 0, 4, 7, 2, 6, 7, 0, 8, 9, 4, 8, 5, 4, 2, 2, 8, 4, 9, 9, 9, 2, 7, 6, 4, 0, 4, 8, 5, 4, 0, 3, 8, 8, 2, 1, 7, 2, 9, 9, 4, 9, 8, 9, 7, 8, 1, 3, 3, 1, 3, 2, 4, 9, 8, 6, 3, 4, 1, 
5, 3, 3, 1, 0, 0, 9, 8, 8, 7, 3, 1, 8, 5, 9, 8, 2, 3, 0, 5, 3, 3, 4, 6, 8, 0, 4, 6, 2, 9, 7, 2, 4, 3, 2, 3, 2, 4, 8, 3, 2, 9, 2, 8, 6, 9, 3, 8, 7, 6, 1, 1, 4, 2, 0, 8, 4, 5, 4, 6, 7, 2, 7, 9, 0, 8, 9, 3, 2, 1, 3, 2, 7, 5, 5, 8, 3, 2, 8, 8, 0, 1, 1, 7, 8, 8, 4, 3, 5, 5, 5, 2, 6, 2, 7, 9, 1, 0, 5, 8, 0, 7, 0, 6, 0, 3, 0, 1, 4, 9, 3, 4, 2, 2, 6, 5, 6, 4, 5, 2, 4, 2, 2, 5, 0, 0, 1, 7, 5, 6, 9, 1, 0, 4, 1, 3, 1, 3, 2, 5, 3, 9, 5, 9, 2, 1, 6, 8, 5, 8, 0, 5, 
6, 9, 9, 0, 9, 5, 8, 5, 4, 6, 5, 9, 7, 5, 2, 2, 5, 3, 5, 3, 4, 5, 1, 6, 5, 9, 7, 5, 7, 8, 4, 2, 6, 6, 0, 2, 8, 3, 5, 2, 3, 0, 0, 8, 1, 3, 1, 0, 3, 3, 7, 6, 1, 6, 6, 4, 4, 3, 6, 7, 5, 5, 3, 8, 8, 3, 8, 9, 3, 6, 2, 1, 4, 1, 5, 2, 8, 2, 9, 6, 5, 9, 6, 8, 1, 1, 9, 1, 6, 9, 9, 3, 0]

*/