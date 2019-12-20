extern crate rand;

use rand::Rng;
use std::collections::HashMap;
use std::collections::VecDeque;

struct Comp{
    prog          : Vec<i128>,  
    relative_code : i128,
    input         : i128,
    output        : Vec<i128>,
    pos           : i128,
  }
  
  impl Comp{
    fn new(prog:Vec<i128>,inp:i128)->Comp{
      Comp{
        prog          : prog.clone(),
        relative_code : 0,
        input         : inp,
        output        : vec![],
        pos           : 0,
      }
    }

    fn set_input(&mut self,i:i128){
        self.input = i;
    }
    
    fn clear_output(&mut self){
        self.output.clear();
    }    
  
    fn write(&mut self,id:i128,val:i128)
    {
        if id<0 { panic!("write id less then 0"); }
  
        if id >= self.prog.len() as i128 {
          self.prog.resize(id as usize+1, 0);
        }
  
        self.prog[id as usize] = val;
  
        //println!("write:{}={}",id,val);
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
    
            //println!("pos:{} val:{} i:{}",self.pos,self.prog[self.pos as usize],code);
  
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
                     // println!("rel offset:{} rel_code:{}",p1,self.relative_code);                     
                        self.pos+=2;
                }
                3 => {     
                  let p1 = self.get_adress((full_code/100)%10,self.pos+1); //
                        self.write(p1,self.input);
                        self.pos+=2;
                },
                4 => { 
                        let p1 = self.get_value((full_code/100 )%10,self.pos+1);
                        //println!("out:{}",p1);
                        self.output.push(p1);
                        self.pos+=2;
                        if self.output.len()>=0 {
                            return self.output.clone();
                          //  return vec![output[0],output[1],output[2],]
                        }
                },
                5..=6 => {
                        let (p1,p2) = self.get_params2(self.pos);
                        self.pos+=3;
  
                        //println!("pos:{}",self.pos);
                        //println!("code:{} p1:{} p2:{}",code,p1,p2);
                             if code==5 { if p1!=0 {self.pos = p2;} }
                        else if code==6 { if p1==0 {self.pos = p2;} }
                        //println!("pos:{}",self.pos);
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

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
struct Vec2{
    x: i64,
    y: i64,
}

impl Vec2 {
    fn new(fx:i64,fy:i64)->Vec2 {
        Vec2{
            x:fx,
            y:fy,
        }
    }
}

struct Field{
    field : HashMap<Vec2,char>
}

impl Field{
    fn new()->Self{
        Field{
            field : HashMap::new(),
        }        
    }

    fn get_char(&self,n:i32)->char{
         match n  {
           -1 => '?',
            0 => '■',
            1 => 'x',
            2 => 'o',
            _ => panic!("err code {}",n),
        }
    }

    fn paint(&mut self,pos:Vec2,col:i32)
    {
        if self.field.get(&pos)==None {
            self.field.insert(pos,self.get_char(col));
        }
    }

    fn print(&self,pos:Vec2,dx:i64,dy:i64)
    {
      for y in -dy..dy {
        for x in -dx..dx {
          let mut c = self.field.get(&Vec2::new(x,y)).unwrap_or(&'?');
          if pos.x==x && pos.y==y {
              c = &'+';
          }
          print!("{}",c);
        }
        println!("");
      }
    }

    fn is_ok(&self,p:Vec2)->bool{
        let c0 = self.get_char(0);
        let v:&char = &self.field.get(&p).unwrap_or(&c0);
        v==&self.get_char(1) || v==&self.get_char(2)
    }

    fn shortest(&self, s:Vec2)->i64
    {
        let mut positions = VecDeque::new();
        let mut was: HashMap::<Vec2,bool> = HashMap::new();
        positions.push_back((s,0));

        while positions.len()>0 {
            let (pos,l) = positions.pop_front().unwrap();
            
            if self.field.get(&pos).unwrap_or(&self.get_char(0))==&self.get_char(2) {
                return l;
            }

            let v = was.get(&pos).unwrap_or(&false);

            if !v {
                was.insert(pos,true);
                let ld = Vec2::new(pos.x-1,pos.y  );
                let rd = Vec2::new(pos.x+1,pos.y  );
                let ud = Vec2::new(pos.x  ,pos.y-1);
                let dd = Vec2::new(pos.x  ,pos.y+1);

                if !was.get(&ld).unwrap_or(&false) && self.is_ok(ld) { positions.push_back((ld,l+1)); }
                if !was.get(&rd).unwrap_or(&false) && self.is_ok(rd) { positions.push_back((rd,l+1)); }
                if !was.get(&ud).unwrap_or(&false) && self.is_ok(ud) { positions.push_back((ud,l+1)); }
                if !was.get(&dd).unwrap_or(&false) && self.is_ok(dd) { positions.push_back((dd,l+1)); }
            }
        }
        
        -1
    }
}

fn main() {
    let data = vec![3,1033,1008,1033,1,1032,1005,1032,31,1008,1033,2,1032,1005,1032,58,1008,1033,3,1032,1005,1032,81,1008,1033,4,1032,1005,1032,104,99,1001,1034,0,1039,1002,1036,1,1041,1001,1035,-1,1040,1008,1038,0,1043,102,-1,1043,1032,1,1037,1032,1042,1106,0,124,1001,1034,0,1039,102,1,1036,1041,1001,1035,1,1040,1008,1038,0,1043,1,1037,1038,1042,1105,1,124,1001,1034,-1,1039,1008,1036,0,1041,102,1,1035,1040,1002,1038,1,1043,101,0,1037,1042,1105,1,124,1001,1034,1,1039,1008,1036,0,1041,102,1,1035,1040,1001,1038,0,1043,101,0,1037,1042,1006,1039,217,1006,1040,217,1008,1039,40,1032,1005,1032,217,1008,1040,40,1032,1005,1032,217,1008,1039,33,1032,1006,1032,165,1008,1040,33,1032,1006,1032,165,1101,0,2,1044,1106,0,224,2,1041,1043,1032,1006,1032,179,1102,1,1,1044,1105,1,224,1,1041,1043,1032,1006,1032,217,1,1042,1043,1032,1001,1032,-1,1032,1002,1032,39,1032,1,1032,1039,1032,101,-1,1032,1032,101,252,1032,211,1007,0,42,1044,1106,0,224,1102,0,1,1044,1106,0,224,1006,1044,247,1001,1039,0,1034,1001,1040,0,1035,1001,1041,0,1036,1001,1043,0,1038,102,1,1042,1037,4,1044,1106,0,0,6,28,51,33,63,27,52,11,53,13,96,8,87,11,23,65,43,11,13,9,37,66,68,40,19,41,6,90,28,19,38,86,38,22,7,44,36,23,17,1,16,54,36,74,14,79,2,14,83,10,38,19,62,66,27,56,33,52,47,98,41,39,77,83,48,29,49,15,80,59,9,72,79,55,24,66,50,24,27,56,37,41,13,72,35,13,64,70,5,66,78,37,78,24,43,93,22,41,30,58,14,45,6,27,44,48,40,52,31,12,3,72,7,14,59,35,17,63,34,79,93,17,54,98,35,21,91,25,32,77,10,31,88,17,35,79,96,11,83,15,48,9,19,64,24,65,86,32,71,22,88,55,31,18,88,68,34,40,94,1,71,24,40,44,28,43,4,98,21,80,17,53,2,94,6,43,59,23,66,63,12,30,45,39,93,41,85,43,51,18,99,59,86,40,36,26,94,33,41,28,66,79,81,11,61,46,32,72,71,47,39,22,69,60,36,50,12,44,28,41,79,17,6,74,8,56,39,33,67,23,20,51,12,7,26,57,1,92,80,11,52,19,5,54,13,41,56,37,22,57,43,18,97,27,83,30,3,77,85,66,64,17,99,27,25,95,40,81,97,13,35,46,14,25,63,36,72,87,20,96,29,2,69,90,27,27,91,52,14,14,73,55,4,73,19,85,39,84,23,23,90,40,5,88,53,77,8,92,11,82,66,6,27,84,53,38,93,34,37,58,20,43,25,73,78,30,17,92,54,38,26,67,16,30,28,79,77,26,3,15,82,59,34,34,18,44,34,33,83,35,90,31,58,44,16,18,65,8,70,90,32,46,21,41,54,39,43,93,23,99,11,43,50,98,33,34,53,54,53,16,39,88,53,36,69,85,26,44,38,62,98,6,79,26,35,49,67,22,11,74,35,80,4,50,18,54,4,10,4,58,4,46,20,15,77,73,11,41,58,85,39,87,37,73,36,36,67,28,12,17,34,53,38,89,96,34,39,67,64,33,81,37,74,88,20,84,94,53,39,57,73,13,76,1,35,14,73,29,29,23,73,52,16,85,87,33,48,13,2,93,78,7,17,60,49,13,36,89,40,25,44,55,26,81,37,31,84,31,62,2,66,77,23,88,11,81,9,63,46,19,35,54,17,85,24,1,86,28,72,1,1,61,27,38,81,8,67,82,3,11,77,35,62,83,20,28,61,37,37,92,22,72,76,37,52,17,62,68,38,53,2,57,82,67,25,11,59,3,49,97,1,40,91,75,7,85,98,33,90,1,37,57,14,34,67,65,20,85,10,18,86,20,52,84,24,20,70,10,64,16,64,2,15,85,36,28,7,87,47,44,9,29,54,83,28,37,81,68,18,12,80,26,98,97,25,86,69,39,70,22,23,72,15,56,94,27,14,13,8,50,73,90,24,95,14,41,57,22,67,25,80,46,39,84,80,19,22,63,53,45,62,21,84,36,69,41,44,96,38,92,21,23,64,35,11,75,57,88,6,7,90,10,36,19,68,78,23,62,34,49,4,80,38,2,70,48,39,55,20,22,39,8,90,64,38,39,47,41,63,72,5,10,72,88,35,50,5,66,30,80,74,23,97,39,98,19,17,85,38,34,62,37,25,58,15,93,37,13,71,72,72,4,84,40,92,61,88,9,7,62,59,87,17,36,39,43,21,11,16,58,16,58,20,66,18,83,33,66,62,90,32,74,15,58,62,43,16,66,22,90,2,68,30,54,18,59,22,50,12,60,35,66,77,51,36,64,89,82,21,85,0,0,21,21,1,10,1,0,0,0,0,0,0];

    let mut comp = Comp::new(data,0);  
    let start_pos = Vec2::new(0,0);
    let mut pos = start_pos.clone();
    let mut end_pos : Option<Vec2> = None;

    let mut rng = rand::thread_rng();
    let mut f = Field::new();

    for io in 0..=4000000
    {
        let rand_i: u32 = rng.gen();
        let move_dir = ((rand_i%4)+1) as i128;

        comp.set_input(move_dir);
        let output = comp.calc();
        
        //north (1), south (2), west (3), and east (4)
        let dir = match move_dir {
            1 => Vec2::new( 0,-1),
            2 => Vec2::new( 0, 1),
            3 => Vec2::new(-1, 0),
            4 => Vec2::new( 1, 0),
            _ => panic!("wrong dir"),
        };

        let new_pos = Vec2::new(pos.x+dir.x,pos.y+dir.y);
   
        match output[0] {
            0 =>                  f.paint(new_pos,0),
            1 => { pos = new_pos; f.paint(pos    ,1); },
            2 => { pos = new_pos; end_pos = Some(pos); f.paint(pos,2); },
            _ => panic!("wrong code"),
        }

        comp.clear_output();
        if io%100000==0 {
            f.print(pos,25,25);
            println!("end:{:?}",end_pos);
        }
    }
    f.print(start_pos,25,25);
        
    println!("fin score:{}",f.shortest(start_pos));
}
