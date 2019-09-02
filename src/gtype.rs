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
    println!("largest_number2 {}", largest_number2);
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
