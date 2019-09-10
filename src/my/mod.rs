pub fn say_hello() {
    println!("hello world");
}

pub fn life_cycle_demo() {
    let r;
    {
        let r = 5;
        r = &x;
    }
    println!("r : {}", r);
}
