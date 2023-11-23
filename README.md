This is a simple binary to disassemble (much like objdump) a Universal Machine binary.

This is primarily intended for the University of Rhode Island course CSC411, but is applicable to the "Universal Machine" from ICFP 2006.

Usage: `rumdump [um_binary]`

If no argument is given, input is expected on STDIN.

This relies on ANSI terminal colors, and is currently display optimized for programs of fewer than 1 million instructions. It will work with larger programs, but display won't align perfectly.