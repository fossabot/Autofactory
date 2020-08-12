<!-- markdownlint-disable no-inline-html no-bare-urls line-length header-increment no-duplicate-header -->

# Brainfuck FPGA

## The Language

Brainfuck is a very simple language, with 8 (2^3) commands. The language has a *tape*, which has indexes from 0 upwards. Each of the *cells* on the *tape* can store 8 bits of information, which is treated as an integer. There is a *head*, which can be at any of the places on the *tape*. The commands are as follows:

* `>` Move the *head* to the right.
* `<` Move the *head* to the left.
* `+` Increments the *cell* at the place where the *head* is.
* `-` Decrements the *cell* at the place where the *head* is.
* `[` Jumps past the matching `]` if the *cell* at the place where the *head* is is zero (0b00000000).
* `]` Jumps to the matching `[` if the *cell* at the place where the *head* is is non-zero.
* `.` Prints the *cell* in the location of the *head* as an ascii character.
* `,` Reads a character from input into the current *cell* as an ascii character.

## Computer Architecture

The computer will need three storage components:

* The *tape*
* The instruction storage
* A *stack* that is used to store the location of the current `[` and `]` (and all the ones before it) so that loops can be computed efficiently.

There will also be a couple of registers that store

1. The current instruction
2. The *head* location

There needs to be an increment and decrement unit, but that will not be that important. Another important thing would be if there is a command to copy the current cell to the instruction storage or something, so then there can be a program that compiles to brainfuck. (Perhaps something that swaps output with override instruction?)
