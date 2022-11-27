use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let help = r"hxdc - Convert a hex number to decimal
Usage: hxdc <number>";

    if args.len() < 2 {
        println!("{}\n", help);
        return;
    }

    if let Ok(number) = usize::from_str_radix(&args[1], 16) {
        println!("{}", number);
    } else {
        println!("{}\n", help);
    }
}
