cargo build --release
elf2uf2-rs ./target/thumbv6m-none-eabi/release/picotest

cp ./target/thumbv6m-none-eabi/release/picotest.uf2 /media/gate/RPI-RP2

