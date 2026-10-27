use std::env;
use std::fs;

fn main() {
    println!("Info about your system:");

    let username = env::var("USER").unwrap();
    println!("  Username: {username}");

    let os_name = fs::read_to_string("/etc/os-release").unwrap();
    let mut os = String::new();
    for line in os_name.lines() {
        if line.starts_with("NAME=") {
            os = line.replace("NAME=", "").replace("\"", "").trim().to_string();
            break;
        }
    }
    println!("  OS: {os}");

    let cpuinfo = fs::read_to_string("/proc/cpuinfo").unwrap();
    let mut cpu = String::new();
    for line in cpuinfo.lines() {
        if line.starts_with("model name") {
            cpu = line.replace("model name", "").replace(":", "").trim().to_string();
            break;
        }
    }
    println!("  CPU: {cpu}");

    let meminfo = fs::read_to_string("/proc/meminfo").unwrap();
    let mut mem = String::new();
    for line in meminfo.lines() {
        if line.starts_with("MemTotal:") {
            mem = line.replace("MemTotal:", "").trim().to_string();
            break;
        }
    }
    println!("  Memory: {mem}");
}
