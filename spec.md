# Assembly specification

- This a custom assemby language created for the chip-8 rom.
- This repo includes a custom assembler written in rust

## Registers

> Registers from V0 to VF are referenced by their name **[rv0-rvf]**

> I register by **irg**

> Delay timer and Sound timer by **rdt** and **rst** respectively

### Operations

mov - Specify data to be loaded into a register can be an literal or from another register.

draw - sprite to be drawn.

jump - jump to the specified address

add - add and save the reg in the same register

se - skips the next instruction if the the values are equal

clr - clears the screen