use std::collections::HashSet;
use std::collections::HashMap;

fn get_data(v:&Vec<&str>)->HashSet<(usize,usize)> {
    let mut hash = HashSet::new();

    for y in 0..v.len() {
        for x in 0..v[y].len() {
            if v[y].chars().nth(x).unwrap()=='#' { hash.insert((x,v.len()-1-y)); }
        }
    }
    hash
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
struct Vec2{
    x: i64,
    y: i64,
}

impl Vec2 {
    fn new(fx:i64,fy:i64)->Vec2
    {
        Vec2{
            x:fx,
            y:fy,
        }
    }
    fn newu(fx:usize,fy:usize)->Vec2
    {
        Vec2{
            x:fx as i64,
            y:fy as i64,
        }
    }

    fn gdc(&self,a:u64,b:u64) -> u64
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

    fn len_no_square(&self,pos_x:i64,pos_y:i64)->i64
    {
        (self.x - pos_x)*(self.x - pos_x) + (self.y - pos_y)*(self.y - pos_y)
    }

    fn normal(&mut self)
    {
        let len = self.gdc(self.x.abs() as u64,self.y.abs() as u64);
        self.x/=len as i64;
        self.y/=len as i64;
    }
}

fn comp(v:&Vec<&str>) -> (usize,usize,usize){

    let hash = get_data(v);
    let mut res = (0usize,0usize,0usize);

    for i in &hash {        
        let mut hash_dir : HashSet<Vec2> = HashSet::new();

        for j in &hash {
            if i!=j {
                   let mut dir = Vec2::new(j.0 as i64 - i.0 as i64 ,j.1 as i64 - i.1 as i64);
                   dir.normal();
                   hash_dir.insert(dir);
            }        
        }
        //println!("{:?}={}",i,hash_dir.len());
        if hash_dir.len()>res.0 {
            res.0 = hash_dir.len();
            res.1 = i.0;
            res.2 = i.1;
        }
    }
    res
}

fn get_angle(dx:i64,dy:i64)->f64
{
    let angle = ((dx as f64).atan2(dy as f64)*180.0f64)/std::f64::consts::PI;
    if angle<0.0f64 { return 360.0f64 + angle; } 
    angle
}

fn fire(v:&Vec<&str>,x:usize,y:usize)->Vec<Vec2>{
    let mut res : Vec<Vec2> = vec![];
    
    let hash = get_data(v);
    
    let i = hash.get(&(x,y)).unwrap();
    let mut hash_dir : HashMap<Vec2,Vec<Vec2>> = HashMap::new();

    for j in &hash {
        if i!=j {
                let mut dir = Vec2::new(j.0 as i64 - i.0 as i64 ,j.1 as i64 - i.1 as i64);
                dir.normal();
               
                let v = hash_dir.get_mut(&dir);
                if v==None { hash_dir.insert(dir.clone(),vec![]);  }
                hash_dir.get_mut(&dir).unwrap().push(Vec2::newu(j.0,j.1));
        }        
    }

    //println!("hashdir {:?}",hash_dir);

    let mut all = vec![];

    for mut k in hash_dir {
        // sort points by distance
        k.1.sort_by(|a,b| {
            let a_len = a.len_no_square(x as i64,y as i64);
            let b_len = b.len_no_square(x as i64,y as i64);
            a_len.partial_cmp(&b_len).unwrap() }   
        );
        // create dirs vec
        all.push(k);
    }

    // sort dirs by angle
    all.sort_by(|a, b| {
        let a_ang = get_angle(a.1[0].x as i64 - x as i64,a.1[0].y as i64 - y as i64);
        let b_ang = get_angle(b.1[0].x as i64 - x as i64,b.1[0].y as i64 - y as i64);
        a_ang.partial_cmp(&b_ang).unwrap() }
    );

    //println!("all aft {:?}",all);

    let mut num = 0;
    let mut id = 0;

    // rotate and shoot over
    while num<hash.len()-1 {

        if all[id].1.len()>0 {
            res.push(Vec2::new(all[id].1[0].x, v.len() as i64 -1-all[id].1[0].y));
            all[id].1.remove(0);
            num+=1;
        }
        id = (id+1)%all.len();
    }
    
    res
}

fn main() {

let data = vec![
".#......##.#..#.......#####...#..",
"...#.....##......###....#.##.....",
"..#...#....#....#............###.",
".....#......#.##......#.#..###.#.",
"#.#..........##.#.#...#.##.#.#.#.",
"..#.##.#...#.......#..##.......##",
"..#....#.....#..##.#..####.#.....",
"#.............#..#.........#.#...",
"........#.##..#..#..#.#.....#.#..",
".........#...#..##......###.....#",
"##.#.###..#..#.#.....#.........#.",
".#.###.##..##......#####..#..##..",
".........#.......#.#......#......",
"..#...#...#...#.#....###.#.......",
"#..#.#....#...#.......#..#.#.##..",
"#.....##...#.###..#..#......#..##",
"...........#...#......#..#....#..",
"#.#.#......#....#..#.....##....##",
"..###...#.#.##..#...#.....#...#.#",
".......#..##.#..#.............##.",
"..###........##.#................",
"###.#..#...#......###.#........#.",
".......#....#.#.#..#..#....#..#..",
".#...#..#...#......#....#.#..#...",
"#.#.........#.....#....#.#.#.....",
".#....#......##.##....#........#.",
"....#..#..#...#..##.#.#......#.#.",
"..###.##.#.....#....#.#......#...",
"#.##...#............#..#.....#..#",
".#....##....##...#......#........",
"...#...##...#.......#....##.#....",
".#....#.#...#.#...##....#..##.#.#",
".#.#....##.......#.....##.##.#.##"];


let data9 = vec![
".#..##.###...#######",
"##.############..##.",
".#.######.########.#",
".###.#######.####.#.",
"#####.##.#.##.###.##",
"..#####..#.#########",
"####################",
"#.####....###.#.#.##",
"##.#################",
"#####.##.###..####..",
"..######..##.#######",
"####.##.####...##..#",
".#####..#.######.###",
"##...#.##########...",
"#.##########.#######",
".####.#.###.###.#.##",
"....##.##.###..#####",
".#.#.###########.###",
"#.#.#.#####.####.###",
"###.##.####.##.#..##"];

//let data = vec![
//".#....#####...#..",
//"##...##.#####..##",
//"##...#...#.#####.",
//"..#.....#...###..",
//"..#.#.....#....##"];

//let data = vec![
//  "###",
//    "###",
//    "###"];
    

    let c = comp(&data);
    println!("pos:{},{}",  c.1,c.2);
    let fired = fire(&data,c.1,c.2);


    println!("fired:{:?}",fired);
    println!("res: 200th {:?}",fired[199]);
}

//823 too low

#[test]
fn test0(){
    let data = vec![".#..#",
    ".....",
    "#####",
    "....#",
    "...##"];

    assert_eq!(comp(&data).0,8);
}

#[test]
fn test1(){

let data = vec![
".#..#..###",
"####.###.#",
"....###.#.",
"..###.##.#",
"##.##.#.#.",
"....###..#",
"..#.#..#.#",
"#..#.#.###",
".##...##.#",
".....#.#.."];

assert_eq!(comp(&data).0,41);
}

#[test]
fn test2(){

let data = vec![
".#..##.###...#######",
"##.############..##.",
".#.######.########.#",
".###.#######.####.#.",
"#####.##.#.##.###.##",
"..#####..#.#########",
"####################",
"#.####....###.#.#.##",
"##.#################",
"#####.##.###..####..",
"..######..##.#######",
"####.##.####...##..#",
".#####..#.######.###",
"##...#.##########...",
"#.##########.#######",
".####.#.###.###.#.##",
"....##.##.###..#####",
".#.#.###########.###",
"#.#.#.#####.####.###",
"###.##.####.##.#..##"];

assert_eq!(comp(&data).0,210);
} 

#[test]
fn test_angles()
{
    assert_eq!(get_angle( 0, 1),  0.0);
    assert_eq!(get_angle( 1, 0), 90.0);
    assert_eq!(get_angle( 1, 1), 45.0);
    assert_eq!(get_angle( 0, 1),  0.0);
    assert_eq!(get_angle( 0,-1),180.0);
    assert_eq!(get_angle(-1, 0),270.0);
    assert_eq!(get_angle(-1,-1),225.0);
    assert_eq!(get_angle(-1, 1),315.0);
}
