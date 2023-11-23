# bare-bones-stm32-rust
The most basic, (almost) dependency free example possible to start STM32 tinkering in rust.

Most of the embedded rust tutorials start with the HAL type crates, and while these work fine they bring a lot of baggage with them. This makes it hard to understand exactly what is necessary to get started.

The intention of this repo is to try and provide an as simple as possible example of some simple digital input, output and interrupt.

It is assumed that VSCode will be used as the editor.

Note: this is not very rust-like code. It is more like C. *Do not write real code like this, its just meant to be a starting point for understanding the hardware!*

# Requirements
- cortex-debug extension for VSCode
- OpenOCD
- GDB for ARM

# Running it!
Open the project in VSCode and hit F5.

# How does the build chain work?
When run from VSCode, the binary is built and then the cortex-debug extension (which includes a runner which automatically starts OpenOCD and connects to it using GDB) will automatically load the code onto the target.

# What should I expect it to do?
When you press the User button the West LED (Green) will light.
The switch is connected to Port A.0
The LED is connected to Port E.15

# Schematic

![image](https://github.com/neilpate/bare-bones-stm32-rust/assets/7802334/2540be47-1020-40d1-8ae6-6265641b3036)

![image](https://github.com/neilpate/bare-bones-stm32-rust/assets/7802334/86f800a5-cade-4b65-a3d0-a137370b12bf)
