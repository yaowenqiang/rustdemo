pub fn show () {
    let number_list = vec![34,50, 25, 100,65];
    let mut largest_number = number_list[0];

    for number in number_list {
        if number > largest_number {
            largest_number = number;
        }
    }
    
    println!("The largest number is {}", largest_number);
    assert_eq!(largest_number, 100);

    let number_list2 = vec![34,50, 25, 100,65];
    let largest_number2 =  largest(&number_list2);
    //let largest_number3 =  largest1(&number_list2);
    println!("largest_number2 {}", largest_number2);
    //println!("largest_number3 {}", largest_number3);

    let char_list = vec!['y','m','a','q'];
    let result = largest_char(&char_list);
    //let result1 = largest1(&char_list);
    println!("result1: The largest char is {} ", result);
    //println!("result2: The largest char is {} ", result1);

    let integer = Point{x:5, y:10};
    let float = Point{x:1.0, y:4.0};

}

pub fn largest(list: &[i32]) -> i32 {
    let mut larget_number = list[0];
    for &item in list.iter() {
        if item > larget_number {
            larget_number = item;
        }
    }
    larget_number
}

pub fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

/*
pub fn largest1<T>(list: &[T]) -> T {
    //let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    //largest
    //&list[0]
}*/

pub struct Point<T> {
    pub x:T,
    pub y:T,
}
impl<T> Point<T> {
    pub fn x(&self) -> &T
    {
        &self.x
    }

    pub fn y(&self) -> &T
    {
        &self.y
    }
}
