targt="target/x86_64/debug/bootimage-rmos.bin"
run: $(target)
qemu-system-x86_64 -drive format=raw,file=$(target)
