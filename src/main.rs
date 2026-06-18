use owo_colors::OwoColorize;
use std::{env, fs};

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

    print!("\n{}@{}\n", username.green(), hostname.green());
    for _ in 0..length {
        print!("-");
    }
    println!("\n{}: {}", "OS".green(), os.green());
    println!("{}: {}", "Kernel".green(), kernel.green());
    println!(
        "{}: {} {} {}",
        "Uptime".green(),
        u_hours.green(),
        u_minutes.green(),
        u_seconds.green()
    );
}

fn calc_time(uptime: &mut f32, multiplier: f32, name: &str) -> String {
    if *uptime >= multiplier {
        let time = (*uptime / multiplier).floor();
        *uptime -= multiplier * time;
        format!("{} {}{}", time, name, if time == 1.0 { "" } else { "s" })
    } else {
        "".to_string()
    }
}
