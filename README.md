# bare-bones-stm32-rust
The most basic, (almost) dependency free example possible to start STM32 tinkering in rust.

Most of the embedded rust tutorials start with the HAL type crates, and while these work fine they bring a lot of baggage with them. This makes it hard to understand exactly what is necessary to get started.

The intention of this repo is to try and provide an as simple as possible example.

It is assumed that VSCode will be used as the editor.

# Requirements
- cortex-debug extension for VSCode
- OpenOCD
- GDB for ARM
