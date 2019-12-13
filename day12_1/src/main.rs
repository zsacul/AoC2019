#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
struct Vec3{
    x: i64,
    y: i64,
    z: i64,
}

impl Vec3 {
    fn new(fx:i64,fy:i64,fz:i64)->Vec3
    {
        Vec3{
            x:fx,
            y:fy,
            z:fz,
        }
    }

    fn len_no_square(&self,pos_x:i64,pos_y:i64)->i64
    {
        (self.x - pos_x)*(self.x - pos_x) + (self.y - pos_y)*(self.y - pos_y)
    }
}

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
struct Planet{
    position : Vec3,
    velocity : Vec3,    
}

impl Planet{
    fn new(pos:Vec3)->Planet
    {
        Planet{
            position : pos,
            velocity : Vec3::new(0,0,0),
        }
    }

    fn apply_gravity(&mut self,other:&Planet)
    {
        if self.position.x>other.position.x { self.velocity.x-=1; }
        if self.position.y>other.position.y { self.velocity.y-=1; }
        if self.position.z>other.position.z { self.velocity.z-=1; }
        if self.position.x<other.position.x { self.velocity.x+=1; }
        if self.position.y<other.position.y { self.velocity.y+=1; }
        if self.position.z<other.position.z { self.velocity.z+=1; }
    }

    fn move_planet(&mut self){
        self.position.x+=self.velocity.x;
        self.position.y+=self.velocity.y;
        self.position.z+=self.velocity.z;
    }

    fn get_potential_energy(&self)->i64{
        self.position.x.abs()+
        self.position.y.abs()+
        self.position.z.abs()
    }

    fn get_kinetic_energy(&self)->i64{
        self.velocity.x.abs()+
        self.velocity.y.abs()+
        self.velocity.z.abs()
    }

    fn get_energy(&self)->i64{
        self.get_potential_energy() * self.get_kinetic_energy()
    }
}

struct System {
    planets   : Vec<Planet>,
    iteration : u64,
}

impl System{
    fn new()->System{
        System{
            planets : vec![],
            iteration : 0,
        }
    }

    fn add_planet(&mut self,x:i64,y:i64,z:i64){
        self.planets.push(Planet::new(Vec3::new(x,y,z)));
    }

    fn apply_gravity(&mut self){        
        for a in 0..self.planets.len() {
            for b in 0..self.planets.len() {
                let bb = self.planets[b].clone();
                if a!=b {
                    self.planets[a].apply_gravity(&bb);
                }
            }            
        }
    }

    fn move_planets(&mut self){
        for a in self.planets.iter_mut() { a.move_planet(); }
    }

    fn compute(&mut self,num_iteration:i64){
        for i in 0..num_iteration {
            self.apply_gravity();
            self.move_planets();   
        }
    }
    
    fn get_energy(&self)->i64{
        let mut res = 0;
        for a in &self.planets { res+=a.get_energy(); }
        res
    }
}

fn main() {
    let mut system = System::new();

    system.add_planet(12,  0,-15);
    system.add_planet(-8, -5,-10);
    system.add_planet( 7,-17,  1);
    system.add_planet( 2,-11, -6);
    system.compute(1000);

    println!("res:{}",system.get_energy());
}

#[test]
fn test1()
{
    let mut system = System::new();

    system.add_planet(-1,   0,  2);
    system.add_planet( 2, -10, -7);
    system.add_planet( 4,  -8,  8);
    system.add_planet( 3,   5, -1);
    system.compute(10);

    assert_eq!(system.get_energy(),179);
}
