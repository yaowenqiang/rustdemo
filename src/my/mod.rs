pub fn say_hello() {
    println!("hello world");
}

/*
pub fn life_cycle_demo() {
    let r;
    {
        let r = 5;
        r = &x;
    }
    println!("r : {}", r);

    {
        let x = 5;
        let r = &x;
        println!("r:{}", r)
    }
}
*/
pub fn function_liftcycle () {
    let string1 = String::from("abcd");
    let string2 = String::from("xyz");

    //let result = longest(string1.as_str(), string2);
    //println!("The longest string is {}", result);

    let string1 = String::from("long string is long");
    {
        let string2 = String::from("xyz");
        let result = longest2(string1.as_str(), string2.as_str());
        //println!("The longest string is {}", result );
    }
}

/*
pub fn longest<'a>(x: &str, y: &str) -> &'a &str {
    if x.len() > y.len() {
        x   
    } else {
        y
    }
}
*/
fn longest2<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
