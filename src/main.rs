use std::env;
use std::fs;

fn main() {
    println!("Info about your system:");

    let username = env::var("USER").unwrap();
    println!("  Username: {username}");

    let hostname = fs::read_to_string("/etc/hostname").unwrap().trim().to_string();
    println!("  Hostname: {hostname}");

    let os_name = fs::read_to_string("/etc/os-release").unwrap();
    let mut os = String::new();
    for line in os_name.lines() {
        if line.starts_with("NAME=") {
            os = line.replace("NAME=", "").replace("\"", "").trim().to_string();
            break;
        }
    }
    println!("  OS: {os}");

    let kernel = fs::read_to_string("/proc/sys/kernel/osrelease").unwrap().trim().to_string();
    println!("  Kernel: {kernel}");

    let cpuinfo = fs::read_to_string("/proc/cpuinfo").unwrap();
    let mut cpu = String::new();
    for line in cpuinfo.lines() {
        if line.starts_with("model name") {
            cpu = line.replace("model name", "").replace(":", "").trim().to_string();
            break;
        }
    }
    println!("  CPU: {cpu}");

    let shell = match env::var("SHELL") {
        Ok(shell_value) => {
            shell_value.split("/").last().unwrap_or("Unknown").to_string()
        }
        Err(_) => {
            "Unknown".to_string()
        }
    };
    println!("  Shell: {shell}")
}
