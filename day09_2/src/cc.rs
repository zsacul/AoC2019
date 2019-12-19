use std::collections::HashMap;

struct Comp{
  prog          : Vec<i128>,
  mem           : HashMap<i128,i128>,
  relative_code : i128,
  input         : i128,
  output        : Vec<i128>,
  pos           : i128,
}

impl Comp{
  fn new(prog:Vec<i128>,inp:i128)->Comp{
    Comp{
      prog          : prog.clone(),
      mem           : HashMap::new(),
      relative_code : 0,
      input         : inp,
      output        : vec![],
      pos           : 0,
    }
  }

  fn write(&mut self,id:i128,val:i128)
  {
      if id<0 { panic!("write id less then 0"); }
      let mem_id = id-self.prog.len() as i128;
      if mem_id>=0 { self.mem.insert(mem_id, val); }
              else { self.prog[id as usize] = val; }      
  }

  fn read(&self,id:i128)->i128
  {
      if id<0 { panic!("read id less then 0"); }
      let mem_id = id-self.prog.len() as i128;
      if mem_id>=0 { return *self.mem.get(&mem_id).unwrap_or(&0); }
      self.prog[id as usize]
  }

  fn get_params1(&self,n:i128)->(i128)
  {
    let m = self.read(n);
    let a = self.get_value((m/100 )%10,n+1);
    ( a )
  }
  
  fn get_params2(&self,n:i128)->(i128,i128)
  {
    let m = self.read(n);
    let a = self.get_value((m/100 )%10,n+1);
    let b = self.get_value((m/1000)%10,n+2);
    ( a,b )
  }
  
  fn get_params3(&self,n:i128)->(i128,i128,i128)
  {
    let m = self.read(n);
    let a = self.get_value( (m/100  )%10,n+1);
    let b = self.get_value( (m/1000 )%10,n+2);
    let c = self.get_adress((m/10000)%10,n+3);
    ( a,b,c )
  }

  fn get_value(&self,code:i128,n:i128)->i128
  {
    match code {
      0 => self.read(self.read(n)),
      1 => self.read(n),
      2 => self.read(self.read(n)+self.relative_code),
      _ => panic!("unknown val code:{} [{}/{}]",code,self.pos,self.prog.len()),
    }
  }

  fn get_adress(&self,code:i128,n:i128)->i128
  {
    match code {
      0 => self.read(n),
      2 => self.read(n)+self.relative_code,
      _ => panic!("unknown adress code:{} [{}/{}]",code,self.pos,self.prog.len()),
    }
  }

  fn calc(&mut self) -> Vec<i128>
  {
      loop {
          if self.pos>=self.prog.len() as i128 { return self.output.clone(); }

          let full_code = self.read(self.pos);
          let code = full_code%10;
  
          println!("i:{}",code);

          match code {

            
              1..=2 => { 
                      let (p1,p2,p3) = self.get_params3(self.pos);
                           if code==1 { self.write(p3, p1+p2); }
                      else if code==2 { self.write(p3, p1*p2); }
                      self.pos+=4;
                    },
              7..=8 => {
                      let (p1,p2,p3) = self.get_params3(self.pos);
                           if code==7 { self.write(p3, if p1< p2 {1} else {0}); }
                      else if code==8 { self.write(p3, if p1==p2 {1} else {0}); }
                      self.pos+=4;
              }
              9 => {
                      let p1 = self.get_params1(self.pos);
                      self.relative_code+=p1;
                     // println!("rel offset:{} rel_code:{}",p1,self.relative_code);
                      
                      self.pos+=2;
              }
              3 => { 
                
                      let p1 = self.get_adress((full_code/100)%10,self.pos+1); //self.get_params1(self.pos);//self.read(self.pos+1);
                      println!("pos:{} val:{}",self.pos+1,p1);
                      self.write(p1,self.input);
                      self.pos+=2;
              },
              4 => { 
                      let p1 = self.get_params1(self.pos);
                      //println!("out:{}",p1);
                      self.output.push(p1);
                      self.pos+=2;

                      if self.input>0 {
                        return self.output.clone();
                      }
              },
              5..=6 => {
                      let (p1,p2) = self.get_params2(self.pos);
                      self.pos+=3;
                           if code==5 { if p1!=0 {self.pos = p2;} }
                      else if code==6 { if p1==0 {self.pos = p2;} }
              }
              99 => { return self.output.clone(); },
               _ => { 
                      //println!("pos:{} p[pos]:{} code:{}",self.pos,self.prog[pos as usize],code);
                      panic!("error: wrong op code:{} {} [{}/{}]",full_code,code,self.pos,self.prog.len())
              },
          }
      }
  }
}



fn main() {
  //let ii:i128 = 1125899906842624;
  //println!("ii:{}",ii);

  //let p = vec![1102,34463338,34463338,63,1007,63,34463338,63,1005,63,53,1102,3,1,1000,109,988,209,12,9,1000,209,6,209,3,203,0,1008,1000,1,63,1005,63,65,1008,1000,2,63,1005,63,904,1008,1000,0,63,1005,63,58,4,25,104,0,99,4,0,104,0,99,4,17,104,0,99,0,0,1101,0,34,1006,1101,0,689,1022,1102,27,1,1018,1102,1,38,1010,1102,1,31,1012,1101,20,0,1015,1102,1,791,1026,1102,0,1,1020,1101,24,0,1000,1101,0,682,1023,1101,788,0,1027,1101,0,37,1005,1102,21,1,1011,1102,1,28,1002,1101,0,529,1024,1101,39,0,1017,1102,30,1,1013,1101,0,23,1003,1102,524,1,1025,1101,32,0,1007,1102,25,1,1008,1101,29,0,1001,1101,33,0,1016,1101,410,0,1029,1101,419,0,1028,1101,22,0,1014,1102,26,1,1019,1102,1,35,1009,1102,36,1,1004,1102,1,1,1021,109,11,2107,22,-8,63,1005,63,199,4,187,1106,0,203,1001,64,1,64,1002,64,2,64,109,2,21108,40,40,-2,1005,1011,221,4,209,1106,0,225,1001,64,1,64,1002,64,2,64,109,13,21102,41,1,-7,1008,1019,41,63,1005,63,251,4,231,1001,64,1,64,1106,0,251,1002,64,2,64,109,-19,1202,1,1,63,1008,63,26,63,1005,63,271,1105,1,277,4,257,1001,64,1,64,1002,64,2,64,109,7,2101,0,-6,63,1008,63,24,63,1005,63,297,1106,0,303,4,283,1001,64,1,64,1002,64,2,64,109,7,1205,-1,315,1105,1,321,4,309,1001,64,1,64,1002,64,2,64,109,-11,21107,42,41,0,1005,1010,341,1001,64,1,64,1106,0,343,4,327,1002,64,2,64,109,-8,1207,6,24,63,1005,63,363,1001,64,1,64,1106,0,365,4,349,1002,64,2,64,109,11,1206,8,381,1001,64,1,64,1106,0,383,4,371,1002,64,2,64,109,4,1205,4,401,4,389,1001,64,1,64,1105,1,401,1002,64,2,64,109,14,2106,0,-3,4,407,1001,64,1,64,1106,0,419,1002,64,2,64,109,-33,1202,3,1,63,1008,63,29,63,1005,63,445,4,425,1001,64,1,64,1105,1,445,1002,64,2,64,109,-5,2102,1,7,63,1008,63,25,63,1005,63,465,1105,1,471,4,451,1001,64,1,64,1002,64,2,64,109,11,21107,43,44,7,1005,1011,489,4,477,1105,1,493,1001,64,1,64,1002,64,2,64,109,-3,1208,8,35,63,1005,63,511,4,499,1105,1,515,1001,64,1,64,1002,64,2,64,109,25,2105,1,-2,4,521,1106,0,533,1001,64,1,64,1002,64,2,64,109,-8,21108,44,47,-8,1005,1010,549,1106,0,555,4,539,1001,64,1,64,1002,64,2,64,109,-19,1207,7,35,63,1005,63,577,4,561,1001,64,1,64,1106,0,577,1002,64,2,64,109,2,2108,32,0,63,1005,63,597,1001,64,1,64,1106,0,599,4,583,1002,64,2,64,109,13,2101,0,-7,63,1008,63,32,63,1005,63,625,4,605,1001,64,1,64,1105,1,625,1002,64,2,64,109,-13,2107,24,2,63,1005,63,645,1001,64,1,64,1106,0,647,4,631,1002,64,2,64,109,18,21101,45,0,-4,1008,1015,43,63,1005,63,671,1001,64,1,64,1105,1,673,4,653,1002,64,2,64,109,-6,2105,1,10,1001,64,1,64,1105,1,691,4,679,1002,64,2,64,109,1,1208,-6,23,63,1005,63,707,1105,1,713,4,697,1001,64,1,64,1002,64,2,64,109,-2,1206,8,731,4,719,1001,64,1,64,1106,0,731,1002,64,2,64,109,-7,21102,46,1,5,1008,1010,43,63,1005,63,751,1106,0,757,4,737,1001,64,1,64,1002,64,2,64,109,-9,2108,24,4,63,1005,63,779,4,763,1001,64,1,64,1106,0,779,1002,64,2,64,109,38,2106,0,-7,1106,0,797,4,785,1001,64,1,64,1002,64,2,64,109,-27,2102,1,-6,63,1008,63,29,63,1005,63,819,4,803,1105,1,823,1001,64,1,64,1002,64,2,64,109,1,21101,47,0,7,1008,1015,47,63,1005,63,845,4,829,1105,1,849,1001,64,1,64,1002,64,2,64,109,-11,1201,5,0,63,1008,63,31,63,1005,63,869,1106,0,875,4,855,1001,64,1,64,1002,64,2,64,109,5,1201,4,0,63,1008,63,34,63,1005,63,901,4,881,1001,64,1,64,1105,1,901,4,64,99,21102,27,1,1,21101,915,0,0,1105,1,922,21201,1,58905,1,204,1,99,109,3,1207,-2,3,63,1005,63,964,21201,-2,-1,1,21101,0,942,0,1106,0,922,22101,0,1,-1,21201,-2,-3,1,21102,1,957,0,1106,0,922,22201,1,-1,-2,1106,0,968,22102,1,-2,-2,109,-3,2106,0,0];

  let p = vec![1102,34463338,34463338,63,1007,63,34463338,63,1005,63,53,
               1102,3,1,1000,109,988,209,12,9,1000,
               209,6,209,3,203,0,1008,1000,1,63,
               1005,63,65,1008,1000,2,63,1005,63,904,
               1008,1000,0,63,1005,63,58,4,25,104,
               0,99,4,0,104,0,99,4,17,104,0,99,0,0,1101,0,34,1006,1101,0,689,1022,1102,27,1,1018,1102,1,38,1010,1102,1,31,1012,1101,20,0,1015,1102,1,791,1026,1102,0,1,1020,1101,24,0,1000,1101,0,682,1023,1101,788,0,1027,1101,0,37,1005,1102,21,1,1011,1102,1,28,1002,1101,0,529,1024,1101,39,0,1017,1102,30,1,1013,1101,0,23,1003,1102,524,1,1025,1101,32,0,1007,1102,25,1,1008,1101,29,0,1001,1101,33,0,1016,1101,410,0,1029,1101,419,0,1028,1101,22,0,1014,1102,26,1,1019,1102,1,35,1009,1102,36,1,1004,1102,1,1,1021,109,11,2107,22,-8,63,1005,63,199,4,187,1106,0,203,1001,64,1,64,1002,64,2,64,109,2,21108,40,40,-2,1005,1011,221,4,209,1106,0,225,1001,64,1,64,1002,64,2,64,109,13,21102,41,1,-7,1008,1019,41,63,1005,63,251,4,231,1001,64,1,64,1106,0,251,1002,64,2,64,109,-19,1202,1,1,63,1008,63,26,63,1005,63,271,1105,1,277,4,257,1001,64,1,64,1002,64,2,64,109,7,2101,0,-6,63,1008,63,24,63,1005,63,297,1106,0,303,4,283,1001,64,1,64,1002,64,2,64,109,7,1205,-1,315,1105,1,321,4,309,1001,64,1,64,1002,64,2,64,109,-11,21107,42,41,0,1005,1010,341,1001,64,1,64,1106,0,343,4,327,1002,64,2,64,109,-8,1207,6,24,63,1005,63,363,1001,64,1,64,1106,0,365,4,349,1002,64,2,64,109,11,1206,8,381,1001,64,1,64,1106,0,383,4,371,1002,64,2,64,109,4,1205,4,401,4,389,1001,64,1,64,1105,1,401,1002,64,2,64,109,14,2106,0,-3,4,407,1001,64,1,64,1106,0,419,1002,64,2,64,109,-33,1202,3,1,63,1008,63,29,63,1005,63,445,4,425,1001,64,1,64,1105,1,445,1002,64,2,64,109,-5,2102,1,7,63,1008,63,25,63,1005,63,465,1105,1,471,4,451,1001,64,1,64,1002,64,2,64,109,11,21107,43,44,7,1005,1011,489,4,477,1105,1,493,1001,64,1,64,1002,64,2,64,109,-3,1208,8,35,63,1005,63,511,4,499,1105,1,515,1001,64,1,64,1002,64,2,64,109,25,2105,1,-2,4,521,1106,0,533,1001,64,1,64,1002,64,2,64,109,-8,21108,44,47,-8,1005,1010,549,1106,0,555,4,539,1001,64,1,64,1002,64,2,64,109,-19,1207,7,35,63,1005,63,577,4,561,1001,64,1,64,1106,0,577,1002,64,2,64,109,2,2108,32,0,63,1005,63,597,1001,64,1,64,1106,0,599,4,583,1002,64,2,64,109,13,2101,0,-7,63,1008,63,32,63,1005,63,625,4,605,1001,64,1,64,1105,1,625,1002,64,2,64,109,-13,2107,24,2,63,1005,63,645,1001,64,1,64,1106,0,647,4,631,1002,64,2,64,109,18,21101,45,0,-4,1008,1015,43,63,1005,63,671,1001,64,1,64,1105,1,673,4,653,1002,64,2,64,109,-6,2105,1,10,1001,64,1,64,1105,1,691,4,679,1002,64,2,64,109,1,1208,-6,23,63,1005,63,707,1105,1,713,4,697,1001,64,1,64,1002,64,2,64,109,-2,1206,8,731,4,719,1001,64,1,64,1106,0,731,1002,64,2,64,109,-7,21102,46,1,5,1008,1010,43,63,1005,63,751,1106,0,757,4,737,1001,64,1,64,1002,64,2,64,109,-9,2108,24,4,63,1005,63,779,4,763,1001,64,1,64,1106,0,779,1002,64,2,64,109,38,2106,0,-7,1106,0,797,4,785,1001,64,1,64,1002,64,2,64,109,-27,2102,1,-6,63,1008,63,29,63,1005,63,819,4,803,1105,1,823,1001,64,1,64,1002,64,2,64,109,1,21101,47,0,7,1008,1015,47,63,1005,63,845,4,829,1105,1,849,1001,64,1,64,1002,64,2,64,109,-11,1201,5,0,63,1008,63,31,63,1005,63,869,1106,0,875,4,855,1001,64,1,64,1002,64,2,64,109,5,1201,4,0,63,1008,63,34,63,1005,63,901,4,881,1001,64,1,64,1105,1,901,4,64,99,21102,27,1,1,21101,915,0,0,1105,1,922,21201,1,58905,1,204,1,99,109,3,1207,-2,3,63,1005,63,964,21201,-2,-1,1,21101,0,942,0,1106,0,922,22101,0,1,-1,21201,-2,-3,1,21102,1,957,0,1106,0,922,22201,1,-1,-2,1106,0,968,22102,1,-2,-2,109,-3,2106,0,0];

  //let p = vec![109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99]; // same code on output
  //let p = vec![1102,34915192,34915192,7,4,7,99,0]; //big num
  //let p = vec![104,1125899906842624,99]; // large num in the middle
  
  let mut comp = Comp::new(p,2);  
  let output = comp.calc();

  println!("res:{:?}",output);  
}

#[test]
fn test1() {
  let p = vec![109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99]; // same code on output
  let mut comp = Comp::new(p.clone(),0);    
  assert_eq!(comp.calc(),p);
}

#[test]
fn test2(){
  let p = vec![1102,34915192,34915192,7,4,7,99,0]; 
  let mut comp = Comp::new(p,0);  
  let output = comp.calc();
  assert_eq!(output,vec![1219070632396864]);
}

#[test]
fn test3(){
  let p = vec![104,1125899906842624,99]; // large num in the middle
  let mut comp = Comp::new(p,0);  
  let output = comp.calc();
  assert_eq!(output,vec![1125899906842624]); 
}

#[test]
fn test4(){
  let p = vec![1102, 34915192, 34915192, 7, 4, 7, 99, 0];
    let mut comp = Comp::new(p,0);  
    let output = comp.calc();
    assert_eq!(output,vec![1219070632396864]);  
}

#[test]
fn test5(){
    let p = vec![104, 1125899906842624, 99];
    let mut comp = Comp::new(p,0);  
    let output = comp.calc();
    assert_eq!(output,vec![1125899906842624]);  
}

#[test]
fn test5_6() {
  let p = vec![3, 0, 4, 0, 99];
  let mut comp = Comp::new(p,22);  
  let output = comp.calc();
  assert_eq!(output,vec![22]);
}

#[test]
fn test5_7() {
  let p = vec![1101, 100, -1, 4, 0];
  let mut comp = Comp::new(p,0);  
  let output = comp.calc();  
  assert_eq!(output, vec![]);
} 


#[test]
fn test5_8() {
let p = vec![3,225,1,225,6,6,1100,1,238,225,104,0,1102,67,92,225,1101,14,84,225,1002,217,69,224,101,-5175,224,224,4,224,102,8,223,223,101,2,224,224,1,224,223,223,1,214,95,224,101,-127,224,224,4,224,102,8,223,223,101,3,224,224,1,223,224,223,1101,8,41,225,2,17,91,224,1001,224,-518,224,4,224,1002,223,8,223,101,2,224,224,1,223,224,223,1101,37,27,225,1101,61,11,225,101,44,66,224,101,-85,224,224,4,224,1002,223,8,223,101,6,224,224,1,224,223,223,1102,7,32,224,101,-224,224,224,4,224,102,8,223,223,1001,224,6,224,1,224,223,223,1001,14,82,224,101,-174,224,224,4,224,102,8,223,223,101,7,224,224,1,223,224,223,102,65,210,224,101,-5525,224,224,4,224,102,8,223,223,101,3,224,224,1,224,223,223,1101,81,9,224,101,-90,224,224,4,224,102,8,223,223,1001,224,3,224,1,224,223,223,1101,71,85,225,1102,61,66,225,1102,75,53,225,4,223,99,0,0,0,677,0,0,0,0,0,0,0,0,0,0,0,1105,0,99999,1105,227,247,1105,1,99999,1005,227,99999,1005,0,256,1105,1,99999,1106,227,99999,1106,0,265,1105,1,99999,1006,0,99999,1006,227,274,1105,1,99999,1105,1,280,1105,1,99999,1,225,225,225,1101,294,0,0,105,1,0,1105,1,99999,1106,0,300,1105,1,99999,1,225,225,225,1101,314,0,0,106,0,0,1105,1,99999,8,226,226,224,102,2,223,223,1005,224,329,1001,223,1,223,1108,677,677,224,1002,223,2,223,1006,224,344,101,1,223,223,1007,226,677,224,102,2,223,223,1005,224,359,101,1,223,223,1007,677,677,224,1002,223,2,223,1006,224,374,101,1,223,223,1108,677,226,224,1002,223,2,223,1005,224,389,1001,223,1,223,108,226,677,224,102,2,223,223,1006,224,404,101,1,223,223,1108,226,677,224,102,2,223,223,1005,224,419,101,1,223,223,1008,677,677,224,102,2,223,223,1005,224,434,101,1,223,223,7,677,226,224,1002,223,2,223,1005,224,449,101,1,223,223,1008,226,226,224,102,2,223,223,1005,224,464,1001,223,1,223,107,226,677,224,1002,223,2,223,1006,224,479,1001,223,1,223,107,677,677,224,102,2,223,223,1005,224,494,1001,223,1,223,1008,226,677,224,102,2,223,223,1006,224,509,1001,223,1,223,1107,677,226,224,102,2,223,223,1005,224,524,101,1,223,223,1007,226,226,224,1002,223,2,223,1006,224,539,1001,223,1,223,107,226,226,224,102,2,223,223,1006,224,554,101,1,223,223,108,677,677,224,1002,223,2,223,1006,224,569,1001,223,1,223,7,226,677,224,102,2,223,223,1006,224,584,1001,223,1,223,8,677,226,224,102,2,223,223,1005,224,599,101,1,223,223,1107,677,677,224,1002,223,2,223,1005,224,614,101,1,223,223,8,226,677,224,102,2,223,223,1005,224,629,1001,223,1,223,7,226,226,224,1002,223,2,223,1006,224,644,1001,223,1,223,108,226,226,224,1002,223,2,223,1006,224,659,101,1,223,223,1107,226,677,224,1002,223,2,223,1006,224,674,101,1,223,223,4,223,99,226];
  let mut comp = Comp::new(p,5);  
  let output = comp.calc();  
  assert_eq!(output, vec![8346937]);
} 

#[test]
fn test5_9() {
let p = vec![3,225,1,225,6,6,1100,1,238,225,104,0,1102,67,92,225,1101,14,84,225,1002,217,69,224,101,-5175,224,224,4,224,102,8,223,223,101,2,224,224,1,224,223,223,1,214,95,224,101,-127,224,224,4,224,102,8,223,223,101,3,224,224,1,223,224,223,1101,8,41,225,2,17,91,224,1001,224,-518,224,4,224,1002,223,8,223,101,2,224,224,1,223,224,223,1101,37,27,225,1101,61,11,225,101,44,66,224,101,-85,224,224,4,224,1002,223,8,223,101,6,224,224,1,224,223,223,1102,7,32,224,101,-224,224,224,4,224,102,8,223,223,1001,224,6,224,1,224,223,223,1001,14,82,224,101,-174,224,224,4,224,102,8,223,223,101,7,224,224,1,223,224,223,102,65,210,224,101,-5525,224,224,4,224,102,8,223,223,101,3,224,224,1,224,223,223,1101,81,9,224,101,-90,224,224,4,224,102,8,223,223,1001,224,3,224,1,224,223,223,1101,71,85,225,1102,61,66,225,1102,75,53,225,4,223,99,0,0,0,677,0,0,0,0,0,0,0,0,0,0,0,1105,0,99999,1105,227,247,1105,1,99999,1005,227,99999,1005,0,256,1105,1,99999,1106,227,99999,1106,0,265,1105,1,99999,1006,0,99999,1006,227,274,1105,1,99999,1105,1,280,1105,1,99999,1,225,225,225,1101,294,0,0,105,1,0,1105,1,99999,1106,0,300,1105,1,99999,1,225,225,225,1101,314,0,0,106,0,0,1105,1,99999,8,226,226,224,102,2,223,223,1005,224,329,1001,223,1,223,1108,677,677,224,1002,223,2,223,1006,224,344,101,1,223,223,1007,226,677,224,102,2,223,223,1005,224,359,101,1,223,223,1007,677,677,224,1002,223,2,223,1006,224,374,101,1,223,223,1108,677,226,224,1002,223,2,223,1005,224,389,1001,223,1,223,108,226,677,224,102,2,223,223,1006,224,404,101,1,223,223,1108,226,677,224,102,2,223,223,1005,224,419,101,1,223,223,1008,677,677,224,102,2,223,223,1005,224,434,101,1,223,223,7,677,226,224,1002,223,2,223,1005,224,449,101,1,223,223,1008,226,226,224,102,2,223,223,1005,224,464,1001,223,1,223,107,226,677,224,1002,223,2,223,1006,224,479,1001,223,1,223,107,677,677,224,102,2,223,223,1005,224,494,1001,223,1,223,1008,226,677,224,102,2,223,223,1006,224,509,1001,223,1,223,1107,677,226,224,102,2,223,223,1005,224,524,101,1,223,223,1007,226,226,224,1002,223,2,223,1006,224,539,1001,223,1,223,107,226,226,224,102,2,223,223,1006,224,554,101,1,223,223,108,677,677,224,1002,223,2,223,1006,224,569,1001,223,1,223,7,226,677,224,102,2,223,223,1006,224,584,1001,223,1,223,8,677,226,224,102,2,223,223,1005,224,599,101,1,223,223,1107,677,677,224,1002,223,2,223,1005,224,614,101,1,223,223,8,226,677,224,102,2,223,223,1005,224,629,1001,223,1,223,7,226,226,224,1002,223,2,223,1006,224,644,1001,223,1,223,108,226,226,224,1002,223,2,223,1006,224,659,101,1,223,223,1107,226,677,224,1002,223,2,223,1006,224,674,101,1,223,223,4,223,99,226];
  let mut comp = Comp::new(p,1);
  let output = comp.calc();  
  assert_eq!(output[output.len()-1], 5074395);
} 

  

  

/*
#[test]
fn test2() {
  let p = vec![1101, 100, -1, 4, 0];
  let output = calc(p,0);
  assert_eq!(output, vec![]);
} 
*/