#!/bin/bash
if [[ "$1" == "simulator" ]];then
    cargo build --release --example $2 --target x86_64-apple-darwin
    out=target/x86_64-apple-darwin/release/examples
    mkdir -p $out/source
    touch $out/source/pdex.bin
    cp $out/lib$2.dylib $out/source/pdex.dylib
    mkdir -p build
    pdc $out/source build/$2.pdx
elif [[ "$1" == "device" ]];then
    cargo build --release --example $2 --target thumbv7em-none-eabihf
    out=target/thumbv7em-none-eabihf/release/examples
    mkdir -p $out/source
    arm-none-eabi-objcopy -O binary $out/$2 $out/source/pdex.bin
    mkdir -p build
    pdc $out/source build/$2-device.pdx
fi
