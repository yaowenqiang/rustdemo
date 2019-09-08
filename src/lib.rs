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
