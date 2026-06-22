use owo_colors::OwoColorize;
use std::{env, ffi::CString, fs, mem, net::UdpSocket, process::Command};


fn main() {
    let spacing: &str = "     ";
    let mut current_logo = LINUX_LOGO;
    let username: String = if let Ok(user) = env::var("USER") {
        user
    } else {
        "unknown".to_string()
    };
    let hostname: String = fs::read_to_string("/etc/hostname")
        .expect("File not found, Slowfetch is only available for Linux devices")
        .trim()
        .to_string();
    let length: u32 = (username.len() as u32) + (hostname.len() as u32) + 1;
    let contents: String = fs::read_to_string("/etc/os-release")
        .expect("File not found, Slowfetch is only available for Linux devices");
    let mut os: String = "Linux".to_string();
    for line in contents.lines() {
        if line.starts_with("PRETTY_NAME") {
            os = line.replace("PRETTY_NAME=", "").replace('"', "");
        }
    }
    if os.to_lowercase().contains("debian") {
        current_logo = DEBIAN_LOGO;
    } else if os.to_lowercase().contains("arch") {
        current_logo = ARCH_LOGO;
    } else if os.to_lowercase().contains("fedora") {
        current_logo = FEDORA_LOGO;
    } else if os.to_lowercase().contains("ubuntu") {
        current_logo = UBUNTU_LOGO;
    }
    let mut kernel: String = "".to_string();
    for (i, word) in fs::read_to_string("/proc/version")
        .expect("File not found, Slowfetch is only available for Linux devices")
        .split(" ")
        .enumerate()
    {
        if i < 3 {
            kernel.push_str(word);
            kernel.push(' ');
        }
    }
    let mut uptime: f32 = fs::read_to_string("/proc/uptime")
        .expect("File not found, Slowfetch is only available for Linux devices")
        .split(" ")
        .next()
        .unwrap()
        .parse::<f32>()
        .unwrap();
    let u_hours: String = calc_time(&mut uptime, 3600.0, "hour");
    let u_minutes: String = calc_time(&mut uptime, 60.0, "minute");
    let u_seconds: String = calc_time(&mut uptime, 1.0, "second");
    let memory: String = fs::read_to_string("/proc/meminfo")
        .expect("File not found, Slowfetch is only available for Linux devices");
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
    let cpu: String = fs::read_to_string("/proc/cpuinfo")
        .expect("File not found, Slowfetch is only available for Linux devices");
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
                .expect("File not found, Slowfetch is only available for Linux devices")
                .replace("\n", "")
        } else {
            "Unknown".to_string()
        }
    } else {
        "Unknown".to_string()
    };
    let packages: u32 = count_packages();
    let gpu: String = get_gpu();
    let fallback: String = "127.0.0.1".to_string();
    let offline: String = "Offline".to_string();
    let private_ip: String = if let Ok(socket) = UdpSocket::bind("0.0.0.0:0") {
        if socket.connect("8.8.8.8:80").is_ok() {
            if let Ok(address) = socket.local_addr() {
                address.ip().to_string()
            } else {
                fallback
            }
        } else {
            fallback
        }
    } else {
        fallback
    };
    let mut public_ip: String =
        if let Ok(output) = Command::new("curl").args(["-s", "ifconfig.me"]).output() {
            if output.status.success() {
                String::from_utf8_lossy(&output.stdout).trim().to_string()
            } else {
                offline
            }
        } else {
            offline
        };
	
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 && args[1].to_string().trim() == "--no-dox".to_string().trim() {
        public_ip = "CENSORED".to_string();
    }
    let locale: String = if let Ok(lang) = env::var("LANG") {
        lang.to_string()
    } else {
        "Unknown".to_string()
    };
    let battery: String = fs::read_to_string("/sys/class/power_supply/BAT0/capacity").unwrap_or("0".to_string());
    let disk = get_disk();
    let empty: String = " ".repeat(current_logo[0].len());

    print!("\n{}{}", current_logo[0], spacing);
    print!("{}@{}\n", username.green(), hostname.green());
    print!("{}{}", current_logo[1], spacing);
    print!("\x1b[0m");
    for _ in 0..length {
        print!("-");
    }
    print!("\n");
    print!("{}{}", current_logo[2], spacing);
    println!("{}: {}", "OS".green(), os);
    print!("{}{}", current_logo[3], spacing);
    println!("{}: {}", "Kernel".green(), kernel);
    print!("{}{}", current_logo[4], spacing);
    println!("{}: {} (apt)", "Packages".green(), packages);
    print!("{}{}", current_logo[5], spacing);
    println!("{}: {}", "Shell".green(), shell);
    println!("{}{}", current_logo[6], spacing);
    print!("{}{}", current_logo[7], spacing);
    println!("{}: {}", "CPU".green(), cpu_model);
    print!("{}{}", current_logo[8], spacing);
    println!("{}: {}", "GPU".green(), gpu);
    print!("{}{}", current_logo[9], spacing);
    println!(
        "{}: {}{}{}",
        "Uptime".green(),
        u_hours,
        u_minutes,
        u_seconds
    );
    print!("{}{}", current_logo[10], spacing);
    println!(
        "{}: {} {} / {} {} ({}%)",
        "Memory".green(),
        ((used_mem * 100.0).round() / 100.0),
        mem_unit,
        ((total_mem * 100.0).round() / 100.0),
        mem_unit,
        ((used_mem / total_mem * 1000.0).round() / 10.0)
    );
    print!("{}{}", current_logo[11], spacing);
    println!(
        "{}: {} {} / {} {} ({}%)",
        "Disk".green(),
        disk.1,
        disk.0,
        disk.2,
        disk.0,
        disk.3
    );
    print!("{}{}", current_logo[12], spacing);
    println!("{}: {}%", "Battery".green(), battery);
    println!("{}{}", current_logo[13], spacing);
    print!("{}{}", current_logo[14], spacing);
    println!("{}: {}", "Private IP".green(), private_ip);
    print!("{}", empty);
    println!("{}: {}", "Public IP".green(), public_ip);
    print!("{}", empty);
    println!("{}: {}", "Locale".green(), locale);
    print!("\n{}", empty);
    for i in 40..48 {
        print!("\x1b[{}m   \x1b[0m", i);
    }
    print!("\n{}", empty);
    for i in 100..108 {
        print!("\x1b[{}m   \x1b[0m", i);
    }

    println!("\n");
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

fn get_disk() -> (String, f64, f64, f64) {
    unsafe {
        let mut stat: libc::statvfs = mem::zeroed();
        let path: CString = CString::new("/").unwrap();
        if libc::statvfs(path.as_ptr(), &mut stat) == 0 {
            let block = stat.f_frsize as f64;
            let mut total = block * stat.f_blocks as f64;
            let mut used = total - block * stat.f_bavail as f64;
            let mut unit: String = "B".to_string();
            if used > 500000000.0 {
                used = (used / 10000000.0).floor() / 100.0;
                total = (total / 10000000.0).floor() / 100.0;
                unit = "GB".to_string();
            } else if used > 1000000.0 {
                used = (used / 10000.0).floor() / 100.0;
                total = (total / 10000.0).floor() / 100.0;
                unit = "MB".to_string();
            }
            return (unit, used, total, ((used / total * 1000.0).floor() / 10.0));
        }
        return ("B".to_string(), 0.0, 0.0, 0.0);
    }
}
pub const DEBIAN_LOGO: [&str; 15] = [
    "\x1b[31m        _,met$$$$$gg.      ",
    "\x1b[31m     ,g$$$$$$$$$$$$$$$P.   \x1b[0m",
    "\x1b[31m   ,g$$P\"\"       \"\"\"Y$$.\". ",
    "\x1b[31m  ,$$P'              `$$$. ",
    "\x1b[31m',$$P       ,ggs.     `$$b:",
    "\x1b[31m`d$$'     ,$P\"'   .    $$$ ",
    "\x1b[31m $$P      d$'     ,    $$P ",
    "\x1b[31m $$:      $$.   -    ,d$$' ",
    "\x1b[31m $$;      Y$b._   _,d$P'   ",
    "\x1b[31m Y$$.    `.`\"Y$$$$P\"'      ",
    "\x1b[31m `$$b      \"-.__           ",
    "\x1b[31m  `Y$$b                    ",
    "\x1b[31m   `Y$$.                   ",
    "\x1b[31m      `Y$$b.               ",
    "\x1b[31m           `\"\"\"\"           ",
];

pub const ARCH_LOGO: [&str; 15] = [
    "\x1b[36m                -`                ",
    "\x1b[36m               .o+`               ",
    "\x1b[36m              `ooo/`              ",
    "\x1b[36m             +oooooo:             ",
    "\x1b[36m            -+oooooo+:            ",
    "\x1b[36m           /:-:++oooo+:           ",
    "\x1b[36m          /++++++++++++:          ",
    "\x1b[36m        `/+++ooooooooooo/`        ",
    "\x1b[36m      .oosssso-````/osssss+`      ",
    "\x1b[36m     -ossssso.      :sssssso.     ",
    "\x1b[36m    :ossssss/        ossso+++.    ",
    "\x1b[36m  /osssso+/:-        -:/+ossso+-  ",
    "\x1b[36m `sso+:-`                `.-/+so: ",
    "\x1b[36m`++:.                        `-/+/",
    "\x1b[36m.`                              `/",
];

pub const FEDORA_LOGO: [&str; 15] = [
    "\x1b[34m            .',;;:::;;,'.          ",
    "\x1b[34m       .';cccccccccccccc:;,.       ",
    "\x1b[34m    .:cccccccccccccccccccccc:.     ",
    "\x1b[34m  .;cccccccccccc;\x1b[37m.:ddl:.\x1b[34m;ccccc;.   ",
    "\x1b[34m .:cccccccccccc;\x1b[37mOWMKOOXMWd\x1b[34m;ccccc:. ",
    "\x1b[34m.:cccccccccccc;\x1b[37mKMMc\x1b[34m;cc;\x1b[37mxMMc\x1b[34m;ccccc:.",
    "\x1b[34m,ccccccccccccc;\x1b[37mMMM.\x1b[34m;cc;;\x1b[37mWW:\x1b[34m;cccccc,",
    "\x1b[34m:ccccccccccccc;\x1b[37mMMM.\x1b[34m;cccccccccccccc:",
    "\x1b[34m:ccccccc;\x1b[37moxOOo\x1b[34m;\x1b[37mMMMOOOk.\x1b[34m;cccccccccc:",
    "\x1b[34mcccccc;\x1b[37mOMMKxdd\x1b[34m:\x1b[37mMMMkdc.\x1b[34m;ccccccccccc ",
    "\x1b[34mccccc;\x1b[37mXMO'\x1b[34m;ccc;\x1b[37mMMM.\x1b[34m;cccccccccccccc'",
    "\x1b[34mccccc;\x1b[37mOMNc.\x1b[34mccc\x1b[37m.xMMd\x1b[34m;cccccccccccc;  ",
    "\x1b[34mcccccc;\x1b[37mdNMWXXXWM0;\x1b[34m;ccccccccccc:,   ",
    "\x1b[34mccccccc;\x1b[37m.:odl:.\x1b[34m;ccccccccccc:,.     ",
    "\x1b[34m ':cccccccccccccccc::;,.           ",
];

pub const UBUNTU_LOGO: [&str; 15] = [
    "\x1b[31m                     ....                ",
    "\x1b[31m           .',:clooo:  .:looooo:.        ",
    "\x1b[31m      .;loooooool:,'   :oooooooooc       ",
    "\x1b[31m     ;loool;.          'oooooooooo,      ",
    "\x1b[31m   ;clool'              .oooooooc.  ,,   ",
    "\x1b[31m      ...                 ......  .:oo,  ",
    "\x1b[31m .;clol:,.                        .loooo'",
    "\x1b[31m:oooooooo,                         'ooool",
    "\x1b[31m,loooooooc.                       .loooo.",
    "\x1b[31m  .,;;;'.                          ;ooooc",
    "\x1b[31m      ...                          ,oool.",
    "\x1b[31m     ;ooooo:.             ;oooooc.  :l.  ",
    "\x1b[31m      .cooooooc,,..     cooooooooo.      ",
    "\x1b[31m          .':looooooo;  ,ooooooooc       ",
    "\x1b[31m              ..';::c'  .;loooooc:'      ",
];

pub const LINUX_LOGO: [&str; 15] = [
    "\x1b[20m           .-\"\"\"\"-.           ",
    "\x1b[20m          /,.  ,-.  \\         ",
    "\x1b[20m          |()L( ()| |         ",
    "\x1b[20m          |,'__`\".| |         ",
    "\x1b[20m         .j `--\"' `  `        ",
    "\x1b[20m        / '        '   \\      ",
    "\x1b[20m       / /         `   `.     ",
    "\x1b[20m     / /             l   |    ",
    "\x1b[20m    . ,              |   |    ",
    "\x1b[20m    ,\"`.            .|   |    ",
    "\x1b[33m _.'   ``.          | `..-'l  ",
    "\x1b[33m|       `.`,        |      `  ",
    "\x1b[33m|         `.    __.j         )",
    "\x1b[33m|__        |--\"\"___|      ,-' ",
    "\x1b[33m   `\"--...,+\"\"\"\"   `._,.-'    ",
];
