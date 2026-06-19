use owo_colors::OwoColorize;
use std::{env, fs, process::Command};

fn main() {
    let username: String = if let Ok(user) = env::var("USER") {
        user
    } else {
        "unknown".to_string()
    };
    let hostname: String = fs::read_to_string("/etc/hostname")
        .expect("Found file!")
        .trim()
        .to_string();
    let length: u32 = (username.len() as u32) + (hostname.len() as u32) + 1;
    let contents: String = fs::read_to_string("/etc/os-release").expect("Found file!");
    let mut os: String = "Linux".to_string();
    for line in contents.lines() {
        if line.starts_with("PRETTY_NAME") {
            os = line.replace("PRETTY_NAME=", "").replace('"', "");
        }
    }
    let mut kernel: String = "".to_string();
    for (i, word) in fs::read_to_string("/proc/version")
        .expect("Found file!")
        .split(" ")
        .enumerate()
    {
        if i < 3 {
            kernel.push_str(word);
            kernel.push(' ');
        }
    }
    let mut uptime: f32 = fs::read_to_string("/proc/uptime")
        .expect("Found file!")
        .split(" ")
        .next()
        .unwrap()
        .parse::<f32>()
        .unwrap();
    let u_hours: String = calc_time(&mut uptime, 3600.0, "hour");
    let u_minutes: String = calc_time(&mut uptime, 60.0, "minute");
    let u_seconds: String = calc_time(&mut uptime, 1.0, "second");
    let memory: String = fs::read_to_string("/proc/meminfo").expect("Found file!");
    let mut total_mem: f32 = 0.0;
    let mut available_mem: f32 = 0.0;
    for line in memory.lines() {
        if line.starts_with("MemTotal") {
            total_mem = line
                .replace("MemTotal:", "")
                .replace("kB", "")
                .trim()
                .parse()
                .unwrap();
        } else if line.starts_with("MemAvailable") {
            available_mem = line
                .replace("MemAvailable:", "")
                .replace("kB", "")
                .trim()
                .parse()
                .unwrap();
        }
    }
    let mut used_mem: f32 = total_mem - available_mem;
    let mut mem_unit: String = "KB".to_string();
    if used_mem > 512000.0 {
        used_mem /= 1048576.0;
        total_mem /= 1048576.0;
        mem_unit = "GB".to_string();
    } else if used_mem > 1024.0 {
        used_mem /= 1024.0;
        total_mem /= 1024.0;
        mem_unit = "MB".to_string();
    }
    let cpu: String = fs::read_to_string("/proc/cpuinfo").expect("Found file!");
    let mut cpu_model: String = "Unknown".to_string();
    for line in cpu.lines() {
        if line.starts_with("model name") {
            cpu_model = line
                .replace("model name", "")
                .replace(':', "")
                .trim()
                .to_string();
        }
    }
    let shell: String = if let Ok(stat) = fs::read_to_string("/proc/self/stat") {
        let values: Vec<&str> = stat.split_whitespace().collect();
        if values.len() > 3 {
            fs::read_to_string(format!("/proc/{}/comm", values[3]))
                .expect("Found file!")
                .replace("\n", "")
        } else {
            "Unknown".to_string()
        }
    } else {
        "Unknown".to_string()
    };
    let packages: u32 = count_packages();
    let gpu: String = get_gpu();

    print!("\n{}@{}\n", username.green(), hostname.green());
    for _ in 0..length {
        print!("-");
    }
    println!("\n{}: {}", "OS".green(), os.green());
    println!("{}: {}", "Kernel".green(), kernel.green());
    println!(
        "{}: {}{}{}",
        "Uptime".green(),
        u_hours.green(),
        u_minutes.green(),
        u_seconds.green()
    );
    println!(
        "{}: {} {} / {} {} ({}%)",
        "Memory".green(),
        ((used_mem * 100.0).round() / 100.0).green(),
        mem_unit,
        ((total_mem * 100.0).round() / 100.0).green(),
        mem_unit,
        ((used_mem / total_mem * 1000.0).round() / 10.0).green()
    );
    println!("{}: {}", "CPU".green(), cpu_model.green());
    println!("{}: {}", "GPU".green(), gpu.green());
    println!("{}: {}", "Shell".green(), shell.green());
    println!("{}: {} (apt)", "Packages".green(), packages.green());
    println!("");
}

fn calc_time(uptime: &mut f32, multiplier: f32, name: &str) -> String {
    if *uptime >= multiplier {
        let time = (*uptime / multiplier).floor();
        *uptime -= multiplier * time;
        format!("{} {}{} ", time, name, if time == 1.0 { "" } else { "s" })
    } else {
        "".to_string()
    }
}

fn count_packages() -> u32 {
    if let Ok(packages) = Command::new("dpkg-query")
        .args(["-f", "'${binary:Package}\n'", "-W"])
        .output()
    {
        if packages.status.success() {
            return String::from_utf8_lossy(&packages.stdout).lines().count() as u32 - 1;
        } else {
            0
        }
    } else {
        0
    }
}

fn get_gpu() -> String {
    if let Ok(lspci) = Command::new("lspci").output() {
        if lspci.status.success() {
            let specs = String::from_utf8_lossy(&lspci.stdout);
            let mut result: String = "".to_string();
            for line in specs.lines() {
                if line.to_lowercase().contains("vga") || line.to_lowercase().contains("3d") {
                    if let Some(text) = line.find("controller:") {
                        result = line[text + 11..].trim().to_string()
                    } else {
                        result = line.to_string();
                    }
                }
            }
            result
        } else {
            "Unknown".to_string()
        }
    } else {
        "Unknown".to_string()
    }
}
