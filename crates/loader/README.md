Important: https://www.psdevwiki.com/ps4/SELF_-_SPRX 
- p_filesz - Size in bytes of the segment in the file image
- p_memsz	- Size in bytes of the segment in memory. So the total amount of memory a segment needs.

Therefore `p_memsz - p_filesz =` additional amount of memory needed, filled with 0s. We do this because programs often need space for variables that initially contain zero but don't need those zero bytes stored explicitly in the executable.