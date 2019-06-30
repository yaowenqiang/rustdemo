mod my;
extern crate some_crate;
//use some_crate::User;


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
}
