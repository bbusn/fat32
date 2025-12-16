
# fat32 driver - Rust

A basic implementation of fat32 written in Rust **no_std**

## Installation

*We first need to create a fat32 test image*

### Linux

1. Create the file

```bash
dd if=/dev/zero of=fat32.img bs=1M count=64
```

2. Format it to fat32

```bash
mkfs.fat -F 32 fat32.img
```

3. Mount it

Create the directory

```bash
sudo mkdir -p /mnt/fat32test
```

Mount it

```bash
sudo mount -o loop fat32.img /mnt/fat32test
```

4. Add test files and folders in it

Create a file at the root

```bash
echo "This is a file" > /mnt/fat32test/file.txt
```
Create a folder

```bash
mkdir /mnt/fat32test/folder
```
Create a file in this folder

```bash
echo "This is a file" > /mnt/fat32test/folder/file.txt
```

5. Unmount the image

```bash
sudo umount /mnt/fat32test
```
