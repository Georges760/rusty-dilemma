MEMORY {
    BOOT2  : ORIGIN = 0x10000000, LENGTH = 0x100
    FLASH  : ORIGIN = 0x10000100, LENGTH = 4096K - 0x100
    CONFIG : ORIGIN = ORIGIN(FLASH) + LENGTH(FLASH), LENGTH = 8192K
    RAM    : ORIGIN = 0x20000000, LENGTH = 256K
}

__config_start = ORIGIN(CONFIG) - ORIGIN(BOOT2);

SECTIONS {
  .config (NOLOAD) : ALIGN(4)
  {
    . = ALIGN(4);
    *(.config .config.*);
    . = ALIGN(4);
  } > CONFIG
}
