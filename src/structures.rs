


struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
pub fn custom_struct(){
    let  userobject = User{
        active:false, 
        username:   "apurva".to_string(),
        email:"example@gmail.com".to_string(),
        sign_in_count:2,
    }; 

    println!("{}",userobject.username) ; 
}