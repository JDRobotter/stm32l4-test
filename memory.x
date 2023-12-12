MEMORY
{
  /* */
  FLASH (rx) : ORIGIN = 0x08000000, LENGTH = 10K
  /* */
  ROM (rx) : ORIGIN = 0x8007c00, LENGTH = 1K
  /* */
  RAM_VECTOR_TABLE (rwx): ORIGIN = 0x20000000, LENGTH = 192
  /* */
  RAM (rwx) : ORIGIN = 0x200000C0, LENGTH = 4K - 192
}

_stack_start = ORIGIN(RAM) + LENGTH(RAM);
