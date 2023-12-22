
mod scope;
mod ownership;
mod structures;
mod calculate_area_using_rust;
use calculate_area_using_rust::cal_area ;
use structures::custom_struct ;


fn main() {

    println!("Hello, world!");
    // scope_funtion()  ;
    // owner(); 
    custom_struct() ;
   let mut area =  cal_area(34, 50) ;
   println!("{}",area) ;


   let v:Vec<i32> = Vec::new() ; 
   let v2 = vec![1,23,43,132] ; 
   print!("{:?}",v2 ); 
    
}
