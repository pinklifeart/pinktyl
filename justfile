# TODO: change to use a variable perhaps?
build:
  cargo objcopy --release -- -O binary target/firmware.bin

build-right:
  cargo objcopy --release --features right -- -O binary target/firmware.bin
  
build-left:
  cargo objcopy --release --features left -- -O binary target/firmware.bin

