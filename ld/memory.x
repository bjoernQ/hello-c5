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

    .rwtext : ALIGN(4)
    {
        *(.rwtext.literal .rwtext .rwtext.literal.* .rwtext.*)
    } > RAM

    .data : ALIGN(4)
    {
        *(.sdata .sdata.* .sdata2 .sdata2.*);
        *(.data .data.*);
        *(.data1)
    } > RAM

    .bss (NOLOAD) : ALIGN(4)
    {
        _bss_start = .;
        *(.dynsbss)
        *(.sbss)
        *(.sbss.*)
        *(.gnu.linkonce.sb.*)
        *(.scommon)
        *(.sbss2)
        *(.sbss2.*)
        *(.gnu.linkonce.sb2.*)
        *(.dynbss)
        *(.sbss .sbss.* .bss .bss.*);
        *(.share.mem)
        *(.gnu.linkonce.b.*)
        *(COMMON)
        _bss_end = .;
    } > RAM
}
