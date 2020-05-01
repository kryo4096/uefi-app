#!/bin/bash

mkdir -p qemu-resources/disk/EFI/BOOT

cargo xbuild  --target x86_64-unknown-uefi &&\
cp target/x86_64-unknown-uefi/debug/uefi-app.efi qemu-resources/disk/EFI/BOOT/bootx64.efi &&\
qemu-system-x86_64 -nodefaults\
  -machine q35,accel=kvm\
  -m 128M\
  -drive if=pflash,format=raw,file=qemu-resources/OVMF_CODE.fd,readonly=on\
  -drive if=pflash,format=raw,file=qemu-resources/OVMF_VARS.fd,readonly=on\
  -drive format=raw,file=fat:rw:qemu-resources/disk\
  -vga std\
  -display gtk