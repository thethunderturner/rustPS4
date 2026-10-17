# This file will contain things that I learned along the way

## File offsets
A position measured in bytes from the beginning of a file. It describes where data is stored inside a file.\
When a file is first opened using `File::open()`, its cursor starts at offset `0x0`. Each read advances the cursor by the number of bytes read. We can use `seek()` to move the cursor to a different offset.

## Virtual Memory Addresses
When the ps4 executes a program, it doesnt execute instructions directly from the `eboot.bin`, BUT the loader places executable's segments into the guests virtual address space. So it descibes where a running program expects to access that data in its address space.\
These segments are specified in `eboot.bin`. A PS4 eboot.bin can be a SELF container containing ELF information, or a plain ELF in a decrypted/dumped form. Consider:
```
AA BB CC DD 11 22 33 44
File size: 0x8

Segment 1:
  Segment offset: 0x0
  Virtual address: 0x400000
  Segment size: 0x4
Segment 2:
  Segment offset: 0x4
  Virtual address: 0x500000
  Segment size: 0x4
```
For segment 1, the loader reads that the segment starts at `0x0` and ends at `0x4`. So:
```
AA -> 0x400000
BB -> 0x400001
CC -> 0x400002
DD -> 0x400003
```
For segment 2, the loader reads that the segment starts at `0x4` and ends at `0x8`, because its of size `0x4`. So:
```
11 -> 0x500000
22 -> 0x500001
33 -> 0x500002
44 -> 0x500003
```
When loading a segment into memory, each byte's destination address is calculated using its offset relative to the beginning of that segment, not the beginning of the file.\
For example: The byte at file offset `0x4` is the first byte of Segment 2, so its relative offset within Segment 2 is 0. Therefore, it is placed at Segment 2's starting virtual address, 0x500000.
The next byte has a relative offset of 1, so it goes to `0x500001`, and so on.

## Guest Memory
A continuous region of the PS4's virtual memory. When we load the ELF, the `PT_LOAD` headers tell us where to place executable data in guest virtual memory. Example:
```
ElfProgramHeader {
    p_type: 1,
    p_flags: 5,
    p_offset: 16384,
    p_vaddr: 0,
    p_paddr: 0,
    p_filesz: 55652344,
    p_memsz: 55652344,
    p_align: 16384,
},
```
Using a load bias X, the virtual address of the segment starts at `X + p_vaddr`, and has size `p_memsz`. Note, that we allocate from `X + p_vaddr` to `X + p_vaddr + p_filesz`, and additionally from `X + p_vaddr + p_filesz + 1` to `X + p_vaddr + p_memsz` with 0s.\
As specified by `p_flags`, each region in the guest memory has certain permissions, ie Read, Write, Execute.

## Address Space
An address space is the collection of virtual memory addresses available to a program.\
For this emulator, we'll represent it as a collection of mapped memory regions.