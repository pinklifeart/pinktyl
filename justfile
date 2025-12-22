# TODO: change to use a variable perhaps?
build:
  cargo objcopy --release -- -O binary target/firmware.bin

build-all: build-right build-left

build-right:
  cargo objcopy --release --features right -- -O binary target/right.bin
  
build-left:
  cargo objcopy --release --features left -- -O binary target/left.bin

flash-right:
  sudo dfu-util -i 0 -a 0 -s 0x08000000 -D target/right.bin
  
flash-left:
  sudo dfu-util -i 0 -a 0 -s 0x08000000 -D target/left.bin
