# Chip8 Emulator Rust
Rust rewrite of my previous [Chip8 emulator](https://github.com/olduf/chip8_emu). Uses SDL this time around. Built to mess around with Rust and SDL.

Pretty sure it's buggy, but it passes the [1-4 test roms](https://github.com/Timendus/chip8-test-suite), and runs space invaders and tetris.

## Requirement
 - [SDL 2](https://wiki.libsdl.org/SDL2/Installation)
 - Rust

## Building the project
In a terminal, navigate to the project folder.

### Application
- Run `cargo build --release`

## Usage
### Application
- `./target/release/app --path <path to the rom>`
OR
- `cargo run -- --path <path to the rom>`

### Key Binding
Here is the `COSMAC VIP` keypad:
```
1 2 3 C
4 5 6 D
7 8 9 E
A 0 B F
```

It is bound to these keys:
```
1 2 3 4
Q W E R
A S D F
Z X C V
```

Not configurable, works much better a `QWERTY` keyboard.

#### Options
- -h, --help: show help
- -p, --path: path to the rom you want to load
- -V, --version: show version

## Included ROMs
### Games
- `INVADERS`
- `TETRIS`
### Timendus Test ROMs
- `1-chip8-logo.ch8`
- `2-imb-logo.ch8`
- `3-corax+.ch8 `
- `4-flags.ch8`

## Technical References
- [Guide to making a CHIP-8 emulator](https://tobiasvl.github.io/blog/write-a-chip-8-emulator/)
- [Cowgod's Chip-8 Technical Reference v1.0](http://devernay.free.fr/hacks/chip8/C8TECH10.HTM)