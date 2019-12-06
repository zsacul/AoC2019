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

fn calc(prog:Vec<i64>,inp:i64) -> Vec<i64>
{
    let mut p      = prog.clone();
    let mut pos    = 0;
    let mut output = vec![];
    
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
                    p[p1] = inp; 
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
            99 => { return output; },
             _ => { 
                    println!("pos:{} p[pos]:{} code:{}",pos,p[pos],code );
                    panic!("error: wrong op code")
             },
        }
    }

    output
}

fn main() {
  let vv = vec![3,225,1,225,6,6,1100,1,238,225,104,0,1102,67,92,225,1101,14,84,225,1002,217,69,224,101,-5175,224,224,4,224,102,8,223,223,101,2,224,224,1,224,223,223,1,214,95,224,101,-127,224,224,4,224,102,8,223,223,101,3,224,224,1,223,224,223,1101,8,41,225,2,17,91,224,1001,224,-518,224,4,224,1002,223,8,223,101,2,224,224,1,223,224,223,1101,37,27,225,1101,61,11,225,101,44,66,224,101,-85,224,224,4,224,1002,223,8,223,101,6,224,224,1,224,223,223,1102,7,32,224,101,-224,224,224,4,224,102,8,223,223,1001,224,6,224,1,224,223,223,1001,14,82,224,101,-174,224,224,4,224,102,8,223,223,101,7,224,224,1,223,224,223,102,65,210,224,101,-5525,224,224,4,224,102,8,223,223,101,3,224,224,1,224,223,223,1101,81,9,224,101,-90,224,224,4,224,102,8,223,223,1001,224,3,224,1,224,223,223,1101,71,85,225,1102,61,66,225,1102,75,53,225,4,223,99,0,0,0,677,0,0,0,0,0,0,0,0,0,0,0,1105,0,99999,1105,227,247,1105,1,99999,1005,227,99999,1005,0,256,1105,1,99999,1106,227,99999,1106,0,265,1105,1,99999,1006,0,99999,1006,227,274,1105,1,99999,1105,1,280,1105,1,99999,1,225,225,225,1101,294,0,0,105,1,0,1105,1,99999,1106,0,300,1105,1,99999,1,225,225,225,1101,314,0,0,106,0,0,1105,1,99999,8,226,226,224,102,2,223,223,1005,224,329,1001,223,1,223,1108,677,677,224,1002,223,2,223,1006,224,344,101,1,223,223,1007,226,677,224,102,2,223,223,1005,224,359,101,1,223,223,1007,677,677,224,1002,223,2,223,1006,224,374,101,1,223,223,1108,677,226,224,1002,223,2,223,1005,224,389,1001,223,1,223,108,226,677,224,102,2,223,223,1006,224,404,101,1,223,223,1108,226,677,224,102,2,223,223,1005,224,419,101,1,223,223,1008,677,677,224,102,2,223,223,1005,224,434,101,1,223,223,7,677,226,224,1002,223,2,223,1005,224,449,101,1,223,223,1008,226,226,224,102,2,223,223,1005,224,464,1001,223,1,223,107,226,677,224,1002,223,2,223,1006,224,479,1001,223,1,223,107,677,677,224,102,2,223,223,1005,224,494,1001,223,1,223,1008,226,677,224,102,2,223,223,1006,224,509,1001,223,1,223,1107,677,226,224,102,2,223,223,1005,224,524,101,1,223,223,1007,226,226,224,1002,223,2,223,1006,224,539,1001,223,1,223,107,226,226,224,102,2,223,223,1006,224,554,101,1,223,223,108,677,677,224,1002,223,2,223,1006,224,569,1001,223,1,223,7,226,677,224,102,2,223,223,1006,224,584,1001,223,1,223,8,677,226,224,102,2,223,223,1005,224,599,101,1,223,223,1107,677,677,224,1002,223,2,223,1005,224,614,101,1,223,223,8,226,677,224,102,2,223,223,1005,224,629,1001,223,1,223,7,226,226,224,1002,223,2,223,1006,224,644,1001,223,1,223,108,226,226,224,1002,223,2,223,1006,224,659,101,1,223,223,1107,226,677,224,1002,223,2,223,1006,224,674,101,1,223,223,4,223,99,226];
  println!("res:{:?}",calc(vv,5));
}

#[test]
fn test1() {
  let p = vec![3, 0, 4, 0, 99];
  let output = calc(p,22);
  assert_eq!(output,vec![22]);
}

#[test]
fn test2() {
  let p = vec![1101, 100, -1, 4, 0];
  let output = calc(p,0);
  assert_eq!(output, vec![]);
}