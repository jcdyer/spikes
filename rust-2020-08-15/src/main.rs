use chainable_if::if_chain;

fn main() {
    let x = 14;
    let this_is_the_variable_i_want_to_expand: u8 = rand::random();
    if_chain!{
        | x < 127 => println!("Ascii {}", this_is_the_variable_i_want_to_expand),
        | x > 128 => println!("Not-ascii"),
    }
    println!("Hello, world!");
}
