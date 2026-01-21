MEMORY
{
  RAM : ORIGIN = 0x40800000, LENGTH = 256K
  FLASH : ORIGIN = 0x42000020, LENGTH = 1M
}

SECTIONS {
    .flash.appdesc : ALIGN(4)
    {
        KEEP(*(.flash.appdesc));
        KEEP(*(.flash.appdesc.*));
    } > FLASH

    .text : ALIGN(4) {
        *(.literal .text .literal.* .text.*)
    } > FLASH

    .gap (NOLOAD): ALIGN(4) {
        . = . + 0x20;
    } > FLASH

    .rodata : ALIGN(4)
    {
        .byte 0x00, 0x00, 0x00, 0x00

        *(.data .data.*)
        *(.rodata .rodata.*)
        *(.srodata .srodata.*)
    } > FLASH
}
