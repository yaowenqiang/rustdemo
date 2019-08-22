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
    let does_not_exist = v3[200];
    println!("{}", does_not_exist);
    //let does_not_exist = v3.get(100);
    //println!("{}", does_not_exist);

}
