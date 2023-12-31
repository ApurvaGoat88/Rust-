use std::collections::HashMap ;

pub fn hash(){
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("{}:::::{}" ,team_name,score); 
    for (key, value) in &scores {
     println!("{key}: {value}");
    }
}
