use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let help = r"dchx - Convert a decimal number to hex
Usage: dchx <number>";

    if args.len() < 2 {
        println!("{}\n", help);
        return;
    }

    if let Ok(number) = args[1].parse::<usize>() {
        println!("{:X}", number);
    }
}
