
# requirements
# qemu,dosfstools on Mac

run: make_img
	qemu-system-x86_64 \
	-drive if=pflash,format=raw,file=OVMF_CODE.fd \
	-drive if=pflash,format=raw,file=OVMF_VARS.fd \
	-hda hos.img

make_img: build
	qemu-img create -f raw hos.img 200M && \
	mkfs.fat -n 'HEDWIG OS' -s 2 -f 2 -R 32 -F 32 hos.img && \
	mkdir -p mnt && \
	hdiutil attach -mountpoint mnt hos.img && \
	mkdir -p mnt/EFI/BOOT && \
	cp target/hos/debug/hos mnt/EFI/BOOT/BOOTX64.EFI && \
	hdiutil detach mnt

build:
	cargo build

dl_ovmf:
	curl -O https://raw.githubusercontent.com/uchan-nos/mikanos-build/master/devenv/OVMF_CODE.fd
	curl -O https://raw.githubusercontent.com/uchan-nos/mikanos-build/master/devenv/OVMF_VARS.fd
