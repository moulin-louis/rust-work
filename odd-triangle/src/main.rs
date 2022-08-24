use std::env;

fn draw_line(lenght:i32) {
    let mut x = 0;
    while lenght != x
    {
        print!("*");
        x += 1;
    }
    println!("");
}

fn main() {
    if env::args().len() != 2 {
        return ; }
    let nbr = String::from(env::args().nth(1).expect("Give a nbr idiot"));
    let mut nbr:i32 = nbr.parse().unwrap();
    let mut x = 1;
    while nbr > 0
    {
        if x%2 == 1
        {
            draw_line(x);
        }
        nbr -= 1;
        x += 1;
    }
}