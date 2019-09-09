pub trait Summary {
    //fn summarize(&self) -> String;

    fn summarize(&self) -> String {
        //String::from("Read more...")
        format!("(Read more from {})...", self.summarize_author())
    }
    fn summarize_author(&self) ->String;
}

#[derive(Debug)]
pub struct NewArticle {
    pub headline: String, pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewArticle  {
    fn summarize(&self) -> String 
    {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }

    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String, 
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

pub fn notify (item: impl Summary) {
    println!("Breaking news! {}", item.summarize())
}

//trait bound
//
pub fn notify2<T: Summary>(item: T) {
    println!("breaking news! {}", item.summarize())
}

pub fn notify3(item: impl Summary + Display)
{

}


pub fn notify4<T: Summary + Display>(item: T) {

}

pub fn some_function<T: Display + Clone, U: Clone + Debug>(t: T, u:U) -> i32 {
    
}
fn some_function2<T,U>(t:T, u:U) -> i32 
    where T: Display + Cone,
          U: Clone + Debug
{

}


pub fn return_summarizeable() -> impl Summary {
    Tweet {
        username:  String::from("horse_ebooks"),
        content: String::from("of course ,as you probably already know, people"),
        replay: false,
        retweet: false,
    }
}

fn return_summarizeable2(switch: bool) -> impl Summary {
   if switch {
       NewArticle {
           headline: String::from("headline"),
           location: String::from("location"),
           author: String::from("author"),
           content: String::from("content"),
       }
   }  else {
       Tweet {
           username: String::from("username"),
           content: String::from("content"),
           replay: false,
           retweet: false,
       }

   }
}

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
   let mut largest = list[0];

   for &item  in list.iter() {
       if item > largest {
           largest = item;
       }
   }
}

pub fn largest_test_demo() {
    let number_list = vec![34, 50, 25, 100,60];
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y','m','a','q'];

    let result = largest(&char_list);
    println!("The largest number is {}", result);



}
