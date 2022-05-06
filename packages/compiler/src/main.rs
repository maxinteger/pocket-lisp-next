use std::io::BufRead;
use std::{env, fs, io, result};
use wasmtime::{Engine, Instance, Module, Store};

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut vm = VirtualMachine::new();

    match args.len() {
        1 => {
            repl(&mut vm);
        }
        2 => {
            run_file(&mut vm, args[1].as_str());
        }
        _ => {
            println!("Usage: rlox [path]");

            std::process::exit(64);
        }
    }
}

fn repl(vm: &mut VirtualMachine) {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    loop {
        print!("> ");
        if let Ok(line) = lines.next().unwrap() {
            interpret(vm, line.as_str());
        } else {
            break;
        }
    }
}

fn run_file(vm: &mut VirtualMachine, path: &str) {
    if let Ok(source) = fs::read_to_string(path) {
        match interpret(vm, source.as_str()) {
            InterpretResult::Ok => {}
            InterpretResult::CompileError => {
                eprintln!("Compilation error");
                std::process::exit(65);
            }
            InterpretResult::RuntimeError => {
                eprintln!("Runtime error");
                std::process::exit(70);
            }
        }
    } else {
        eprintln!("Could not open file '{}'", path);
        std::process::exit(64);
    }
}

fn interpret(vm: &mut VirtualMachine, source: &str) -> InterpretResult {
    let mut scanner = Scanner::new(source);
    let mut parser = Parser::new(&mut scanner, &mut vm.chunks);
    let parse_result = parser.parse();

    if !parse_result {
        return InterpretResult::CompileError;
    }

    vm.run()
}

fn invoke_wasm_module(module_name: String) -> result::Result<String, wasmtime_wasi::Error> {
    let engine = Engine::default();
    let module = Module::from_file(&engine, module_name)?;
    let mut store = Store::new(&engine, ());
    let instance = Instance::new(&mut store, &module, &[])?;
    let exported_run = instance.get_typed_func::<(), i32, _>(&mut store, "run")?;
    let res = exported_run.call(&mut store, ())?;
    Ok(res.to_string())
}
