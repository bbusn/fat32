
# fat32 driver - Rust

A basic implementation of fat32 written in Rust **no_std**

## Create an image for testing

*We first need to create a fat32 test image*

**Linux**

1. Create the file called "test.img"

```bash
dd if=/dev/zero of=test.img bs=1M count=64
```

2. Format it to fat32

```bash
mkfs.fat -F 32 test.img
```

3. Create the directory to mount it

```bash
sudo mkdir -p /mnt/fat32test
```

4. Mount it

```bash
sudo mount -o loop test.img /mnt/fat32test
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

You're done and can test the lib with this test.img
