mod structs;
mod sh;
mod pm;
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
    println!("hello world");
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
    while_and_loop();
    enums();
    unions();
    option();
    arrays_fn();
    vectors();
    slices();
    strings();
    tuples();
    pm::pattern_match();
    generics();
    functions();
    methods();
    closures();
    hof();
    traits();
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

   let tmp = 20;
   let day = if tmp > 20 {"sunny"}  else {"cloudy"};
   println!("{}", day);

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
    sh::stack_and_heap();
}
fn while_and_loop()
{
    let mut x = 1;
    while x < 1000 {
        x *= 2;
        if x == 64 {
            continue;
        }
        println!("x = {}", x);
    }

    let mut y = 1;
    loop {
        y *= 2;
        println!("y = {}", y);
        if y == 1<<10 {break;}
    }
    for_loop();
    match_statement();
    structures();
    lines();
}

fn for_loop()
{
    for x in 1..11 {

        if x == 5 { continue;}

        if x == 8 { break;}

        println!("x = {}",x);
    }

    for (pos, y) in (30..41).enumerate()
    {
        println!("{} => {}", pos, y);
    }

}
fn match_statement()
{
    let country_code = 1000;
    let country = match country_code 
    {
        44 => "UK",
        46 => "Sweden",
        7 => "Russia",
        1...999 => "unknown",  // 999 is included
        _ => "invalid"
    };
    println!("The country with code {} is {}", country_code, country);
}
fn structures ()
{
    let p = Point {
        x:3.0,
        y:4.0
    };

    let p2 = Point {
        x:10.0,
        y:20.0
    };
    println!("point p is a  ({},{})",p.x, p.y);
}

fn lines() {
    let p = Point {
        x:3.0,
        y:4.0
    };

    let p2 = Point {
        x:10.0,
        y:20.0
    };
    let myline = Line {
        start: p,
        end: p2
    };

    println!("myline  starts at  ({},{}), ends at ({},{})",myline.start.x, myline.start.y, myline.end.x, myline.end.y);



}
struct Point {
    x: f64,
    y: f64
}


struct Line {
    start: Point,
    end: Point
}

enum Color {
    Red,
    Green,
    Blue,
    RgbColor(u8, u8, u8), //tuple
    Cmyk {
        cyan: u8,
        magenta:u8,
        yellow: u8,
        black: u8
    }, //struct
}
fn enums() {
    //let c:Color = Color::Red;
    //let c:Color = Color::RgbColor(0,0,0);
    //let c:Color = Color::RgbColor(255,255,255);
    let c:Color = Color::Cmyk{cyan:0, magenta:128, yellow:1, black: 255};
    match c {
        Color::Red => println!("R"),
        Color::Green => println!("G"),
        //_ => println!("some other color")
        Color::Blue => println!("B"),
        Color::RgbColor(0,0,0) => println!("Black"),
        Color::Cmyk{cyan:_, magenta:_, yellow:_, black: 255} => println!("black"),
        Color::RgbColor(r,g,b) => println!("rgb({}, {}, {})", r, g, b),
        _ => println!("some other color")
    }

}

union IntOrFloat {
    i: i32,
    f: f32
}


fn process_value(iof: IntOrFloat)
{
    unsafe
    {
        match iof {
            IntOrFloat { i:42} =>  {println!("meaning of life");},
            IntOrFloat { f} =>  {println!("f32 = {}", f);},
        }
    }
}
fn unions () {
    let mut iof = IntOrFloat{
        i:32
    };
    unsafe {
        iof.i = 42;
    }
    let mut iof2 = IntOrFloat{
        f:32.0
    };
    unsafe {
        iof2.f = 42.0;
    }
    process_value(iof);
    process_value(iof2);
}

fn option ()
{
    //Option<T>
    let x = 3.0;
    //let y = 2.0;
    let y = 0.0;

    // Some(z) None)
    
    let result:Option<f64> = 
        if y != 0.0 {Some(x/y)} else {None};
    println!("{:?}", result);
    match result {
        Some(z) => println!("{}/{} = {}", x,y,z),
        None => println!("can not divide {} by {} ", x,y)
    }
    // if let / while let
    if let Some(z) = result{println!("z = {}", z)};
}
fn arrays_fn()
{
    let mut a:[i32;5] = [1,2,3,4,5];
    let mut b         = [1,2,3,4,5];
    println!("a has {} elements, first is {}", a.len(), a[0]);
    a[0] = 123;
    println!("a has {} elements, first is {}", a.len(), a[0]);
    println!("{:?}",a);
    if a != [1,2,3,4,5] {
        println!("does not match");
    }

    let b = [1; 10];
    let c = [1u16; 10];
    println!("{:?}",b);
    for i in 0..b.len()
    {
        println!("b[{}] = {}", i, b[i]);
    }

    println!("b took up {} bytes ", mem::size_of_val(&b) );
    println!("c took up {} bytes ", mem::size_of_val(&c) );

    let mtx:[[f32;3];2] = [
        [1.0,0.0,0.0],
        [0.0,2.0,3.0]
    ];
    println!("{:?}", mtx);

    for i in 0..mtx.len()
    {
        for j in 0..mtx[i].len() {
            if i == j {
                println!("mtx[{}][{}] = {}",i ,j , mtx[i][j]);
            }
        }
    }

}

fn vectors () {
    //usize isize
    //let idx:u32 = -1;
    //let idx:u32 = 1;
    let idx:usize = 0;
    let mut a = Vec::new();
    a.push(1);
    a.push(2);
    a.push(3);
    println!("a = {:?}",a );
    a.push(44);
    a[idx] = 34343;
    println!("a = {:?}",a );
    println!("a[0] = {}",a[0] );
    println!("a[0] = {}",a[idx] );

    //return Option type
    match a.get(6) {
        Some(x) => println!("a[6] = {}",x),
        None => println!("error, No such element")
    }

    for x in &a {
        println!("{}", x);
    }
    a.push(44);
    for x in &a {
        println!("{}", x);
    }

    let last_elem = a.pop();
    println!("last eleme is {:?}, a = {:?}", last_elem, a);

    while let Some(x) = a.pop()
    {
        println!("{}", x);
    }
    println!("last eleme is {:?}, a = {:?}", last_elem, a);
}

/*fn use_slice (slice:&mut [i32]) {

}*/

fn use_slice (slice: &mut [i32]) 
{    //& means borrow
    println!("first elem = {}, len = {}", slice[0], slice.len());
    slice[0] = 100;
    println!("first elem = {}, len = {}", slice[0], slice.len());
}

fn slices ()
{
    let mut data = [1,2,3,4,5];
    //use_slice(&mut data[1..4]);
    use_slice(&mut data);
    println!("{:?}", data);

}

fn strings ()
{

    let s:&'static str = "hello你好"; // &str = string slice
    // s = "abc你好";

    for s in s.chars() {
        println!("{}", s);
    }

    for s in s.chars().rev() {
        println!("{}", s);
    }

    let c = "a";
    let d = "你";
    println!("size of c {}: {}", c, mem::size_of_val(c));
    println!("size of d {} : {}", d, mem::size_of_val(c));

    if let Some(first_letter) = s.chars().nth(0) {
        println!("first letter is {}", first_letter);
    }

    //String

    let mut letters = String::new();
    let mut a = 'a' as u8;
    while a <= ('z' as u8) {
        letters.push(a as char);
        letters.push_str(",");
        a += 1;
    }

    println!("{:?}", letters);

    //str <> String

    //let u:&'str = &letters;

    //concatencation
    //String + str
    let z = letters + "abc";

    println!("{}", z);
    //let zz = letters + letters;
    //println!("{}", zz);

    let mut abc = String::from("Hello World");
    let mut def = "Hello world".to_string();
    abc.remove(0);
    println!("{}", abc);
    abc.push_str("!!!");
    println!("{}",abc.replace("ello","goodbye"));

}

fn sum_and_product(x:i32, y:i32) -> (i32, i32)
{
    (x + y, x * y)
}

fn tuples ()
{
    let x = 3;
    let y = 4;
    let sp = sum_and_product(x, y);
    println!("{:?}", sp);
    println!("{0} +  {1} =  {2}, {0} * {1} = {3}", x, y, sp.0, sp.1);

    //Destructing

    let (a, b) =  sp;
    println!("{0} +  {1} =  {2}, {0} * {1} = {3}", x, y, a, b);

    let sp2 = sum_and_product(4,7);

    let combined = (sp, sp2);
    println!("{:?}", combined);


    println!("last elem = {}", (combined.1).1);

    let ((c,d),(e,f)) = combined;
    println!("{},{},{},{}", c,d,e,f);

    let foo = (true, 42.0, -1i8);
    println!("{:?}", foo);

    let single_tuple = (42);
    println!("{:?}", single_tuple);

    let single_tuple2 = (42, );
    println!("{:?}", single_tuple2);
}


struct Point2 {
    x: f64,
    y: f64 
}
#[derive(Debug)]
struct Point3<T> {
    x: T,
    y: T 
}

#[derive(Debug)]
struct Point4<T, V> {
    x: T,
    y: V 
}

#[derive(Debug)]
struct Point5<T> {
    x: T,
    y: T 
}
#[derive(Debug)]
struct Line2<T>
{
    start: Point5<T>,
    end: Point5<T>
}

fn generics() 
{
   //let a = Point3{x:0, y:0}; 
   //let a:Point3<i32> = Point3{x: 0, y:0 }; 
   //let a:Point3<u16> = Point3{x: 0, y:1 }; 
   let a:Point3<u16> = Point3{x: 0, y:1 }; 
   //let b:Point3<f64> = Point3{x:1.2, y:1.4}; 
   let b:Point3<f64> = Point3{x:1.2, y:1.4}; 

   let c = Point3{x: 10, y:11 }; 
   let d = Point3{x: 10.0, y:11.0 }; 

   let e:Point4<i32,f64> = Point4{x: 12, y:13.0 }; 
   let f:Point4<f64,i32> = Point4{x: 12.0, y:13 }; 
   println!("{:?}", a);
   println!("{:?}", b);
   println!("{:?}", c);
   println!("{:?}", d);
   println!("{:?}", e);
   println!("{:?}", f);

   //let g = Point5{x:0, y:4};
   //let h = Point5{x:0.0, y:4.0};
//   let g:Point5<f64> = Point5{x:1.0, y:4.0};
   let g:Point5<f64> = Point5{x:1.0, y:4f64};
   let h:Point5<f64> = Point5{x:0.0, y:4.0};
   let myline2 = Line2{start:g ,end: h};
   println!("{:?}", myline2);
}


fn increse(x:&mut i32)
{
    *x = 100;
}

fn product(x: i32, y: i32) -> i32
{
    x * y
}

fn print_value(x:i32)
{
    println!("Value =  {}", x);
    let mut z = 1;
    increse(&mut z);
    println!("Value =  {}", z);
}

fn functions ()
{
    print_value(2) ;   
    println!("product 3 and 4 = {}", product(3,4));
}

impl Line {
    fn len(&self) -> f64
    {
       let dx = self.start.x - self.end.x; 
       let dy = self.start.y - self.end.y; 
       (dx*dx +  dy * dy).sqrt()
    }
}
fn methods ()
{
    let p = Point{x:1f64, y:2f64};
    let p1 = Point{x:3f64, y:4f64};
    let myline = Line{start:p, end:p1};
    println!("length = {}", myline.len());
}


fn closures()
{
    let sh = say_hello;
    sh();
    let plus_one = | x:i32 |-> i32 {
        x + 1
    };

    let p = plus_one(1);
    println!("{}", p);
    let mut two = 2;
    {
        let plus_two = | x | {
            let mut z = x;
            z += two;
            z
        };
        println!("2 + 2 = {}", plus_two(2));
    }
    let borrow_two = &mut two;
    println!("borrow_two = {}" ,borrow_two);

    // T by value
    // T&
    // &mut

    let plus_three = | x: &mut i32 | *x += 3;
    /*
    let plus_three = | x: &mut i32 |  {
        *x += 3;
    };
    */
    let mut f = 12;
    plus_three(&mut f);
    println!("f = {}", f);
}

fn is_even (x: i32) -> bool
{
    x %2 == 0
}
// higher order function
fn hof()
{
    let limit = 500;
    let mut sum = 0;
    for i in 0..
    {
        let isq = i * i;
        if isq > limit {break;}
        else if  is_even(isq) { sum += isq; }

    }
    println!("the sum is {}", sum);

    let sum2 = 
        (0..).map(|x| x * x )
             .take_while(|&x| x < limit)
             .filter(|x| is_even(*x))
             .fold(0, |sum, x | sum + x);

    println!("the sum is {}", sum2);
}


trait Animal {
    fn create(name: &'static str) -> Self;
    fn name(&self) -> &'static str;
    fn talk(&self)
    {
        println!("{} cannot talk", self.name());
    }
}

struct Human {
    name: &'static str
}

impl Animal for Human
{
    fn create (name: &'static str) -> Human
    {
        Human{name: name}
    }

    fn name(&self) -> &'static str
    {
        self.name
    }
    fn talk(&self) 
    {
        println!("{} says hello",self.name());
    }
}
trait Summable <T>{
    fn sum(&self)  -> T;
}
impl Summable<i32> for Vec<i32>
{
    fn sum(&self) -> i32
    {
        let mut result:i32  = 0;
        for x in self
        {
            result += *x;
        }
        return  result;
    }

}
fn traits()
{
   let h = Human::create("john");
   let h2 = Human{name:"jack"};
   let h3:Human = Animal::create("haha");
   h.talk();
   h2.talk();
   h3.talk();

   let a = vec![1,2,3];
   println!("sum = {}", a.sum());

}
