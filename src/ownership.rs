

pub fn owner (){ 
    let mut s1 = String::from("hello") ; 
    let mut s2  = "world"; 
    println!("The original string is {}",s1);
    s1.push_str(&s2) ; 
    println!("String after push :{} ",s1 );

    let len = cal_len(&s1) ;
    println!("{}",len) ;



}
fn cal_len(str : &String)-> usize{
    str.len() 
}