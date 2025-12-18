<br>

<div align="center">
  <img src="https://github.com/bbusn/fat32/blob/main/readme/pirate.gif" width="550" />

# fat32 - driver

A basic implementation of fat32 in rust **no_std**

[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)   [![Rust](https://img.shields.io/badge/rust-%23232.svg?logo=rust&logoColor=white)](https://rust-lang.org/)

<br><br>

</div>

## 💻 Installation

**Linux - x86_64**

```bash
cargo run --target x86_64-unknown-linux-gnu -C link-arg=-nostartfiles
```

**macOs - aarch64**

```bash
cargo run --target aarch64-unknown-linux-gnu -C link-arg=-nostartfiles
```
<br><br>

## 💿 Create a testing image

We first need to create a fat32 test image in order to be able to use our library on it

**Linux, macOs**

1. Make the script executable

```bash
chmod 755 scripts/create_image.sh
```

2. Execute it to create the image

```bash
sudo ./scripts/create_image.sh
```

You're done and can test the lib with this disk.img

<div align="center">
	<img src="https://github.com/bbusn/fat32/blob/main/readme/excited.gif" width="175" />

</div>

<br><br>
