use colored::*;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let c = if args.len() > 1 { args[1].as_str() } else { "" };

    if let Some((w, _)) = term_size::dimensions() {
        println!("{:-^1$}", "-".color(c), w);
    } else {
        println!("--------------------")
    }
}
