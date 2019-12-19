use std::collections::HashMap;

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
                      if self.output.len()==2 { return self.output.clone(); }
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

enum Dir{
    Up,
    Right,
    Down,
    Left,
}

struct Robot{
    dir     : Dir,
    field   : HashMap<Vec2,i128>,
    visited : HashMap<Vec2,i128>,
    pos     : Vec2,
}

impl Robot{
    fn new()->Self{
        Robot{
            dir     : Dir::Up,
            field   : HashMap::new(),
            visited : HashMap::new(),
            pos     : Vec2::new(0,0),
        }
    }
    fn rotate(&mut self,rotation_dir:i128)
    {
        if rotation_dir==1 
        {
            self.dir = match self.dir{
                Dir::Down  => Dir::Left,
                Dir::Left  => Dir::Up,
                Dir::Up    => Dir::Right,
                Dir::Right => Dir::Down,
            }
        }
        else
        {
            self.dir = match self.dir{
                Dir::Down  => Dir::Right,
                Dir::Right => Dir::Up,
                Dir::Up    => Dir::Left,
                Dir::Left  => Dir::Down,
            }
        }
    } 
      
    fn paint(&mut self,color:i128)
    {
        self.field.insert(self.pos,color);
        self.visited.insert(self.pos,1);
    }

    fn get_color_of_field(&self)->i128
    {
        *self.field.get(&self.pos).unwrap_or(&0)
    }

    fn move_forward(&mut self){
        let d = match self.dir {
            Dir::Down  => Vec2::new( 0, 1),
            Dir::Up    => Vec2::new( 0,-1),
            Dir::Right => Vec2::new( 1, 0),
            Dir::Left  => Vec2::new(-1, 0),
        };
        self.pos.x+=d.x;
        self.pos.y+=d.y;
    }

    fn work(&mut self,color:i128,rotation_dir:i128)
    {
        self.paint(color);
        self.rotate(rotation_dir);        
        self.move_forward();        
    }

    fn print(&self,dx:i64,dy:i64)
    {
      for y in -dy..dy {
        for x in -dx..dx {
          let c = self.field.get(&Vec2::new(x,y)).unwrap_or(&0);
          if c==&1 { print!("■") }
              else { print!(" ") }
        }
        println!("");
      }
    }

}

fn main()
{
    let p = vec![3,8,1005,8,311,1106,0,11,0,0,0,104,1,104,0,3,8,102,-1,8,10,1001,10,1,10,4,10,1008,8,0,10,4,10,1002,8,1,29,3,8,102,-1,8,10,1001,10,1,10,4,10,108,0,8,10,4,10,101,0,8,50,1,2,19,10,1006,0,23,1,103,14,10,1,1106,15,10,3,8,1002,8,-1,10,1001,10,1,10,4,10,1008,8,1,10,4,10,102,1,8,88,1006,0,59,3,8,1002,8,-1,10,101,1,10,10,4,10,1008,8,1,10,4,10,1002,8,1,113,2,101,12,10,2,1001,0,10,2,1006,14,10,3,8,1002,8,-1,10,101,1,10,10,4,10,108,0,8,10,4,10,102,1,8,146,1,1106,11,10,1006,0,2,1,9,8,10,3,8,1002,8,-1,10,1001,10,1,10,4,10,1008,8,1,10,4,10,101,0,8,180,1,6,13,10,1,1102,15,10,2,7,1,10,3,8,1002,8,-1,10,1001,10,1,10,4,10,108,0,8,10,4,10,1002,8,1,213,1006,0,74,2,1005,9,10,3,8,1002,8,-1,10,101,1,10,10,4,10,1008,8,0,10,4,10,1002,8,1,243,3,8,1002,8,-1,10,101,1,10,10,4,10,108,1,8,10,4,10,101,0,8,264,2,104,8,10,3,8,1002,8,-1,10,1001,10,1,10,4,10,108,1,8,10,4,10,1001,8,0,290,101,1,9,9,1007,9,952,10,1005,10,15,99,109,633,104,0,104,1,21101,387512640296,0,1,21101,0,328,0,1106,0,432,21102,1,665749660564,1,21101,339,0,0,1106,0,432,3,10,104,0,104,1,3,10,104,0,104,0,3,10,104,0,104,1,3,10,104,0,104,1,3,10,104,0,104,0,3,10,104,0,104,1,21102,179318226984,1,1,21101,386,0,0,1105,1,432,21101,46266346499,0,1,21101,0,397,0,1105,1,432,3,10,104,0,104,0,3,10,104,0,104,0,21102,709580555028,1,1,21102,420,1,0,1106,0,432,21102,1,988220642068,1,21101,0,431,0,1106,0,432,99,109,2,21202,-1,1,1,21101,40,0,2,21102,1,463,3,21102,1,453,0,1106,0,496,109,-2,2106,0,0,0,1,0,0,1,109,2,3,10,204,-1,1001,458,459,474,4,0,1001,458,1,458,108,4,458,10,1006,10,490,1102,0,1,458,109,-2,2105,1,0,0,109,4,2102,1,-1,495,1207,-3,0,10,1006,10,513,21101,0,0,-3,21201,-3,0,1,22101,0,-2,2,21102,1,1,3,21101,532,0,0,1106,0,537,109,-4,2106,0,0,109,5,1207,-3,1,10,1006,10,560,2207,-4,-2,10,1006,10,560,22102,1,-4,-4,1105,1,628,21201,-4,0,1,21201,-3,-1,2,21202,-2,2,3,21102,1,579,0,1105,1,537,22101,0,1,-4,21101,1,0,-1,2207,-4,-2,10,1006,10,598,21101,0,0,-1,22202,-2,-1,-2,2107,0,-3,10,1006,10,620,22101,0,-1,1,21102,620,1,0,106,0,495,21202,-2,-1,-2,22201,-4,-2,-4,109,-5,2105,1,0];  
    let mut robot = Robot::new();
    let mut comp  =  Comp::new(p,1);

    robot.paint(1);

    loop {
        comp.clear_output();
        let output = comp.calc();    
        if output.len()<2 { break; }
        
        robot.work(output[0],output[1]);
        let inp = robot.get_color_of_field();
        comp.set_input(inp);
    }

    robot.print(50,6);
}

