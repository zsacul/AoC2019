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

    fn is_zero(&self)->bool{
        self.x==0 && self.y==0 && self.z==0
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

    fn apply_gravity(&mut self,other:&Vec3)
    {
             if self.position.x>other.x { self.velocity.x-=1; }
        else if self.position.x<other.x { self.velocity.x+=1; }
             if self.position.y>other.y { self.velocity.y-=1; }
        else if self.position.y<other.y { self.velocity.y+=1; }
             if self.position.z>other.z { self.velocity.z-=1; }
        else if self.position.z<other.z { self.velocity.z+=1; }
    }

    fn get_min_dist(&mut self,other:&Planet)->i64
    {
        let mut             res = (self.position.x-other.position.x).abs();
        res = std::cmp::min(res , (self.position.y-other.position.y).abs());
              std::cmp::min(res , (self.position.z-other.position.z).abs())
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
                let bb = self.planets[b].position.clone();
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

    fn compute_loop(&mut self)->i64{
        let mut iterations = 0;
        let planets_copy = self.planets.clone();

        loop {
            self.apply_gravity();
            self.move_planets();   
            iterations+=1;
            if self.planets==planets_copy { return iterations; }
            if iterations%10000000==0 { println!("i:{}",iterations/10000000); }
        }
    }
    
    fn get_energy(&self)->i64{
        let mut res = 0;
        for a in &self.planets { res+=a.get_energy(); }
        res
    }
}
// nie ma az do 9750
fn main() {
    let mut system = System::new();

    system.add_planet(12,  0,-15);
    system.add_planet(-8, -5,-10);
    system.add_planet( 7,-17,  1);
    system.add_planet( 2,-11, -6);

    println!("res:{}",system.compute_loop());
}

#[test]
fn test1()
{
    let mut system = System::new();

    system.add_planet(-1,   0,  2);
    system.add_planet( 2, -10, -7);
    system.add_planet( 4,  -8,  8);
    system.add_planet( 3,   5, -1);

    assert_eq!(system.compute_loop(),2772);
}

fn test2(){
    let mut system = System::new();

    system.add_planet(-8,-10,0);
    system.add_planet(5, 5,  10);
    system.add_planet(2, -7, 3);
    system.add_planet(9, -8, -3);
                                     
    assert_eq!(system.compute_loop(),4686774924);
}

