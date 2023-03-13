mod chunk;
mod compiler;
mod debug;
mod op_code;
mod scanner;
mod source;
mod token;
mod vm;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    match args.len() {
        1 => source::repl(),
        2 => source::file(&args[1]),
        _ => {
            eprintln!("Usage: ./{} [path]", args[0]);
            std::process::exit(64);
        }
    }
}
