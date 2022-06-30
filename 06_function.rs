use rand::Rng; 

fn main() {

    let mut r : i32;
    
    for _i in 0..10{
        r = rand::thread_rng().gen_range(0..100);
        print!("{}-",r);
        tekCift(r);
    }
}

fn tekCift(s:i32){
    if s%2==0{
        println!("çift-");
    }else{
        println!("tek-");
    }
}


