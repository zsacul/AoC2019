fn calc(prog:Vec<i64>) -> i64
{
    let mut p    = prog.clone();
    let mut last = 0;    
    let mut pos  = 0;
    
    while pos<p.len() {

        match p[pos] {
             1 => { 
                    let p1 = p[pos+1] as usize;
                    let p2 = p[pos+2] as usize;
                    let p3 = p[pos+3] as usize;
                    p[p3] = p[p1]+p[p2]; pos+=4;
                    last = p[p3]
                  },
             2 => { 
                    let p1 = p[pos+1] as usize;
                    let p2 = p[pos+2] as usize;
                    let p3 = p[pos+3] as usize;
                    p[p3] = p[p1]*p[p2]; pos+=4;
                    last = p[p3]
                  },
            99 => { return last;         },
             _ => { panic!("error: wrong op code") },
        }
    }
    
    last
}

fn main() {
//let r = vec![1, 0,0,3,1,1,2,3,1,3,4,3,1,5,0,3,2,9,1,19,1,9,19,23,1,23,5,27,2,27,10,31,1,6,31,35,1,6,35,39,2,9,39,43,1,6,43,47,1,47,5,51,1,51,13,55,1,55,13,59,1,59,5,63,2,63,6,67,1,5,67,71,1,71,13,75,1,10,75,79,2,79,6,83,2,9,83,87,1,5,87,91,1,91,5,95,2,9,95,99,1,6,99,103,1,9,103,107,2,9,107,111,1,111,6,115,2,9,115,119,1,119,6,123,1,123,9,127,2,127,13,131,1,131,9,135,1,10,135,139,2,139,10,143,1,143,5,147,2,147,6,151,1,151,5,155,1,2,155,159,1,6,159,0,99,2,0,14,0];
  let r = vec![1,12,2,3,1,1,2,3,1,3,4,3,1,5,0,3,2,9,1,19,1,9,19,23,1,23,5,27,2,27,10,31,1,6,31,35,1,6,35,39,2,9,39,43,1,6,43,47,1,47,5,51,1,51,13,55,1,55,13,59,1,59,5,63,2,63,6,67,1,5,67,71,1,71,13,75,1,10,75,79,2,79,6,83,2,9,83,87,1,5,87,91,1,91,5,95,2,9,95,99,1,6,99,103,1,9,103,107,2,9,107,111,1,111,6,115,2,9,115,119,1,119,6,123,1,123,9,127,2,127,13,131,1,131,9,135,1,10,135,139,2,139,10,143,1,143,5,147,2,147,6,151,1,151,5,155,1,2,155,159,1,6,159,0,99,2,0,14,0];
  println!("res:{}",calc(r));
}

#[test]
fn test1() {   assert_eq!(calc(vec![1,0,0,0,99]), 2); }

#[test]
fn test2() {   assert_eq!(calc(vec![2,3,0,3,99]), 6); }
#[test]
fn test3() {   assert_eq!(calc(vec![2,4,4,5,99,0]), 9801); }
