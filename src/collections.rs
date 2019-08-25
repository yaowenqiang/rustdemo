pub fn show_vctor () {
    let v: Vec<i32> = Vec::new();
    let mut v2 = vec![1,2,3];
    /*
    for i in v2 {
        println!("{}", i);
    }
    */
    v2.push(100);

    //let third: &i32 = &v2[2];
    let third: i32 = v2[2];
    println!("the third element is {}", third);

    match v2.get(1) {
        Some(second) => println!("second is {}", second),
        None => println!("There is no third element.")
    }

    let v3 = vec![1,2,3,4,5];
    //let does_not_exist = v3[200];
    //println!("{}", does_not_exist);
    let mut first =  v3[0];
    //v3.push(100);
    println!("{}", first);
    first = 4;
    println!("{}", first);
    //let does_not_exist = v3.get(100);
    //println!("{}", does_not_exist);
    //

    let v = vec![100,25,57];
    //for i in v {
    for i in &v {
        println!("{}", i);
    }

    let mut v2 = vec![100, 32, 57];

    for i in &mut v2 {
        *i += 100;
        println!("{}", i);
    }
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let  row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(10.12),
        SpreadsheetCell::Text(String::from("blue"))
    ];

    println!("{:?}",row);


    let mut s = String::new();
    let data = "s1 initial contents";
    let s1 = data.to_string();

    let s2 = "s2 initial contents".to_string();

    let s3 = String::from("initial contents");

    let mut s4  = String::from("foo");
    s4.push_str("bar");
    println!("{}", s4);

    let s5 = s1 + " " + &s2;
    println!("s5: {}", s5);
    println!("s2: {}", s2);
    //println!("s1: {}", s1);






}
