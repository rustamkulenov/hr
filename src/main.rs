use colored::*;
use std::env;
use std::io;

fn main() {

    let mut text = String::new();
    if !atty::is(atty::Stream::Stdin) {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        text.push_str(" ");
        text.push_str(input.trim());
        text.push_str(" ");
    }

    let text_w = text.len();

    let args: Vec<String> = env::args().collect();
    let color = if args.len() > 1 { args[1].as_str() } else { "" };

    if let Some((w, _)) = term_size::dimensions() {
        let w1 = (w - text_w) / 2;
        print!("{:-^1$}", "-".color(color), w1);
        print!("{}", text.color(color));
        println!("{:-^1$}", "-".color(color), w - text_w - w1);
    } else {
        println!("---------- {} ----------", text)
    }
}
