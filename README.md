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
cargo run --target x86_64-unknown-linux-gnu
```

**Linux - aarch64**

```bash
cargo run --target aarch64-unknown-linux-gnu
```
<br><br>

## 💿 Create a testing image

We first need to create a fat32 test image in order to be able to use our library on it

**Linux, macOs**

1. Make the script executable

```bash
chmod +x scripts/create_image.sh
```

2. Execute it to create the image

```bash
sudo ./scripts/create_image.sh
```

You're done and can test the lib with this disk.img

<br><br>

## 🎹 Commands

**Exit**

```bash
exit
```
*You can also quit with Ctrl + C*

  <img src="https://github.com/bbusn/fat32/blob/main/readme/run.png" width="825" />

**Navigate**

```bash
cd folder
```

```bash
cd ..
```

```bash
cd /
```

  <img src="https://github.com/bbusn/fat32/blob/main/readme/cd.png" width="825" />

**Read a file**

```bash
cat file.txt
```

  <img src="https://github.com/bbusn/fat32/blob/main/readme/cat.png" width="825" />

<br><br>


<div align="center">
	<img src="https://github.com/bbusn/fat32/blob/main/readme/excited.gif" width="175" />

</div>

<br><br>
