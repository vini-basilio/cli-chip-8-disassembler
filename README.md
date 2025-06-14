![image](https://github.com/user-attachments/assets/5f6b23c8-9ec2-4fbd-8611-1093cf12e58f)
-----
# CHIP-8 Disassembler

Hello\! This is a disassembler for CHIP-8 ROMs.

## Why This Project?

I've been developing a CHIP-8 emulator in TypeScript (you can find the repository [here](https://github.com/vini-basilio/chip8-vite) and play it [here](https://chip8-vite.vercel.app/)). While striving for accuracy with the original CHIP-8 specification, I encountered challenges in debugging and verifying the emulator's behavior. Reading raw hexadecimal opcodes, even with knowledge of the instruction set, isn't always straightforward.

Simultaneously, I'm eager to learn a systems programming language, and Rust's excellent documentation and robust ecosystem made it a natural choice. This project serves as an opportunity to port and refactor parts of my emulator's core logic into Rust, specifically focusing on the disassembly process.

## About the Project

CHIP-8 opcodes exhibit varied patterns. For instance:

  * `1nnn`: A jump instruction where the first nibble (`1`) defines the opcode.
  * `8xy1`: An "OR" binary operation where both the first nibble (`8`) and the last nibble (`1`) are part of the opcode definition.

The core opcode decoding logic for both this CLI and my emulator is inspired by [Tania's](https://www.taniarascia.com/writing-an-emulator-in-javascript-chip8/) elegant approach to managing CHIP-8 opcodes. I highly recommend checking out her blog for a detailed study of the implementation.

## Assembly Conventions

All assembly output in this disassembler is based on [Cowgod's Chip-8 Technical Reference v1.0](http://devernay.free.fr/hacks/chip8/C8TECH10.HTM). Super CHIP (S-CHIP) instructions are also covered. For S-CHIP instructions, a clear label disclaimer is provided:

```txt
SCD 0x01 [super set]
```

## Running the Disassembler

1.  **Download:** Obtain the `.exe` executable file from the releases.
2.  **Open Terminal:** Launch your terminal or command prompt.
3.  **Navigate:** Change your directory to where the executable is located. For example, if it's on your Desktop:
    ```bash
    C:\Users\[your user name]\Desktop>
    ```
4.  **Execute:** Run the program, providing the path to your CHIP-8 ROM file. If the ROM is in the same folder as the executable:
    ```bash
    C:\Users\[]\Desktop>cli-chip-8-disassembler.exe ibm_logo.ch8
    ```

---

## Extending Functionality

If you need to support additional instruction sets (as there is no universally "formal" CHIP-8 assembly standard), you'll need to modify two key parts of this library's source code:

### 1. Define New Instruction Patterns

You'll first need to insert a new pattern into the **`instruction!` macro** located in `src\modules\instrutions`. This macro helps define how each instruction is identified and what arguments it takes.

The `instruction!` macro has the following structure:

```rust
macro_rules! instruction {
    ($id:expr, $mask:expr, $pattern:expr, [$($arg_mask:expr, $arg_shift:expr),*]){
```

Here's a breakdown of its parameters:

* **`$id:expr`**: A unique string identifier for your instruction (e.g., `"IF_VX_EQUALS_LIT"`).
* **`$mask:expr`**: A hexadecimal mask used to isolate the opcode portion of the instruction.
* **`$pattern:expr`**: The specific hexadecimal pattern that identifies this instruction after the mask is applied.
* **`[$($arg_mask:expr, $arg_shift:expr),*]`**: A comma-separated list of tuples, where each tuple defines an argument:
    * **`$arg_mask:expr`**: A mask to extract the argument value from the opcode.
    * **`$arg_shift:expr`**: The number of bits to shift the extracted argument to the right to get its actual value.

For example, to define the "If VX Equals Literal" instruction:

```rust
//                                mask    pattern      X   shift     NN    shift
instruction!("IF_VX_EQUALS_LIT", 0xF000, 0x3000, [0x0F00,      8, 0x00FF,      0]),
```

### 2. Implement Decoding Logic

After defining your new instruction pattern, you'll need to add a corresponding match expression within the **`decode` function** in `src\modules\disassembler`. This is where you define how the instruction should be represented in human-readable assembly format.

You'll add a new arm to the `match` statement that corresponds to your `$id` from the `instruction!` macro. For our example, "IF\_VX\_EQUALS\_LIT", the decoding logic would look like this:

```rust
// ... inside the decode function's match statement
"IF_VX_EQUALS_LIT" => format!("SE V{:01X}, 0x{:02X}", decoded.arguments[0], decoded.arguments[1]),
// ... other instructions
```

Here, `decoded.arguments[0]` and `decoded.arguments[1]` refer to the values extracted using the `arg_mask` and `arg_shift` you defined in the `instruction!` macro. The `format!` macro then constructs the final assembly string.

---

## Resources

Here are some excellent resources that were invaluable during the development of this project and can aid in understanding CHIP-8 emulation and architecture:

* [How to write a CHIP-8 emulator](https://tobiasvl.github.io/blog/write-a-chip-8-emulator/) by Tobiasvl: A comprehensive guide to building a CHIP-8 emulator.
* [chip-8\_documentation](https://github.com/trapexit/chip-8_documentation) by trapexit: A repository containing various CHIP-8 related documents and specifications.
* [Chip-8.pdf](https://www.cs.columbia.edu/~sedwards/classes/2016/4840-spring/designs/Chip8.pdf) by Stephen Edwards (Columbia University): A detailed technical design document for CHIP-8.
* **IBM Logo ROM and Test Suite**: The IBM Logo ROM used for testing can be found [here](https://github.com/Timendus/chip8-test-suite/blob/main/bin/2-ibm-logo.ch8).
* [Chip-8 Archive ROMs](https://github.com/JohnEarnest/chip8Archive/tree/master/roms): A collection of various CHIP-8 ROMs.
* [All instrutions set from another extensions](https://chip8.gulrak.net/)
