use std::collections::HashMap;

#[derive(Debug,Clone,Hash)]
struct Elem {
    n : i64,
    name : String,
}

impl Elem {
    fn new(s:String)->Elem{
        let t:Vec<&str> = s.split(' ').collect();
        Elem{
            n : t[0].parse::<i64>().unwrap(),
            name : t[1].to_string(),
        }
    }
}

fn comp(data:Vec<String>) -> i64 {
    let mut hash : HashMap<String,Elem> = HashMap::new();
    let mut res = 0;
    
    for d in data {
        let t : Vec<&str> = d.split(" => ").collect();
        let r_elem = Elem::new(t[1].to_string());
        hash.insert(r_elem.name, r_elem);
    }
    res
}

fn main() {

    let data = vec![
"10 LSZLT, 29 XQJK => 4 BMRQJ",
"22 HCKS => 1 GQKCZ",
"1 HCKS, 8 WZWRV, 18 HVZR => 7 BGFR",
"1 LSZLT, 1 WRKJ, 3 LJFP, 3 RNLPB, 1 NZGK, 3 LDSV, 5 RJDN, 8 HGFGC => 3 QZTXD",
"1 BRSGQ, 1 XGLF, 1 ZHSK, 20 LSZLT, 16 WFCPT, 3 KTWV, 1 QRJC => 4 XPKX",
"1 DCLR, 6 RNLPB => 5 HCKS",
"1 HFHFV => 3 SHLMF",
"2 LTMZQ, 21 FGCXN => 6 QKFKV",
"3 BGFR => 7 WRKJ",
"3 KHSB => 2 XQJL",
"3 SHLMF => 2 LPLG",
"12 SVHWT, 20 BXPSZ => 9 NBMF",
"2 FGCXN, 32 DCSVN => 8 TBDWZ",
"1 KHSB, 3 HGFGC => 6 WZWRV",
"27 WFCPT, 4 KTWV, 14 BRSGQ, 1 MFNK, 1 WRKJ, 2 NZGK, 24 FBFLK => 5 TRLCK",
"2 SVHWT => 3 QRJC",
"1 MNVR, 1 FKBMW => 2 FGCXN",
"4 GJXW => 9 JXFS",
"3 XQJK => 5 WNJM",
"1 WZVWZ, 1 XQJL => 9 SHKJV",
"2 DCSVN => 4 HDVC",
"2 GJXW => 2 RNLPB",
"1 QKFKV, 1 PBRWB => 5 WTZQ",
"14 QKFKV => 6 RDFTD",
"166 ORE => 1 QDSXV",
"2 DCSVN => 5 BXPSZ",
"113 ORE => 6 LTMZQ",
"13 MNVR => 7 RJDN",
"2 NZGK, 9 XQJK, 18 WRKJ => 9 KTWV",
"1 NZGK => 8 XQJK",
"6 RZCGN, 6 HDVC, 1 DLKR => 9 DSLXW",
"18 HVZR => 8 LJFP",
"7 XQJL => 1 NPDS",
"15 DLKR, 1 DSLXW, 26 MJFVP => 3 FBFLK",
"125 ORE => 9 MNVR",
"3 RJDN => 4 HFHFV",
"1 TBDWZ, 1 DCLR => 2 HVZR",
"2 SHKJV => 5 GJXW",
"7 LTMZQ, 1 QDSXV, 1 FKBMW => 3 DCSVN",
"9 LPLG, 11 JXFS => 3 BRSGQ",
"5 JXFS, 1 ZHSK, 25 XGLF => 4 MFNK",
"5 PBRWB => 2 SVHWT",
"15 SHKJV => 5 XGLF",
"1 XQJL, 2 NPDS => 4 DLKR",
"39 JXFS => 5 KSHF",
"6 GJXW, 1 FBFLK => 7 HGFGC",
"3 JXFS => 1 LSZLT",
"3 NBMF, 1 BMRQJ => 2 LDSV",
"1 JXFS, 25 GJXW, 10 HGFGC => 4 NZGK",
"8 QZTXD, 26 KSHF, 60 WNJM, 6 GJXW, 9 TRLCK, 20 XPKX, 21 FGCXN, 57 GQKCZ, 6 WRKJ => 1 FUEL",
"4 SVHWT, 1 RZCGN => 3 ZHSK",
"1 BXPSZ => 7 DCLR",
"8 RDFTD, 1 SHKJV, 1 HFHFV => 6 MJFVP",
"1 LTMZQ => 9 KHSB",
"5 WTZQ, 4 HGFGC, 4 HCKS => 9 WFCPT",
"184 ORE => 4 FKBMW",
"4 XQJL => 3 WZVWZ",
"12 QDSXV => 9 RZCGN",
"1 FBFLK, 7 HVZR => 9 PBRWB"];


    println!("Hello, world!");
}

#[test]
fn test0()
{
    let d = vec![
"10 ORE => 10 A",
"1 ORE => 1 B",
"7 A, 1 B => 1 C",
"7 A, 1 C => 1 D",
"7 A, 1 D => 1 E",
"7 A, 1 E => 1 FUEL"];

assert_eq!(comp(d),31);
}