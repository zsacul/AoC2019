use std::collections::HashMap;
use std::collections::VecDeque;

struct Comp{
    prog          : Vec<i128>,  
    relative_code : i128,
    input         : VecDeque<i128>,
    output        : Vec<i128>,
    pos           : i128,
  }
  
  impl Comp{
    fn new(prog:Vec<i128>)->Comp{
      Comp{
        prog          : prog.clone(),
        relative_code : 0,
        input         : VecDeque::new(),
        output        : vec![],
        pos           : 0,
      }
    }

    fn set_input(&mut self,i:i128){
        self.input.push_back(i);
    }
    
    fn clear_output(&mut self){
      self.output.clear();
    }    

    fn clear_input(&mut self){
      self.input.clear();
    }    

    fn reset(&mut self){
      self.clear_output();
      self.clear_input();
      self.pos = 0;  
    }

    fn write(&mut self,id:i128,val:i128)
    {
        if id<0 { panic!("write id less then 0"); }
  
        if id >= self.prog.len() as i128 {
          self.prog.resize(id as usize+1, 0);
        }
  
        self.prog[id as usize] = val;
    }
  
    fn read(&self,id:i128)->i128
    {
        if id<0 { panic!("read id less then 0"); }
        *self.prog.get(id as usize).unwrap_or(&0)
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
        2 => self.read(self.read(n) + self.relative_code),
        _ => panic!("unknown val code:{} [{}/{}]",code,self.pos,self.prog.len()),
      }
    }
  
    fn get_adress(&self,code:i128,n:i128)->i128
    {
      match code {
        0 => self.read(n),
        2 => self.read(n) + self.relative_code,
        _ => panic!("unknown adress code:{} [{}/{}]",code,self.pos,self.prog.len()),
      }
    }
  
    fn calc(&mut self) -> Vec<i128>
    {
        loop {
            if self.pos>=self.prog.len() as i128 { return self.output.clone(); }
  
            let full_code = self.read(self.pos);
            let code = full_code%100;
    
            match code {          
                1..=2 => { 
                        let (p1,p2,p3) = self.get_params3(self.pos);
                             if code==1 { self.write(p3, p1+p2); }
                        else if code==2 { self.write(p3, p1*p2); }
                        self.pos+=4;
                      },
                7..=8 => {
                        let (p1,p2,p3) = self.get_params3(self.pos);
                             if code==7 { if p1< p2 { self.write(p3, 1); } 
                                               else { self.write(p3, 0); } }
                        else if code==8 { if p1==p2 { self.write(p3, 1); } 
                                               else { self.write(p3, 0); } }
                        self.pos+=4;
                }
                9 => {
                        let p1 = self.get_params1(self.pos);
                        self.relative_code+=p1;    
                        self.pos+=2;
                }
                3 => {     
                        let p1 = self.get_adress((full_code/100)%10,self.pos+1); //
                        let i = self.input.pop_front().unwrap();
                        self.write(p1,i);
                        self.pos+=2;
                },
                4 => { 
                        let p1 = self.get_value((full_code/100 )%10,self.pos+1);

                        self.output.push(p1);
                        self.pos+=2;
                        if self.output.len()>=0 {
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
                        panic!("error: wrong op code:{} {} [{}/{}]",full_code,code,self.pos,self.prog.len())
                },
            }
        }
    }
  }

fn get_val(data:&Vec<i128>,x:i128,y:i128)->bool{
  let mut comp = Comp::new(data.clone());
  comp.set_input(x);
  comp.set_input(y);
  comp.calc()[0]==1
}

fn main() {
    let data = vec![109,424,203,1,21101,0,11,0,1105,1,282,21101,18,0,0,1105,1,259,2101,0,1,221,203,1,21101,31,0,0,1105,1,282,21102,38,1,0,1106,0,259,20101,0,23,2,21201,1,0,3,21101,1,0,1,21101,0,57,0,1105,1,303,2101,0,1,222,21001,221,0,3,21001,221,0,2,21101,259,0,1,21101,0,80,0,1106,0,225,21102,117,1,2,21102,91,1,0,1105,1,303,2101,0,1,223,20102,1,222,4,21102,1,259,3,21101,0,225,2,21102,1,225,1,21101,0,118,0,1105,1,225,21001,222,0,3,21102,1,77,2,21102,133,1,0,1105,1,303,21202,1,-1,1,22001,223,1,1,21102,1,148,0,1105,1,259,2102,1,1,223,21002,221,1,4,20101,0,222,3,21102,20,1,2,1001,132,-2,224,1002,224,2,224,1001,224,3,224,1002,132,-1,132,1,224,132,224,21001,224,1,1,21102,195,1,0,106,0,109,20207,1,223,2,20102,1,23,1,21101,0,-1,3,21101,0,214,0,1106,0,303,22101,1,1,1,204,1,99,0,0,0,0,109,5,1202,-4,1,249,21201,-3,0,1,21201,-2,0,2,22101,0,-1,3,21102,250,1,0,1106,0,225,22101,0,1,-4,109,-5,2105,1,0,109,3,22107,0,-2,-1,21202,-1,2,-1,21201,-1,-1,-1,22202,-1,-2,-2,109,-3,2106,0,0,109,3,21207,-2,0,-1,1206,-1,294,104,0,99,21202,-2,1,-2,109,-3,2105,1,0,109,5,22207,-3,-4,-1,1206,-1,346,22201,-4,-3,-4,21202,-3,-1,-1,22201,-4,-1,2,21202,2,-1,-1,22201,-4,-1,1,21202,-2,1,3,21102,1,343,0,1105,1,303,1106,0,415,22207,-2,-3,-1,1206,-1,387,22201,-3,-2,-3,21202,-2,-1,-1,22201,-3,-1,3,21202,3,-1,-1,22201,-3,-1,2,21202,-4,1,1,21102,384,1,0,1106,0,303,1105,1,415,21202,-4,-1,-4,22201,-4,-3,-4,22202,-3,-2,-2,22202,-2,-4,-4,22202,-3,-2,-3,21202,-4,-1,-2,22201,-3,-2,1,22101,0,1,-4,109,-5,2105,1,0];    
    let mut minv = std::i128::MAX;
    let mut ans  = 0;
    let size = 100-1;

    for y in 0..1500{
      for x in 0..1500{
        if get_val(&data,x     ,y     ) &&
           get_val(&data,x+size,y     ) &&
           get_val(&data,x     ,y+size) &&
           get_val(&data,x+size,y+size)          
        { 
          let len = x*x + y*y;
          if len<minv {
            minv = len;
            ans = x*10000+y;
            println!("{} {}",x,y);
          }       
        }
      }
      if y%100==0 { println!("y:{}",y); }
    }      

    println!("answer {}", ans);
}
