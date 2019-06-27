#[derive(Debug)]
pub struct User {
    pub name: String,
    pub email: String,
    pub age: i16,
    pub user_type: UserType
}

impl User{
    pub fn print_user(self) {
        println!("haha,The name of the user is {}.\n", self.name);
        println!("HIs email is  {}.\n", self.email);
        println!("He is {} years old.\n", self.age);
    }
}


#[derive(Debug)]
pub enum UserType{
    Guest,
    Regular,
    Admin
}
