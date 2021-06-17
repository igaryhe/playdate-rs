#!/bin/bash
if [$1 == 'simulator']
then
    out=target/release/examples
    mkdir $out/source
    touch $out/source/pdex.bin
    cp $out/lib$2.dylib $out/source/
    pdc $out/source out.pdx
elif [$1 == 'device']
    out=target/thumbv7em-none-eabihf/release/examples
    mkdir $out/source
    arm-none-eabi-objcopy -O binary $out/$2 %out/source/pdex.bin
    pdc $out/source out.pdx
fi