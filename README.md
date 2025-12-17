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
cargo run
```

**macOs - aarch64**

```bash
cargo run --target aarch64-unknown-linux-gnu
```
<br><br>

## 💿 Create a testing image

We first need to create a fat32 test image in order to be able to use our library on it

**Linux, macOs**

1. Create the file called "test.img"

```bash
dd if=/dev/zero of=test.img bs=1M count=64
```

2. Format it to fat32

```bash
mkfs.fat -F 32 -I test.img
```

3. Create the directory to mount it

```bash
sudo mkdir -p /mnt/fat32test
```

4. Mount it

```bash
sudo mount -o loop,uid=$(id -u),gid=$(id -g) test.img /mnt/fat32test
```

5. Create a file at the root of this image

```bash
echo "This is a file" > /mnt/fat32test/file.txt
```

6. Create a folder

```bash
mkdir /mnt/fat32test/folder
```

7. Create a file in this folder

```bash
echo "This is a file" > /mnt/fat32test/folder/file.txt
```

8. Unmount the image

```bash
sudo umount /mnt/fat32test
```

9. Remove the directory created

```bash
sudo rm -rf /mnt/fat32test
```

You're done and can test the lib with this test.img

<div align="center">
	<img src="https://github.com/bbusn/fat32/blob/main/readme/excited.gif" width="200" />

</div>

<br><br>
