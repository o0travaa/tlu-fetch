use std::env;
use std::fs;

fn main() {
    println!("Info about your system:");

    // Username
    let username = match env::var("USER") {
        Ok(un_value) => {
            un_value.trim().to_string()
        }
        Err(_) => {
            "Unknown".to_string()
        }
    };

    // Hostname
    let hostname = match fs::read_to_string("/etc/hostname") {
        Ok(hn_value) => {
            hn_value.trim().to_string()
        }
        Err(_) => {
            "Unknown".to_string()
        }
    };

    println!("{username}@{hostname}");
    println!("---------------------");

    // OS
    let os_name = match fs::read_to_string("/etc/os-release") {
        Ok(os_value) => {
            os_value
        }
        Err(_) => {
            "".to_string()
        }
    };
    let mut os = String::new();
    for line in os_name.lines() {
        if line.starts_with("PRETTY_NAME=") {
            os = line.replace("PRETTY_NAME=", "").replace("\"", "").trim().to_string();
            break;
        }
    }
    if os.is_empty() {
        os = "Unknown".to_string()
    }
    println!("OS: {os}");

    // Kernel Version
    let kernel = match fs::read_to_string("/proc/sys/kernel/osrelease") {
        Ok(kernel_value) => {
            kernel_value.trim().to_string()
        }
        Err(_) => {
            "Unknown".to_string()
        }
    };
    println!("Kernel: {kernel}");

    // CPU
    let cpuinfo = match fs::read_to_string("/proc/cpuinfo") {
        Ok(cpu_value) => {
            cpu_value
        }
        Err(_) => {
            "".to_string()
        }
    };
    let mut cpu = String::new();
    for line in cpuinfo.lines() {
        if line.starts_with("model name") {
            cpu = line.replace("model name", "").replace(":", "").trim().to_string();
            break;
        }
    }
    if cpu.is_empty() {
        cpu = "Unknown".to_string()
    }
    println!("CPU: {cpu}");

    // Shell
    let shell = match env::var("SHELL") {
        Ok(shell_value) => {
            shell_value.split("/").last().unwrap_or("Unknown").to_string()
        }
        Err(_) => {
            "Unknown".to_string()
        }
    };
    println!("Shell: {shell}")
}
