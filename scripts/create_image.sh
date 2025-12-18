#!/bin/sh

# Create the file called "test.img"
dd if=/dev/zero of=test.img bs=1M count=64

# Format it to fat32
mkfs.fat -F 32 -I test.img

# Create the directory to mount it
sudo mkdir -p /mnt/fat32test

# Mount it
sudo mount -o loop,uid=$(id -u),gid=$(id -g) test.img /mnt/fat32test

# Create a file at the root of this image
echo "This is a file" > /mnt/fat32test/file.txt

# Create a folder
mkdir /mnt/fat32test/folder

# Create a file in this folder
echo "This is a file" > /mnt/fat32test/folder/file.txt

# Unmount the image
sudo umount /mnt/fat32test

# Remove the directory created
sudo rm -rf /mnt/fat32test
