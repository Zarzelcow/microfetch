mod config;

use std::io::Error;
use std::{env, fs, os};
use std::cmp::max;
use std::collections::HashMap;
use std::process::Command;

fn main() {
    let sys_info = config::get_print_config();
    let padding = config::LOGO.iter().map(|s| s.len()).max().unwrap();
    for i in 0..max(config::LOGO.len(), sys_info.len()) {
        println!("{:<padding$} {}", config::LOGO.get(i).unwrap_or(&""), sys_info.get(i).unwrap_or(&"".to_string()));
    }
}

fn get_distro() -> String {
    let str = fs::read_to_string("/etc/os-release").expect("Couldn't read /etc/os-release");
    return str.split('\n').find(|line| line.starts_with("PRETTY_NAME")).unwrap()["PRETTY_NAME=".len()..].replace('"', "");
}

fn get_wm() -> String {
    if env::var("WAYLAND_DISPLAY").is_ok() {
        let needle = format!("{}/{}", env::var("XDG_RUNTIME_DIR").unwrap(), env::var("WAYLAND_DISPLAY").unwrap());

        let procs = fs::read_dir("/proc").unwrap();
        for path in procs.filter_map(|res| res.map(|dir| dir.path()).ok()).filter(|p| p.is_dir()) {
            if let Ok(fd) = fs::read_dir(format!("{}/fd", path.to_str().unwrap())) {
                for handle in fd.filter_map(|res| res.ok()) {
                    if fs::read_link(handle.path()).unwrap().to_str().unwrap().contains(&needle) {
                        return fs::read_to_string(format!("{}/comm", path.to_str().unwrap())).unwrap().trim_end().to_string();
                    }
                }
            }
        }
    } else {
        return "X11 support not implemented".to_string();
    }

    "UNKNOWN".to_string()
}

fn command_run(command: &str, arg: &str) -> Result<String, Error> {
    Command::new(command)
        .arg(arg)
        .output()
        .map(|out| String::from_utf8(out.stdout).unwrap())
}

fn get_kernel() -> String {
    command_run("uname", "-r").expect("Failed to get kernel version").trim().to_string()
}

fn get_pacman_packages() -> usize {
    command_run("pacman", "-Qq").expect("Failed to get pacman packages").lines().count()
}

fn get_flatpak_packages() -> usize {
    command_run("flatpak", "list").expect("Failed to list flatpak packages").lines().count()
}

fn get_hostname() -> String {
    let output = Command::new("hostname").output().expect("Failed to get hostname");
    String::from_utf8(output.stdout).expect("Failed to get hostname").replace('\n', "")
}

fn get_gpu(number: usize) -> String {
    let lines: Vec<String> = command_run("lspci", "-mm").map(|out| out.split('\n').map(|s| s.to_string()).collect()).expect("Failed to get lspci devices");
    let gpu = lines.iter()
        .filter(|line| line.contains("VGA"))
        .map(|line| line["00:02.0 \"VGA compatible controller\" ".len()..].to_string())
        .nth(number)
        .unwrap();
    let split = gpu.split("\" ").collect::<Vec<&str>>();
    let manufacturer = split[0].replace('"', "");
    let model = split[1].replace('"', "");
    return remove_all_from_string(config::GPU_REMOVE, format!("{} {}", manufacturer, model));
}

fn get_user() -> String {
    env::var("USER").expect("Failed to get user (could not read $USER)")
}

fn get_terminal() -> String {
    env::var("TERM").expect("Failed to get terminal")
}

fn get_memory() -> String {
    let fdkf: Vec<String> = fs::read_to_string("/proc/meminfo")
        .map(|s| s.split('\n').flat_map(|s| s.split(':')).map(|s| s.to_string()).collect())
        .expect("Failed to get memory (unable to read /proc/meminfo");
    let map: HashMap<String, u64> = fdkf.chunks_exact(2)
        .map(|arr| (arr[0].to_string(), arr[1].to_string().trim_start().split_whitespace().next().unwrap().parse().unwrap()))
        .collect();
    // Taken from https://github.com/dylanaraps/neofetch
    let mem_total = map.get("MemTotal").unwrap();
    let mem_used = mem_total - map.get("MemAvailable").unwrap();
    return format!("{}MiB / {}MiB ({:.0}%)", mem_used / 1024, mem_total / 1024, (mem_used as f64 / *mem_total as f64) * 100f64);
}

fn get_uptime() -> String {
    let uptime = fs::read_to_string("/proc/uptime")
        .expect("Failed to get uptime (cannot open /proc/uptime)")
        .split_whitespace()
        .next().unwrap()
        .parse::<f32>()
        .expect("Failed to get uptime (cannot parse /proc/uptime)");
    let hours = (uptime / 3600f32).floor() as i32;
    let minutes = (uptime / 60f32).floor() as i32 % 60;
    format!("{} hours {} mins", hours, minutes)
}

fn get_shell() -> String {
    let ppid = os::unix::process::parent_id();
    let process_name = fs::read_to_string(format!("/proc/{}/comm", ppid)).unwrap();
    process_name.trim().to_string()
}

fn trim_whitespace(s: String) -> String {
    s.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn remove_all_from_string(to_remove: &[&str], string: String) -> String {
    trim_whitespace(to_remove.iter().fold(string, |acc, item| acc.replace(item, "")))
}

fn cpu_format(model: &str, core_count: usize, frequency: String) -> String {
    format!("{} ({}) @ {}", remove_all_from_string(config::CPU_REMOVE, model.to_string()).trim(), core_count, frequency.trim())
}

fn get_cpu() -> String {
    // Idea from https://github.com/ss7m/paleofetch/
    let lines: Vec<String> = fs::read_to_string("/proc/cpuinfo").map(|s| s.split('\n').map(|str| str.to_string()).collect()).expect("Couldn't read /proc/cpuinfo");
    let core_count = lines.iter().filter(|line| line.starts_with("model name")).count();
    let model_name = lines.iter().find(|line| line.starts_with("model name"))
        .map(|line| line["model name	: ".len()..].to_string())
        .map(|name| name.split_once('@').map_or(name.clone(), |(name, _frequency)| name.to_string()))
        .expect("Failed to get model name");
    let frequency = lines.iter().filter(|line| line.starts_with("cpu MHz")).map(|line| line["cpu MHz		: ".len()..].parse::<f32>().unwrap().floor() as i32).max().unwrap();
    cpu_format(model_name.as_str(), core_count, format!("{} MHz", frequency))
}