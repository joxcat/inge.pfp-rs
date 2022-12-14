/* Source: microbit/yotta_targets/bbc-microbit-classic-gcc/ld/NRF51822.ld */
MEMORY
{
  FLASH (rx) : ORIGIN = 0x00018000, LENGTH = 0x28000
  RAM (rwx) :  ORIGIN = 0x20002000, LENGTH = 0x2000
}

OUTPUT_FORMAT ("elf32-littlearm", "elf32-bigarm", "elf32-littlearm")
