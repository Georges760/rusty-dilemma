flash:
  cargo objcopy --release -- target/binary.elf
  elf2uf2-rs -d ./target/binary.elf
  # picotool load -f ./target/binary.elf || picotool load -f ./target/binary.elf
  # picotool reboot

flash-lcd:
  cargo objcopy --release --features gc9a01 -- target/binary-lcd.elf
  elf2uf2-rs -d ./target/binary-lcd.elf

flash-bl:
  nix build .#bl-binaries
  picotool load ./result/boot.elf
  picotool load ./result/binary.elf
  picotool reboot

dbg:
  cargo objcopy --features probe -- target/binary.elf
  probe-rs run --chip RP2040 target/binary.elf --speed 400
