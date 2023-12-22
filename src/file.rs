
use std::fs::File ;

pub  fn file_functino(){
    let f = File::open("hello.txt"); 
    let f  = match f{
        Ok(f)=>f,
        Err(e)=>panic!("error {}",e)
    };
}
