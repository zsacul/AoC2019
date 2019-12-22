fn cut(s:&Vec<i32>,n:i32)->Vec<i32>
{
    let mut res = vec![];
    let l = s.len() as i32;
    for i in 0..s.len() as i32 {
        let id = (l+i+n)%l;
        res.push(s[id as usize]);
    }
    res
}

fn new_stack(s:&Vec<i32>)->Vec<i32>
{
    println!("new stack");
    let mut res = vec![];
    for i in (0..s.len()).rev() {
        res.push(s[i]);
    }
    res
}

fn increment(s:&Vec<i32>,n:i32)->Vec<i32> {
    let mut res = vec![];
    let l = s.len() as i32;
    let mut id=0i32;
    res.resize(l as usize,0);

    for i in 0..s.len() {        
        res[id as usize] = s[i];
        id = (id+n)%l;
    }
    res
}

fn init_stack(n:i32)->Vec<i32>{
    let mut res = vec![];
    for i in 0..n { res.push(i); }
    res
}


fn find_card(data:&Vec<i32>,id:i32)->i32
{
    for i in 0..data.len() {
        if data[i]==id { return i as i32; }
    }
    -1
}

fn compute(data:Vec<&str>,num:i32,card:i32)->i32
{
    let mut stack = init_stack(num);

    for s in data {
        let st = s.to_string();
        let tab = st.split(" ").collect::<Vec<&str>>();

        match tab[0] {
            "cut"  => stack = cut(&stack, tab[1].parse::<i32>().unwrap() ),
            "deal" => {
                        match tab[1] {
                            "with" => stack = increment(&stack, tab[3].parse::<i32>().unwrap() ),
                            "into" => stack = new_stack(&stack),
                            _      => panic!("error"),
                        }
                      },
            _ => panic!("error") ,
        }
    }
    find_card(&stack,card)
}

fn main() {

    let data = vec![
    "cut -4258",
    "deal with increment 71",
    "cut -6593",
    "deal into new stack",
    "deal with increment 54",
    "cut -5397",
    "deal into new stack",
    "cut 1327",
    "deal with increment 20",
    "deal into new stack",
    "deal with increment 45",
    "cut -9986",
    "deal into new stack",
    "deal with increment 47",
    "cut -3318",
    "deal with increment 75",
    "cut 542",
    "deal with increment 48",
    "cut 8670",
    "deal with increment 13",
    "deal into new stack",
    "deal with increment 5",
    "cut -8813",
    "deal with increment 36",
    "cut 3228",
    "deal with increment 21",
    "cut 5143",
    "deal with increment 13",
    "cut 7329",
    "deal with increment 74",
    "deal into new stack",
    "deal with increment 4",
    "cut 4178",
    "deal with increment 29",
    "cut -7664",
    "deal with increment 17",
    "cut 8216",
    "deal with increment 22",
    "cut -7497",
    "deal with increment 10",
    "cut -2813",
    "deal into new stack",
    "cut 8416",
    "deal with increment 16",
    "cut -4124",
    "deal with increment 13",
    "cut -8531",
    "deal with increment 74",
    "cut -9397",
    "deal with increment 57",
    "cut -1832",
    "deal with increment 34",
    "cut -2538",
    "deal into new stack",
    "cut 7837",
    "deal with increment 57",
    "cut 5257",
    "deal with increment 2",
    "cut -8241",
    "deal with increment 26",
    "deal into new stack",
    "deal with increment 39",
    "cut -659",
    "deal with increment 58",
    "cut 34",
    "deal into new stack",
    "deal with increment 46",
    "cut 9168",
    "deal with increment 35",
    "cut 8530",
    "deal into new stack",
    "cut 297",
    "deal into new stack",
    "cut 1116",
    "deal with increment 69",
    "cut 5440",
    "deal with increment 6",
    "deal into new stack",
    "cut 3811",
    "deal with increment 7",
    "deal into new stack",
    "cut -8657",
    "deal with increment 29",
    "cut 8933",
    "deal with increment 4",
    "cut -6643",
    "deal with increment 37",
    "cut 1688",
    "deal with increment 32",
    "cut -554",
    "deal with increment 69",
    "deal into new stack",
    "deal with increment 64",
    "cut 4395",
    "deal with increment 71",
    "cut -9180",
    "deal with increment 60",
    "cut 6480",
    "deal with increment 73",
    "cut -7146"];

    println!("{}", compute(data,10007,2019));   
}

#[test]
fn test0()
{
    let mut st = init_stack(10);

    st = new_stack(&st);
    
    st = cut(&st, -2);    
    st = increment(&st,7);
    st = cut(&st, 8);
    st = cut(&st, -4);
    st = increment(&st,7);
    st = cut(&st, 3);
    st = increment(&st,9);
    st = increment(&st,3);
    st = cut(&st, -1);

    assert_eq!(st,vec![9,2,5,8,1,4,7,0,3,6]);

    println!("{:?}",st);
}

#[test]
fn test1()
{
    let mut st = init_stack(10);
    st = new_stack(&st);
    assert_eq!(st,[9,8,7,6,5,4,3,2,1,0]);
}

#[test]
fn test2()
{
    let mut st = init_stack(10);
    st = cut(&st,-4);
    assert_eq!(st,[6,7,8,9,0,1,2,3,4,5]);
}