use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::ErrorKind;
use std::io::Read;
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

    let mut scores3 = HashMap::new();
    scores3.insert(String::from("Blue"), 10);
    scores3.insert(String::from("Yellow"), 50);
    scores3.insert(String::from("Yellow"), 70);

    let team_name = String::from("Blue");

    let score = scores.get(&team_name);

    println!("{:?}", score);

    for (key, value)  in  &scores3 {
        println!("{} -> {}", key, value);
    }


    //insert if key does not exist
    scores3.entry(String::from("red")).or_insert(100);
    scores3.entry(String::from("Black")).or_insert(100);
    println!("-------------------------");
    for (key, value)  in  &scores3 {
        println!("{} -> {}", key, value);
    }
    println!("-------------------------");


    println!("{:#?}", &scores3);


    let text = "Hello world wonderful world";

    let mut map = HashMap::new();
    for word in text.split_whitespace()  {
        let count = map.entry(word).or_insert(0); 
        *count += 1;
    }


    println!("{:#?}", &map);
    //panic!("crash and burn");
    //let v = vec![1,2,3];
    //v[100];
    
    //let f = File::open("hello.txt");
    let f = File::open("/tmp/empty.txt");
    let f = match f {
        Ok(file) =>file ,
        Err(error) => {
            //println!("empty.txt does not exist")
            panic!("There was a problem opening the file {:?}", error)
        },
    };

    let f = File::open("empty.txt");
    let f = match f {
        Ok(file) =>file ,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Tried to create file but there was a problem: {:?}", e)
            },
            other_error => panic!("There was a problem opening the file: {:?}", other_error),
        },
    };

}

pub fn open() {
    let f = File::open("file.txt").map_err(|err| {
        if err.kind() == ErrorKind::NotFound {
            File::create("file.txt").unwrap_or_else(|error| {
                panic!("tried to create file but there was a problem: {:?}", error);
            })
        } else {
            panic!("There was a  problem opening the file : {:?}", err);
        } 
    });
}

pub fn open_with_unwrap() {
    let f = File::open("not_exists.txt").unwrap();
}

pub fn open_with_expect() {
    let f = File::open("not_exists.txt").expect("file not_exists.txt does not exists.");
}


pub fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e)
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }

}
pub  fn read_username_from_file2() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

