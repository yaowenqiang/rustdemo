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
fn how_many(x:i32) -> &'static str
{
    match x 
    {
        0 => "no",
        1 | 2 => "one or two",
        z @ 9...11 => "lots of ",
        12 => "a dozen",
        _ if (x % 2 == 0 )=> "Some",
        _ => "a few"

    }
}
pub fn pattern_match() 
{
    for x in 0..13 
    {
        println!("{}: I have {} oranges", x, how_many(x));
    }
    //let point = (3,4);
    //let point = (0,4);
    let point = (1,0);
    match point {
        (0,0) => println!("origin"),
        (0,y) => println!("x axis, y = {}", y),
        //(ref mut x,0) => println!("y axis, x = {}", x),
        (x,0) => println!("y axis, x = {}", x),
        (x,y) => println!("({}, {})", x,y)
    }

    let c:Color = Color::Cmyk{cyan:0, magenta:128, yellow:1, black: 255};
    match c {
        Color::Red => println!("R"),
        Color::Green => println!("G"),
        //_ => println!("some other color")
        Color::Blue => println!("B"),
        Color::RgbColor(0,0,0) => println!("Black"),
        //Color::Cmyk{cyan:_, magenta:_, yellow:_, black: 255} => println!("black"),
        Color::Cmyk{black: 255,..} => println!("black"),
        Color::RgbColor(r,g,b) => println!("rgb({}, {}, {})", r, g, b),
        _ => println!("some other color")
    }
}
