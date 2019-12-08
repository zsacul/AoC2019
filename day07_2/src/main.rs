use std::collections::VecDeque;

fn get_params2(prog:&Vec<i64>,n:usize)->(i64,i64)
{
  let m = prog[n];
  let a_posmode = (m/100)%10==0; 
  let b_posmode = (m/1000)%10==0; 
  ( if a_posmode {prog[prog[n+1] as usize]} else {prog[n+1]},
    if b_posmode {prog[prog[n+2] as usize]} else {prog[n+2]} )
}

fn get_params3(prog:&Vec<i64>,n:usize)->(i64,i64,usize)
{
  let r = get_params2(prog,n);
  (r.0,r.1,prog[n+3] as usize)
}

struct Comp
{
    prog  : Vec<i64>,
    input : VecDeque<i64>,
    output: Vec<i64>,
    pos   : usize,
}

impl Comp{    
    fn new(program:&Vec<i64>) -> Comp {
        Comp {
          prog  : program.clone(),
          input : VecDeque::new(),
          output: vec![],
          pos   : 0,
        }
    }    

    fn clear(&mut self)
    {
        self.input.clear();
        self.output.clear();
        self.pos = 0;
    }

    fn add_data(&mut self,data:i64)
    {
        self.input.push_back(data);
    }

    fn calc(&mut self) -> (i64,bool)
    {  
        while self.pos<self.prog.len() {
            let code = self.prog[self.pos]%100;

            match code {
                1..=2 => { 
                        let (p1,p2,p3) = get_params3(&self.prog,self.pos);
                             if code==1 { self.prog[p3] = p1+p2; }
                        else if code==2 { self.prog[p3] = p1*p2; }
                        self.pos+=4;
                    },
                7..=8 => {
                        let (p1,p2,p3) = get_params3(&self.prog,self.pos);
                             if code==7 { self.prog[p3] = if p1< p2 {1} else {0}; }
                        else if code==8 { self.prog[p3] = if p1==p2 {1} else {0}; }
                        self.pos+=4;
                }
                3 => { 
                        let p1 = self.prog[self.pos+1] as usize;
                        self.prog[p1] = self.input.pop_front().unwrap(); 
                        self.pos+=2;
                },
                4 => { 
                        let p1 = self.prog[self.pos+1] as usize;
                        self.output.push(self.prog[p1]); 
                        self.pos+=2;
                        return (self.output[self.output.len()-1],false);
                },
                5..=6 => {
                        let (p1,p2) = get_params2(&self.prog,self.pos);
                        self.pos+=3;
                        if code==5 { if p1!=0 {self.pos = p2 as usize; } }
                   else if code==6 { if p1==0 {self.pos = p2 as usize; } }
                }
                99 => { 
                        self.pos=0;
                        return (self.output[self.output.len()-1],true); },
                _ => { 
                        println!("pos:{} p[pos]:{} code:{}",self.pos,self.prog[self.pos],code );
                        panic!("error: wrong op code")
                },
            }
        }
        
        panic!("something went wrong");
    }
}

fn run_amp(prog:&Vec<i64>,amp:&Vec<i64>)->i64 {
    let mut c1 = Comp::new(prog); c1.add_data(amp[0]);
    let mut c2 = Comp::new(prog); c2.add_data(amp[1]);
    let mut c3 = Comp::new(prog); c3.add_data(amp[2]);
    let mut c4 = Comp::new(prog); c4.add_data(amp[3]);
    let mut c5 = Comp::new(prog); c5.add_data(amp[4]);

    let mut inp5 = (0i64,false);
    
    while !inp5.1  {
                              c1.add_data(inp5.0); 
        let inp1 = c1.calc(); c2.add_data(inp1.0);
        let inp2 = c2.calc(); c3.add_data(inp2.0);
        let inp3 = c3.calc(); c4.add_data(inp3.0);
        let inp4 = c4.calc(); c5.add_data(inp4.0);
            inp5 = c5.calc();
    }
    inp5.0
}

fn run_all(prog:&Vec<i64>)->i64 {
    let mut res = 0;
    for a in 0..5 {
        for b in 0..5 {
            for c in 0..5 {
                for d in 0..5 {
                    for e in 0..5 {
                        let amp = vec![a+5,b+5,c+5,d+5,e+5];
                        let mut ampm = amp.clone();
                        ampm.sort();
                        ampm.dedup();

                        if ampm.len()==5
                        {
                            res = std::cmp::max(res,run_amp(prog,&amp));
                        }
                    }
                }
            }
        }
    }
    res
}

fn main() {
    
  let vv = vec![3,8,1001,8,10,8,105,1,0,0,21,46,63,76,97,118,199,280,361,442,99999,3,9,102,4,9,9,101,2,9,9,1002,9,5,9,101,4,9,9,102,2,9,9,4,9,99,3,9,101,5,9,9,102,3,9,9,101,3,9,9,4,9,99,3,9,1001,9,2,9,102,3,9,9,4,9,99,3,9,1002,9,5,9,101,4,9,9,1002,9,3,9,101,2,9,9,4,9,99,3,9,1002,9,5,9,101,3,9,9,1002,9,5,9,1001,9,5,9,4,9,99,3,9,102,2,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,1002,9,2,9,4,9,3,9,1001,9,1,9,4,9,3,9,101,1,9,9,4,9,3,9,1001,9,1,9,4,9,3,9,1002,9,2,9,4,9,3,9,1001,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,102,2,9,9,4,9,99,3,9,1002,9,2,9,4,9,3,9,101,2,9,9,4,9,3,9,1001,9,1,9,4,9,3,9,101,2,9,9,4,9,3,9,102,2,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,1002,9,2,9,4,9,3,9,101,2,9,9,4,9,3,9,1001,9,1,9,4,9,3,9,102,2,9,9,4,9,99,3,9,102,2,9,9,4,9,3,9,102,2,9,9,4,9,3,9,102,2,9,9,4,9,3,9,1001,9,2,9,4,9,3,9,101,1,9,9,4,9,3,9,101,2,9,9,4,9,3,9,102,2,9,9,4,9,3,9,102,2,9,9,4,9,3,9,1001,9,2,9,4,9,3,9,101,1,9,9,4,9,99,3,9,1002,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,1001,9,2,9,4,9,3,9,101,1,9,9,4,9,3,9,1001,9,1,9,4,9,3,9,1001,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,101,1,9,9,4,9,3,9,1001,9,1,9,4,9,3,9,101,2,9,9,4,9,99,3,9,101,1,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,1002,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,101,1,9,9,4,9,3,9,101,2,9,9,4,9,3,9,1001,9,2,9,4,9,3,9,102,2,9,9,4,9,3,9,101,2,9,9,4,9,99];
  println!("res:{:?}",run_all(&vv));
}

#[test]
fn test1() {
    let amp:Vec<i64> = vec![9,8,7,6,5];
    let prg = vec![3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5];
    let r = run_amp(&prg,&amp);
    assert_eq!(r,139629729);
}

#[test]
fn test2() { 
    let amp:Vec<i64> = vec![9,7,8,5,6];
    let prg = vec![3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,   53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10];
    let r = run_amp(&prg,&amp);
    assert_eq!(r,18216);
}
