use std::collections::HashSet;

fn get_data(v:&Vec<&str>)->HashSet<(usize,usize)> {
    let mut hash = HashSet::new();

    for y in 0..v.len() {
        for x in 0..v[y].len() {
            if v[y].chars().nth(x).unwrap()=='#' { hash.insert((x,y)); }
        }
    }
    hash
}

fn comp(v:&Vec<&str>){

    let hash = get_data(v);
    for i in &hash {
        let mut res = 0i32;
        for j in &hash {
            if i!=j {
                if (i.0+j.0)%2==0 && (i.1+j.1)%2==0
                {
                    let xx = (i.0 + j.0)/2;
                    let yy = (i.1 + j.1)/2;
                   // println!("{},{} ",xx,yy);
                    if hash.get(&(xx as usize,yy as usize))==None { res+=1; }
                }
            }
        }
        println!("{:?}={}",i,res);
    }

}

fn main() {

    let data = vec![".#..#",
                    ".....",
                    "#####",
                    "....#",
                    "...##"];

    comp(&data);
    //println!("Hello, world!");
}
