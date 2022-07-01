use crate::{get_cpu, get_distro, get_flatpak_packages, get_gpu, get_hostname, get_kernel, get_memory, get_pacman_packages, get_shell, get_terminal, get_uptime, get_user, get_wm};

pub(crate) fn get_print_config() -> Vec<String> {
    return vec![
        format!("{}@{}", get_user(), get_hostname()),
        "==========".to_string(),
        format!(" Distro: {}", get_distro()),
        format!("缾 WM: {}", get_wm()),
        format!(" TERM: {}", get_terminal()),
        format!(" SHELL: {}", get_shell()),
        format!(" PACKAGES: {} (pacman) {} (flatpak)", get_pacman_packages(), get_flatpak_packages()),
        format!(" UPTIME: {}", get_uptime()),
        format!("﬙ CPU: {}", get_cpu()),
        format!(" RAM: {}", get_memory()),
        format!(" KERNEL: {}", get_kernel()),
        format!(" GPU 1: {}", get_gpu(0)),
        format!(" GPU 2: {}", get_gpu(1))
    ];
}

pub(crate) const LOGO: &[&str] = &[
    r"         /\         ",
    r"        /  \        ",
    r"       /    \       ",
    r"      /  /\  \      ",
    r"     /  |  |  \     ",
    r"    /  /    \  \    ",
    r"   /.'*      *'.\   ",
];

pub(crate) const CPU_REMOVE: &[&str] = &[
    "(R)",
    "(TM)",
    "CPU"
];

pub(crate) const GPU_REMOVE: &[&str] = &[
    "Corporation",
    "Advanced Micro Devices, Inc.",
    "[","]"
];