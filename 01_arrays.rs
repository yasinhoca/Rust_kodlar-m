fn main() {
  let dizi : [i32;5] = [1,2,3,4,5];
  println!("{}",dizi[0]);
  println!("{}",dizi.len());
  
  let s: [i32;100] = [5;100]; //100 adet 5 ile dolduruyor
  for i in s{
      print!("{}-",i);
  }
  
}
