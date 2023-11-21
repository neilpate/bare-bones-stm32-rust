# bare-bones-stm32-rust
The most basic, (almost) dependency free example possible to start STM32 tinkering in rust.

Most of the embedded rust tutorials start with the HAL type crates, and while these work fine they bring a lot of baggage with them. This makes it hard to understand exactly what is necessary to get started.

The intention of this repo is to try and provide an as simple as possible example.

It is assumed that VSCode will be used as the editor.

# Requirements
- cortex-debug extension for VSCode
- OpenOCD
- GDB for ARM

# Running it!
Open the project in VSCode and hit F5.

# How does this work?
When run from VSCode, the binary is built and then the cortex-debug extension (which includes a runner which automatically starts OpenOCD and connects to it using GDB) will automatically load the code onto the target.
