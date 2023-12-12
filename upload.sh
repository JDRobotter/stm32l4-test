ELF=./target/thumbv7em-none-eabi/release/stm32l4-test
HEX=$ELF.hex

arm-none-eabi-size $ELF
arm-none-eabi-objcopy -Oihex $ELF $HEX
stm32flash -w $HEX -b 921600 -i "rts&-dtr,,-rts&dtr,," -v -g 0 /dev/ttyUSB0 
