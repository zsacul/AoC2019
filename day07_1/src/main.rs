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

fn calc(prog:&Vec<i64>,ampv:i64,inp:i64) -> Vec<i64>
{
    let mut p      = prog.clone();
    let mut pos    = 0;
    let mut output = vec![];
    let mut inputs =  VecDeque::new();

    inputs.push_back(ampv);
    inputs.push_back(inp);
    
    while pos<p.len() {
        let code = p[pos]%100;

        match code {
             1..=2 => { 
                    let (p1,p2,p3) = get_params3(&p,pos);       
                         if code==1 { p[p3] = p1+p2; }
                    else if code==2 { p[p3] = p1*p2; }
                    pos+=4;
                  },
             7..=8 => {
                    let (p1,p2,p3) = get_params3(&p,pos);       
                         if code==7 { p[p3] = if p1< p2 {1} else {0}; }
                    else if code==8 { p[p3] = if p1==p2 {1} else {0}; }
                    pos+=4;
              }
             3 => { 
                    let p1 = p[pos+1] as usize;
                    p[p1] = inputs.pop_front().unwrap(); 
                    pos+=2;
             },
             4 => { 
                    let p1 = p[pos+1] as usize;
                    output.push(p[p1]); 
                    pos+=2;
             },
             5..=6 => {
                    let (p1,p2) = get_params2(&p,pos);
                    pos+=3;
                    if code==5 { if p1!=0 {pos = p2 as usize; } }
               else if code==6 { if p1==0 {pos = p2 as usize; } }
             }
            99 => { 
                    return output; },
             _ => { 
                    println!("pos:{} p[pos]:{} code:{}",pos,p[pos],code );
                    panic!("error: wrong op code")
             },
        }
    }

    output
}

fn run_amp(prog:&Vec<i64>,amp:&Vec<i64>)->i64 {
    let mut inp = 0;
    for v in amp {
      inp = calc(prog,*v,inp)[0];
    }
    inp
}

fn run_all(prog:&Vec<i64>)->i64 {
    let mut res = 0;
    for a in 0..5 {
        for b in 0..5 {
            for c in 0..5 {
                for d in 0..5 {
                    for e in 0..5 {
                        let amp = vec![a,b,c,d,e];
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
    let amp:Vec<i64> = vec![4,3,2,1,0];
    let prg = vec![3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0];
    let r = run_amp(&prg,&amp);
    assert_eq!(r,43210);
}

#[test]
fn test3() {
    let amp:Vec<i64> = vec![0,1,2,3,4];
    let prg = vec![3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0];
    let r = run_amp(&prg,&amp);
    assert_eq!(r,54321);
}

#[test]
fn test4() {
    let amp:Vec<i64> = vec![1,0,4,3,2];
    let prg = vec![3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0];
    let r = run_amp(&prg,&amp);
    assert_eq!(r,65210);
}

#[test]
fn test2() {
    let prg:Vec<i64> = vec![3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0];
    let r = run_all(&prg);
    assert_eq!(r,43210);
} 