fn get_params1(prog:&Vec<i64>,n:i64,relative_code:i64)->(i64)
{
    let m = read(prog,n);
    let a = match (m/100)%10
    {
        0 => read(prog,read(prog,n+1)),
        2 => read(prog,read(prog,n+1)+relative_code),
        _ => read(prog,n+1),
    };
    (a)  
}

fn get_params2(prog:&Vec<i64>,n:i64,relative_code:i64)->(i64,i64)
{
  let m = read(prog,n);
  let a = match (m/100)%10
  {
      0 => read(prog,read(prog,n+1)),
      2 => read(prog,read(prog,n+1)+relative_code),
      _ => read(prog,n+1),
  };
  
  let b = match (m/1000)%10 {
      0 => read(prog,read(prog,n+2)),
      2 => read(prog,read(prog,n+2)+relative_code),
      _ => read(prog,n+2),
  };
  ( a,b )
}

fn get_params3(prog:&Vec<i64>,n:i64,relative_code:i64 )->(i64,i64,i64)
{
  let r = get_params2(prog,n,relative_code);
  (r.0,r.1,read(prog,n+3))
}

fn write(prog:&mut Vec<i64>,id:i64,val:i64)
{
    if id<0 {panic!("write id less then 0");}
    prog[id as usize] = val;
}

fn read(prog:&Vec<i64>,id:i64)->i64
{
    if id<0 {panic!("read id less then 0");}
    prog[id as usize]
}

fn calc(prog:Vec<i64>,inp:i64) -> Vec<i64>
{
    let mut p      = prog.clone();
    let mut pos    = 0i64;
    let mut output = vec![];
    let mut relative_code : i64 = 0;
    
    while pos<p.len() as i64 {
        let code = read(&p,pos)%100;

        match code {
            1..=2 => { 
                    let (p1,p2,p3) = get_params3(&p,pos,relative_code);
                         if code==1 { write(&mut p,p3, p1+p2); }
                    else if code==2 { write(&mut p,p3, p1*p2); }
                    pos+=4;
                  },
            7..=8 => {
                    let (p1,p2,p3) = get_params3(&p,pos,relative_code);
                         if code==7 { write(&mut p,p3, if p1< p2 {1} else {0}); }
                    else if code==8 { write(&mut p,p3, if p1==p2 {1} else {0}); }
                    pos+=4;
            }
            9 => {
                    let p1 = get_params1(&p,pos,relative_code);
                    relative_code+=p1;
                    println!("rel offset:{} rel_code:{}",p1,relative_code);
                    
                    pos+=2;
            }
            3 => { 
                    let p1 = read(&p,pos+1);
                    write(&mut p,p1,inp);
                    //p[p1] = inp; 
                    pos+=2;
            },
            4 => { 
                    let p1 = read(&p,pos+1);
                    output.push( read(&p,p1)); 
                    pos+=2;
            },
            5..=6 => {
                    let (p1,p2) = get_params2(&p,pos,relative_code);
                    pos+=3;
                    if code==5 { if p1!=0 {pos = p2; } }
               else if code==6 { if p1==0 {pos = p2; } }
            }
            99 => { return output; },
             _ => { 
                    println!("pos:{} p[pos]:{} code:{}",pos,p[pos as usize],code);
                    panic!("error: wrong op code")
            },
        }
    }

    output
}

fn main() {
    let ii:i64 = 1125899906842624;
    println!("ii:{}",ii);

  let p = vec![109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99];
  let p2 = p.clone();
  let output = calc(p,1);


  println!("res:{:?}",output);
}

#[test]
fn test1() {
  let p = vec![109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99];
  let output = calc(p,22);
  assert_eq!(output,p);
}
/*
#[test]
fn test2() {
  let p = vec![1101, 100, -1, 4, 0];
  let output = calc(p,0);
  assert_eq!(output, vec![]);
} 
*/