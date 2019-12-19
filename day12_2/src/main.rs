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

    fn move_planet(&mut self){
        self.position.x+=self.velocity.x;
        self.position.y+=self.velocity.y;
        self.position.z+=self.velocity.z;
    }
}

struct System {
    planets   : Vec<Planet>,
}

impl System{
    fn new()->System{
        System{
            planets : vec![],
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

    fn same_axis(&self,o:&Vec<Planet>,axis:char)->bool
    {
        match axis {
            'x' => self.planets[0].position.x == o[0].position.x &&
                   self.planets[1].position.x == o[1].position.x &&
                   self.planets[2].position.x == o[2].position.x &&
                   self.planets[3].position.x == o[3].position.x &&
                   self.planets[0].velocity.x == o[0].velocity.x &&
                   self.planets[1].velocity.x == o[1].velocity.x &&
                   self.planets[2].velocity.x == o[2].velocity.x &&
                   self.planets[3].velocity.x == o[3].velocity.x ,
            'y' => self.planets[0].position.y == o[0].position.y &&
                   self.planets[1].position.y == o[1].position.y &&
                   self.planets[2].position.y == o[2].position.y &&
                   self.planets[3].position.y == o[3].position.y &&
                   self.planets[0].velocity.y == o[0].velocity.y &&
                   self.planets[1].velocity.y == o[1].velocity.y &&
                   self.planets[2].velocity.y == o[2].velocity.y &&
                   self.planets[3].velocity.y == o[3].velocity.y ,
            'z' => self.planets[0].position.z == o[0].position.z &&
                   self.planets[1].position.z == o[1].position.z &&
                   self.planets[2].position.z == o[2].position.z &&
                   self.planets[3].position.z == o[3].position.z &&
                   self.planets[0].velocity.z == o[0].velocity.z &&
                   self.planets[1].velocity.z == o[1].velocity.z &&
                   self.planets[2].velocity.z == o[2].velocity.z &&
                   self.planets[3].velocity.z == o[3].velocity.z ,
             _  => panic!("err"),
        }
    }

    fn gdc(&self,a:i128,b:i128) -> i128
    {
        let mut aa = a;
        let mut bb = b;
        
        while bb!=0 {
            let t = bb;
            bb = aa%bb;
            aa = t;
        }
        aa
    }    

    fn compute_loop(&mut self)->i128{
        let mut iterations = 0i128;
        let planets_copy = self.planets.clone();

        let mut xi=-1i128;
        let mut yi=-1i128;
        let mut zi=-1i128;

        while xi==-1 || yi==-1 || zi==-1 {

            self.apply_gravity();
            self.move_planets();   

            iterations+=1;
            if self.planets==planets_copy { return iterations; }

            if self.same_axis(&planets_copy,'x') && xi==-1 { xi = iterations; }
            if self.same_axis(&planets_copy,'y') && yi==-1 { yi = iterations; }
            if self.same_axis(&planets_copy,'z') && zi==-1 { zi = iterations; }
        }

        println!("ix:{}",xi);
        println!("iy:{}",yi);
        println!("iz:{}",zi);
        
        xi/=self.gdc(xi,yi);
        yi/=self.gdc(yi,zi);
        zi/=self.gdc(zi,xi);

        xi*yi*zi
    }
}

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
