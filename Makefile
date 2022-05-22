
# requirements on Ubuntu20.04
# - utils
# sudo apt install gcc,make,qemu,qemu-system-x86,curl,wget
# - rustup
# curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

run: build
	mkdir -p mnt/EFI/BOOT && \
	cp mnt/EFI/BOOT/hos.efi mnt/EFI/BOOT/BOOTx64.EFI && \
	qemu-system-x86_64 --bios RELEASEX64_OVMF.fd -drive format=raw,file=fat:rw:mnt -monitor stdio

build:
	cargo build

dl_ovmf:
	wget "https://github.com/retrage/edk2-nightly/raw/master/bin/RELEASEX64_OVMF.fd"