mod structs;
extern crate rand;
use rand::Rng;
use structs::MineField;
mod my;
use std::thread;
use std::io::stdin;
use std::mem;

const MEANING_OF_LIFE  :u8 = 42; // no  fixed address

static mut Z:i32 = 123;

extern crate some_crate;
//use some_crate::User;
#[derive(Debug)]
struct Hero {
    name: String,
    energy: u16,
    strike: bool
}

#[derive(Debug)]
struct Goblin {
    energy: u16,
    strike: bool
}

impl Hero{
    fn jump(&self) {
        //some logick for jumping
    }
}

trait StrikeTrait{
    fn strike(&mut self);
}

impl StrikeTrait for Hero{
    fn strike (&mut self) {
        self.strike = true
    }
}


impl StrikeTrait for Goblin {
    fn strike (&mut self) {
        self.strike = false
    }
}


struct UserCollection<T,P>
{
    name: String,
    users: Vec<P>,
    size: T
}

#[derive(Debug)]
struct TestStruct<'a> {
    x: &'a u32
}

impl<'a> TestStruct<'a>{
    fn return_x(&self) ->&'a u32{
        self.x
    }
}

#[should_panic]
#[test]

fn test_user_structure(){
    let new_user = some_crate::User{
        name:"Dave".to_string(),
        email: "dave@email.com".to_string(),
        age: 32,
        user_type: some_crate::UserType::Guest
    };
    assert_eq!("Dave".to_string(), new_user.name );

}

fn increase_by_five<'a>(x: &'a u16) -> u16 {
    x + 5
}
//mod my;
/*
#[derive(Debug)]
struct User {
    name: String,
    email: String,
    age: i16,
    user_type: UserType
}

impl User{
    pub fn print_user(self) {
        println!("The name of the user is {}.\n", self.name);
        println!("HIs email is  {}.\n", self.email);
        println!("He is {} years old.\n", self.age);
    }
}


#[derive(Debug)]
enum UserType{
    Guest,
    Regular,
    Admin
}
*/
fn say_hello()
{
    print!("hello world");
}
fn multiplication(a:i32, b:i32) -> i32 {
    a * b
}
fn multiplication_print_result(x: i32, y: i32) {
   // println!("{:?}", x*y );
    println!("{:?}", multiplication(x, y));
}
fn main() {
    let user_type = some_crate::UserType::Regular;
    println!("user type is {:?}\n", user_type);
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
    let array_slice = &array2[1..4];

    let z = 2;
    {
        let z = 3;
    }
    println!("{}", z);

    let mut string1: String  = String::new();
    string1 = "A string".to_string();
    //let string2 = string1; will produce an  error
    let string2 = &string1;
    println!("{}",string2);
    println!("{}",string1);

    println!("{:?}",multiplication(x,y));
    multiplication_print_result(x, y);
    say_hello();
    my::say_hello();

    let new_user = some_crate::User{
        name:"Dave".to_string(),
        email: "dave@email.com".to_string(),
        age: 32,
        user_type: some_crate::UserType::Guest
    };
    println!("{:?}", new_user);
    // pretty print 
    println!("{:#?}", new_user);
    let new_user2 = some_crate::User{
        name:"Dave".to_string(),
        email: "dave@email.com".to_string(),
        age: 32,
        user_type: some_crate::UserType::Guest
    };
    let user_collection: UserCollection<u8, some_crate::User> = UserCollection{
        name: "user collection 1".to_string(),
        users: vec![new_user2],
        size: 1
    };
    

    new_user.print_user();
    println!("{:?}", user_collection.users);

    let mut x = 5;
    x = increase_by_five(&x);
    println!("{}", x);

    let ts = TestStruct{
        x: &5
    };
    println!("{:?}", ts.x);
    println!("{:?}", ts.return_x());

    let mut hero = Hero{
        name: "Dave".to_string(),
        energy: 100,
        strike: false
    };

    let mut goblin = Goblin{
        energy: 99,
        strike: true
    };

    println!("{:#?}", hero);
    hero.strike();
    println!("{:#?}", hero);

    println!("{:#?}", goblin);
    goblin.strike();
    println!("{:#?}", goblin);

    let thread_1 = thread::spawn(|| {
        "Hello"
    });
    let thread_2 = thread::spawn(|| {
        "World"
    });
    thread::sleep(std::time::Duration::from_millis(100));
    println!("{:?}", thread_1.join().unwrap());
    println!("{:?}", thread_2.join().unwrap());

    let mut mine_field = MineField {
        size: (3,4),
        mines: Vec::new(),
    };

    &mine_field.generate_mines();
    println!("{:#?}", mine_field);
    mine_field.print_fields();


    let mut xx = 0;
    let mut yy = 0;
    println!("You have five tries");
    let mut counter = 1;
   /* 
    loop {
        if counter ==  5 {
            println!("coungrats, you've won!");
            break;
        }
        println!("Enter cooridate x: ");
        let mut input_x = String::new();
        stdin().read_line(&mut input_x);
        let trimmed_x = input_x.trim();
        match trimmed_x.parse::<u32>(){
            Ok(i) => xx = i,
            Err(..) => println!("This was not an integer: {}", trimmed_x)
        };

        println!("Enter cooridate y: ");
        let mut input_y = String::new();
        stdin().read_line(&mut input_y);
        let trimmed_y = input_y.trim();
        match trimmed_y.parse::<u32>(){
            Ok(i) => yy = i,
            Err(..) => println!("This was not an integer: {}", trimmed_y)
        };

        if mine_field.find_by_coordinates(xx,yy)  {
            println!("The game is over:(");
            break;
        }
        counter = counter + 1;
    }
    println!("The mines are marked with o!");

    &mine_field.print_fields_solved();

    */
    let mut c = 123456789;
    println!("c = {} size = {} bytes", c, mem::size_of_val(&c));
    c = -1;
    println!("c = {} after modification", c);
    let z:isize = 123; //isize/usize
    let size_of_z = mem::size_of_val(&z);
    println!("z = {}, takes up {} bytes, {}-bit-os", z, size_of_z, size_of_z * 8);

    let d:char = 'x';
    println!("d = {} size = {} bytes", d, mem::size_of_val(&d));

    let e = 2.5; //double-presision, 8 bytes or 64 bites, f64
    println!("e = {} size = {} bytes", e, mem::size_of_val(&e));

    let g = false;
    println!("g = {} size = {} bytes", g, mem::size_of_val(&g));
    operators();

}

fn operators() {
   let a = 1000;
   let a_cubed = i32::pow(a, 3); 
   println!("{}", a_cubed);
   let b = 2.5;
   let b_cubed = f64::powi(b,3);
   println!("{}", b_cubed);

   let b_to_pi = f64::powf(b, std::f64::consts::PI);
   println!("{}", b_to_pi);
   println!("{}", b * 2.0);
   println!("{}",f64::powf(b, 2.0));

   //bitwise
   let c = 1 | 2; // | RO & AND ^ XOR ! NOR
                  // 01 OR 10 = 11 = 3_10
   println!("{}", c);

   let tow_to_10 = 1 << 10; // >> 
   println!("{}", tow_to_10);

   //logical
   let pi_less_4 = std::f64::consts::PI < 4.0;//true
   println!("{}", pi_less_4);

   let x = 3;
   let x_is_f = x == 5;
   println!("{}", x_is_f);

   //scope and shadowing
   
   scop_and_shadowing();

   unsafe{
        Z = 888;
       println!("{}", Z);
   }

}
fn scop_and_shadowing() {
    let a = 123;
    let a = 456;
    {
        println!("inner a = {}", a);
        let a = 789;
        println!("inner a = {}", a);
        let b = 456;
        println!("b = {}", b);
    }
    println!("a = {}", a);
    println!("{}", MEANING_OF_LIFE);
    unsafe
    {
        Z = 777;
        println!("{}", Z);
    }
}
