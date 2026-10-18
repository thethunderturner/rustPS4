Important Documentation:
- https://www.psdevwiki.com/ps4/SELF_-_SPRX
- https://gabi.xinuos.com/elf/01-intro.html

# Overview
The goal of this crate is to load allocate load segments of the executable in memory, and allocate the needed memory in guest memory with the permissions specified in the segment headers.

# Analysis
A typical PS4 game executable like an `eboot.bin`, can be of ELF format, or SELF short for Signed ELF (SELF is a Sony-specific container built around an ELF executable). A SELF adds Sony's container format around ELF information. SELF may be missing in some titles.\
Consider the file like this:
- SELF Header. Can be identified by the magic bytes: `4F 15 3D 1D`
- SELF Segment Table (contains SELF segments)
- ELF Header. Can be identified by the magic bytes: `7F 45 4C 46`
- ELF program headers
- Segment data

**Therefore**: SELF tells us where the executable data is stored, whereas ELF tells us where and how to load it into guest memory.

## SELF Header
The SELF header contains metadata about the SELF header and more importantly, of the SELF Segment Table structure. The 2-byte field at offset `0x18`, tells us the number of segments in the SELF Segment Table.

## SELF Segment Table
Each segment here has length 32 bytes (or 0x20 in hex), and each field here has length 8 bytes. A SELF segment can use blocked storage, compression, or encryption.

## ELF Header
The ELF header describes the executable, including:
- `e_entry`: The virtual address where execution begins.
- `e_phoff`: The offset of the ELF program header table.
- `e_phnum`: The number of ELF program headers.


## ELF Program Headers
Each program header describes a segment or other information needed to load the executable.

For `PT_LOAD` segments, we use:
- `p_vaddr`: The segment's virtual address.
- `p_memsz`: The amount of guest memory to allocate. Its the ize in bytes of the segment in memory. So the total amount of memory a segment needs.
- `p_filesz`: The number of bytes to copy from the executable. Its the size in bytes of the segment in the file image
- `p_flags`: The memory permissions (READ, WRITE, EXECUTE).
- `p_offset`: The location of segment data in a standalone ELF.

We make the distinction between `p_filesz` and `p_memsz` because the latter is the total amount of memory needed. `p_memsz - p_filesz =` additional amount of memory needed, filled with 0s. We do this because programs often need space for variables that initially contain zero but don't need those zero bytes stored explicitly in the executable.

### PT_LOAD
Tells the loader which bytes to map into guest memory

### PT_DYNAMIC
Identifies a table containing information needed for dynamic linking, such as dependencies, symbol tables, and relocation information.

### Loading Process
1. Detect whether the file is ELF or SELF.
2. Parse the ELF header and program headers.
3. Find the `PT_LOAD` segments.
4. Allocate guest memory using `p_memsz` and `p_vaddr`.
5. Copy `p_filesz` bytes into memory, using ELF or SELF
   segment offsets as appropriate.
6. Leave the remaining memory zero-initialized.
7. Return the AddressSpace and executable entry point.

**ELF vs SELF loading:** Both formats use ELF program headers to determine the virtual memory layout, segment sizes, and memory permissions. 
For a standalone ELF, `p_offset` identifies where segment data is stored in the file. For SELF, the loader first parses the SELF segment table and matches its data entries to ELF program headers to locate the stored bytes. 
SELF data may require additional processing before being copied into guest memory. After obtaining the segment bytes, both formats use the same memory-loading process.

# Additional Information
## Load Bias
- Lets us choose where the executable lives in the PS4's virtual memory
- Lets us move an executable's entire virtual memory layout to a different starting address without changing the relative positions of its segments