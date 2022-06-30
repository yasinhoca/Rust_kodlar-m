use rand::Rng; 

fn main() {
    let mut v: Vec<i32> = Vec::new();
    let mut r : i32;
    
    for _i in 0..20{
        r = rand::thread_rng().gen_range(0..100);
        v.push(r);
    }
    
    for _i in v{
        print!("{}-",_i);
    }
    
}
