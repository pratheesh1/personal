use std::io::{Read, Write};

pub fn repl() {
    loop {
        print!("> ");
        std::io::stdout().flush().unwrap();

        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap_or_else(|e| {
            println!("Error: {}", e);
            std::process::exit(1);
        });

        if line.trim() == "exit" {
            break;
        }

        while !line.trim().ends_with(';') {
            std::io::stdout().flush().unwrap();

            line = line.trim().to_string() + " ";
            std::io::stdin().read_line(&mut line).unwrap_or_else(|e| {
                println!("Error: {}", e);
                std::process::exit(1);
            });
        }

        println!("You entered: {}", line);
    }
}

pub fn file(path: &str) {
    let mut file = std::fs::File::open(path).unwrap_or_else(|e| {
        println!("Error: {}", e);
        std::process::exit(74);
    });

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap_or_else(|e| {
        println!("Error: {}", e);
        std::process::exit(74);
    });

    println!("Running source code: {}", contents);
}
