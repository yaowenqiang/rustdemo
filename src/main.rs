fn main() {
    //let x = 5;
    let mut x = 5;
    let y = 7;
    x = 4;
    println!("{}", x);

    let tup: (u8, i8, f32) = (1, -1, 3.5);

    if x < y {
        println!("{} is smaller than {}",x, y );
    } else if x == y {
        println!("{} is equal to  {}",x, y );
    } else {
        println!("{} is bigger than  than {}",x, y );
    }

//    loop {
//        println!("Rust");
//    }

    for x in 0..10 {
        println!("{}", x);
    }

    let mut end = false;
    let mut counter = 1;
    while end == false {
        println!("Counter is: {}", counter);

        if counter == 10 {
            end = true;
        }
        counter = counter + 1;
    }

    let name = "Jacky";
    match name {
        "Jack" => println!("oh Jack"),
        "Jacky" => println!("oh Jacky"),
        "John" => println!("hi John"),
        _ => println!("hi stranger!"),

    };

    let name: &str = "Rust";
    let names2: &'static str = "Rust21";
    let mut some_string = String::new();
    some_string = "Some String".to_string();
    let string_slice = &some_string;
    let arrays = [1,2,3,4];

    //array.len();

    for a in arrays.iter() {
        println!("a is {}", a);
    }

    let mut vector1:  Vec<i32> = Vec::new();
    let mut vector2:  Vec<i32> = vec![1,2,3,4,5];
    let mut vector3:  Vec<bool> = Vec::with_capacity(10);

    let values = vec![1,2,3];

    for a in values {
        println!("a is {}", a)
    }

    let array2 = ["This", "is", "an", "array"];
    let array_slice = &array[1..4];




}
