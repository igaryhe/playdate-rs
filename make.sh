#!/bin/bash
if [$1 == 'simulator']
then
    out=target/release/examples
    mkdir -p $out/source
    touch $out/source/pdex.bin
    cp $out/lib$2.dylib $out/source/pdex.dylib
    mkdir -p build
    pdc $out/source build/$2.pdx
elif [$1 == 'device']
    out=target/thumbv7em-none-eabihf/release/examples
    mkdir -p $out/source
    arm-none-eabi-objcopy -O binary $out/$2 %out/source/pdex.bin
    mkdir -p build
    pdc $out/source build/$2-device.pdx
fi