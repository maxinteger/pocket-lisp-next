use std::io::BufRead;
use std::{env, fs, io, result};
use wasmtime::{Engine, Instance, Module, Store};

mod emitter;
mod parser;
mod scanner;
mod token;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => {
            repl();
        }
        2 => {
            run_file(args[1].as_str());
        }
        _ => {
            println!("Usage: rlox [path]");

            std::process::exit(64);
        }
    }
}

fn repl() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    loop {
        print!("> ");
        if let Ok(line) = lines.next().unwrap() {
            println!("echo {}", line);
        } else {
            break;
        }
    }
}

fn run_file(path: &str) {
    if let Ok(source) = fs::read_to_string(path) {
        print!("File opened")
    } else {
        eprintln!("Could not open file '{}'", path);
        std::process::exit(64);
    }
}
//
// fn interpret(source: &str) -> InterpretResult {
//     let mut scanner = Scanner::new(source);
//     // let mut parser = Parser::new(&mut scanner, &mut vm.chunks);
//     // let parse_result = parser.parse();
//     //
//     // if !parse_result {
//     //     return InterpretResult::CompileError;
//     // }
//     //
//     // vm.run()
// }

fn invoke_wasm_module(module_name: String) -> result::Result<String, anyhow::Error> {
    let engine = Engine::default();
    let module = Module::from_file(&engine, module_name)?;
    let mut store = Store::new(&engine, ());
    let instance = Instance::new(&mut store, &module, &[])?;
    let exported_run = instance.get_typed_func::<(), i32, _>(&mut store, "run")?;
    let res = exported_run.call(&mut store, ())?;
    Ok(res.to_string())
}
