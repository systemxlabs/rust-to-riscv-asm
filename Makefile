MODE := debug
PROJECT_ELF := target/riscv64gc-unknown-none-elf/$(MODE)/rust-to-riscv-asm
PROJECT_BIN := $(PROJECT_ELF).bin

ifeq ($(MODE), release)
	MODE_ARG := --release
endif

build:
	(rustup target list | grep "riscv64gc-unknown-none-elf (installed)") || rustup target add riscv64gc-unknown-none-elf
	@cargo install cargo-binutils
	@cargo build $(MODE_ARG)

asm:
	@rust-objdump --arch=riscv64 --disassemble --line-numbers --demangle $(PROJECT_ELF)

$(PROJECT_BIN): build
	@rust-objcopy $(PROJECT_ELF) --strip-all -O binary $@

run: $(PROJECT_BIN)
	@qemu-system-riscv64 \
		-d in_asm,int,mmu,pcall,cpu_reset,guest_errors \
		-D /tmp/qemu.log \
		-machine virt \
		-m 2G \
		-nographic \
		-bios bootloader/rustsbi-qemu-2024-03-24.bin \
		-device loader,file=$(PROJECT_BIN),addr=0x80200000

clean:
	@cargo clean

.PHONY: build asm run $(PROJECT_BIN) clean