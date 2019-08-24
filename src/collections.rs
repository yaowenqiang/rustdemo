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


}
