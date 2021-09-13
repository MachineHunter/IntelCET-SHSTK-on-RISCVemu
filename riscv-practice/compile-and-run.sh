riscv64-unknown-linux-gnu-gcc -fPIC -shared -o $1.so $1.c -ldl && \
sudo mount riscv-rust/resources/linux/busybear.bin mnt && \
sudo mv $1.so mnt/root && \
sudo umount mnt
