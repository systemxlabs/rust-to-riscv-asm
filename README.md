# rust-to-riscv-asm
A bare metal program example about converting rust code to RISC-V assembly.

## Get started

### Generate assembly
Only need install rust toolchain
```shell
# build the project
make build
# output asm to a file
make asm > output
```

### Run example on qemu
Need install qemu
```shell
# Unbuntu
sudo apt install qemu-system-riscv64
# MacOS
brew install qemu
```

Run
```shell
make run
```