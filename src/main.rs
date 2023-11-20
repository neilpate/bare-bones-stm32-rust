#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _;

const RCC_ADDR: u32 = 0x4002_1000;
const RCC_AHBENR_OFFSET: u32 = 0x14;
const RCC_AHBENR: u32 = RCC_ADDR + RCC_AHBENR_OFFSET;

const GPIOE_ADDR: u32 = 0x4800_1000;
const GPIO_MODER_OFFSET: u32 = 0x00;
const GPIO_BSRR_OFFSET: u32 = 0x18;
const GPIOE_MODER_ADDR: u32 = GPIOE_ADDR + GPIO_MODER_OFFSET;
const GPIOE_BSRR_ADDR: u32 = GPIOE_ADDR + GPIO_BSRR_OFFSET;

#[entry]
fn main() -> ! {
    unsafe {
        let output_pin = 15; // On the STM32F3Discovery the West LED of the compass (green) is PORTE.15

        // Enable the GPIOE peripheral
        let rcc_ahbenr = &*(RCC_AHBENR as *mut volatile_register::RW<u32>);
        rcc_ahbenr.modify(|r| r | (1 << 21)); // Bit 21 is the I/O port E clock enable

        // Set desired pin as output
        let gpioe_moder = &*(GPIOE_MODER_ADDR as *mut volatile_register::RW<u32>);

        let pin_shift = output_pin * 2; // Calculate the bit position based on pin number
        let mask = 0b11 << pin_shift; // Create a mask for the pin bits in the register (2 bits per pin)

        let mode = 0b01; // General purpose output mode
        let set_mode = mode << pin_shift; // Shift the mode to the correct position

        gpioe_moder.modify(|r| (r & !mask) | set_mode); // First clear the two bits of this pins mode, then OR with the new (bit-shifted) value

        // Now set the output high
        let gpioe_bsrr = &*(GPIOE_BSRR_ADDR as *mut volatile_register::RW<u32>);
        gpioe_bsrr.write(1 << output_pin); // A pin is set by setting the corresponding bit in the lower 16 bits of the BSRR
    }

    loop {} // The entry point needs to be defined as never returning, so we have to loop forever
}
