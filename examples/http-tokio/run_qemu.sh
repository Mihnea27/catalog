KERNEL=./.unikraft/build/http-c_qemu-x86_64
cmd=""

# sudo qemu-system-x86_64 \
#   -kernel "$KERNEL" \
#   -nographic \
#   -m 64M \
#   -netdev bridge,id=en0,br=virbr0 -device virtio-net-pci,netdev=en0 \
#   -append "netdev.ip=172.44.0.2/24:172.44.0.1 -- $cmd" \
#   -cpu max

# sudo qemu-system-x86_64 \
#     -kernel "$KERNEL" \
#     -chardev stdio,id=char0,logfile=serial.log,signal=off \
#     -serial chardev:char0 \
#     -netdev bridge,id=en0,br=virbr0 -device virtio-net-pci,netdev=en0 \
#     -append "netdev.ip=172.44.0.2/24:172.44.0.1 -- $cmd"

sudo qemu-system-x86_64 -s -S -cpu host -enable-kvm -m 2G \
                        -nodefaults -no-acpi -display none -serial stdio -device isa-debug-exit \
                        -netdev bridge,id=en0,br=virbr0 -device virtio-net-pci,netdev=en0 \
                        -append "netdev.ip=172.44.0.2/24:172.44.0.1 -- $cmd" \
                        -kernel http-tokio_qemu-x86_64 -append verbose

# sudo qemu-system-x86_64 \
#   -kernel "$KERNEL" \
#   -nographic \
#   -m 512M \
#   -netdev bridge,id=en0,br=virbr0 -device virtio-net-pci,netdev=en0 \
#   -append "netdev.ip=172.44.0.2/24:172.44.0.1 -- $cmd" \
#   -fsdev local,id=fs0,path=./fs0/,security_model=none \
#   -device virtio-9p-pci,fsdev=fs0,mount_tag=fs0 \
#   -cpu max
