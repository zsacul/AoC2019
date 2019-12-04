fn ok(num:u32)->bool {    
    let tab:Vec<char> = num.to_string().chars().collect();
    let mut b_same = false;
    for i in 0..tab.len()-1 {
        if tab[i+1]< tab[i] { return false; }
        if tab[i+1]==tab[i] && (i==0 || tab[i]!=tab[i-1]) && (i+2==tab.len() || tab[i]!=tab[i+2]) { b_same = true;}
    }
    b_same
}

fn main() {
    let mut res = 0;
    for i in 138307..=654504 {
        if ok(i) { res+=1; }
    }
    println!("{}",res);
}

