use std::collections::HashMap;
pub fn show_hash_map()
{
    let mut  scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);


    println!("{:#?}", scores);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![50,100];
    let scores2 : HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect(); 
    println!("{:#?}", scores2);


    let field_name  = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    println!("{:#?}", scores2);
    println!("{:#?}", map);
    //println!("{}", field_name);
    //println!("{}", field_value);
}
