use nix::sys::ptrace;
use nix::sys::wait::{waitpid, WaitStatus};
use nix::unistd::{fork, ForkResult, execv};
use std::ffi::CString;
use std::io::{self, BufRead};
use::std::collections::HashMap;
use:: std::fs::File;
use std::env;
use std::process;


fn syscall_map() -> HashMap<u64, String> {

    let mut map = HashMap::new();
    let path = "syscalls.txt";
    let file = File::open(path).expect("No se pudo abrir syscalls.txt");
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let syscall_info = line.expect("Error al leer línea");
        let parts: Vec<&str> = syscall_info.split_whitespace().collect();
        map.insert(parts[0].parse::<u64>().expect("NR inválido"), parts[1].to_string());
    }
    map
}


fn main() {

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 || (args[1] != "-v" && args[1] != "-V") {
        println!("Comando mal escrito");
        process::exit(1);
    }

    let binary = CString::new(args[2].to_string()).unwrap();
    let syscall_names = syscall_map();
    let mut syscalls_count: HashMap<u64, u64> = HashMap::new();
    
    match unsafe { fork() } {
        Ok(ForkResult::Child) => {
            
            ptrace::traceme().expect("traceme falló");
            execv(&binary, &[binary.clone()]).expect("execv falló");

        }
        Ok(ForkResult::Parent { child }) => {
            loop {
                
                match waitpid(child, None).expect("waitpid falló") {
                    
                    WaitStatus::Exited(_, code) => {
                        println!("Proceso terminado con código {}", code);
                        break;
                    }
                    WaitStatus::Signaled(_, signal, _) => {
                        println!("Proceso terminó por señal {:?}", signal);
                        break;
                    }
                    WaitStatus::Stopped(_, _) => {
                       
                        let regs = ptrace::getregs(child).expect("getregs falló");
                        let value = syscalls_count.entry(regs.orig_rax).or_insert(0);
                        *value += 1;

                        println!("Syscall: {}", syscall_names.get(&regs.orig_rax).unwrap_or(&"desconocido".to_string()));
                        println!("NR: {}", regs.orig_rax);
                        println!("rdi: {:#x}", regs.rdi);
                        println!("rsi: {:#x}", regs.rsi);
                        println!("rdx: {:#x}", regs.rdx);
                        println!("r10: {:#x}", regs.r10);
                        println!("r8: {:#x}", regs.r8);
                        println!("r9: {:#x}", regs.r9);
                        println!("Total calls: {}", *value);

                        if args[1] == "-V" { let _ = io::stdin().read_line(&mut String::new()); }
                        ptrace::syscall(child, None).expect("ptrace syscall falló");
                        if args[1] == "-v" { println!(""); }
                    }
                    _ => {}
                }
            }
            for key in syscalls_count.keys() {
                println!("Syscall: {}", syscall_names.get(key).unwrap());
                println!("NR: {}", key);
                println!("Total Calls: {}", syscalls_count.get(key).unwrap());
                println!("");
            }
        }
        Err(e) => panic!("fork falló: {}", e)
    }

}

//cargo new nombreProyecto
//cargo run
