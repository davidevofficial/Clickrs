# Clickrs
Clickrs is an open-source, easy-to-use, GUI Autoclicker for linux desktops made in rust with [evdev_rs](https://github.com/ndesh26/evdev-rs "Link to evdev_rs github page") and [eframe](https://github.com/jackos/eframe "Link to eframe github page"), inspired by [xclicker](https://xclicker.xyz/ "Link to xclicker website")

It aims to be fully compatible with wayland by using the low level evdev library and with plenty of options to customize your experience.

**This program requires root because it reads the input from /dev/input/ , so run with sudo**

# Features

- [x] Simple Layout
- [x] Choose any key to "spam"
- [x] Start/Stop with a custom hotkey
- [x] Specify Hold duration for the spam key
- [x] Random click interval option
- [x] Repeat until stopped or repeat a given amount of times
- [ ] Persistance between sessions

# Installing

Compile from source
```rust
cargo build --release
```
or install the distributed binaries

# License and contributions

For any contributions, or anything about the project send an email at <davidevufficial@gmail.com>