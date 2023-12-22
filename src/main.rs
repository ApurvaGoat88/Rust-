
mod scope;
mod hashmap;
mod panic;
mod ownership;
mod structures;
mod calculate_area_using_rust;
use panic::panic_function ; 
mod file;
use file::file_functino ; 
fn main() {

    println!("Hello, world!");
    // scope_funtion()  ;
    // owner(); 
    // custom_struct() ;
//    let mut area =  cal_area(34, 50) ;
//    println!("{}",area) ;


//    let v:Vec<i32> = Vec::new() ; 
//    let v2 = vec![1,23,43,132] ; 
//    println!("{:?}",v2 ); 

//    hash(); 

file_functino() ;
// panic_function() ;
    
}
