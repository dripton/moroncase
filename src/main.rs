/// Transform a string into alternating case, used for making fun of
/// statements you disagree with on the Internet.
///
/// Usage: ./moroncase Phantom Menace was the best Star Wars movie
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("Usage: {} string_to_moronify", args[0]);
        process::exit(1);
    }
    let mut upper_next = true;
    for ii in 1..args.len() {
        let arg = &args[ii];
        for ch in arg.chars() {
            if upper_next {
                print!("{}", ch.to_uppercase());
                upper_next = false;
            } else {
                print!("{}", ch.to_lowercase());
                upper_next = true;
            }
        }
        print!(" ");
    }
    print!("\n");
}
