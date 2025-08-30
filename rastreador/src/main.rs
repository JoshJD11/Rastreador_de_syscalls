use nix::sys::ptrace;
use nix::sys::wait::{waitpid, WaitStatus};
use nix::unistd::{fork, ForkResult, execv};
use std::ffi::CString;
use std::io::{self, BufRead};
use::std::collections::HashMap;
use:: std::fs::File;
use std::env;


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
    let syscall_names = syscall_map();
    
}
