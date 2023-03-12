mod chunk;
mod debug;
mod op_code;
mod source;
mod vm;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    match args.len() {
        1 => source::repl(),
        2 => source::file(&args[1]),
        _ => {
            println!("{}", format!("Usage: {} [path]", args[0]).to_string());
            std::process::exit(64);
        }
    }
}
