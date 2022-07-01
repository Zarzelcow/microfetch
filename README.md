# Microfetch
### A Rust only fetch program focused on code simplicity
```
❯ ./target/debug/microfetch
        /\          alex@museum
       /  \         ==========
      /    \         Distro: Arch Linux
     /  /\  \       缾 WM: wayfire
    /  |  |  \       TERM: alacritty
   /  /    \  \      SHELL: zsh
  /.'*      *'.\     PACKAGES: 756 (pacman) 46 (flatpak)
                     UPTIME: 8 hours 52 mins
                    ﬙ CPU: Intel Core i5-4590 (4) @ 3700 MHz
                     RAM: 8797MiB / 11876MiB (74%)
                     KERNEL: 5.18.7-zen1-1-zen
                     GPU 1: Intel Xeon E3-1200 v3/4th Gen Core Processor Integrated Graphics Controller
                     GPU 2: AMD/ATI Barts PRO Radeon HD 6850  
```

## Usage
Configuration is done at compile time by editing config.rs and nothing else.

Simply run `cargo run` to build and run the program and `cargo install` to install it as the command `microfetch`