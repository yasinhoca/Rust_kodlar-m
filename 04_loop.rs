use rand::Rng; 

fn main() {
    
    let num = rand::thread_rng().gen_range(0..100);
    println!("{}", num);
    
    let mut sayac = 0;
    
    loop{
        print!("{}-",sayac);
        sayac+=1;
        if sayac==num{
            break;
        }
    }
    
}
