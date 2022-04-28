
run:
	mkdir -p mnt/EFI/BOOT && \
	cp target/hos/debug/hos mnt/EFI/BOOT/BOOTx64.EFI && \
	qemu-system-x86_64 --bios bios/RELEASEX64_OVMF.fd -drive format=raw,file=fat:rw:mnt

dl_ovmf:
	mkdir bios && \
	cd bios && \
	wget "https://github.com/retrage/edk2-nightly/raw/master/bin/RELEASEX64_OVMF.fd"