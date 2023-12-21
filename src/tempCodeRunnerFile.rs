println!("Welcome") ; 
    let mut num1 = String::new() ; 
    io::stdin().read_line(&mut num1).expect("Failed to read line");
    let num:i32 = num1.trim().parse().expect("It is not number ") ; 
    {
        num = num*2 ; 
        println!("functional scope : {}",num) ;

    }
    println!("global scope : {}",num) ; 