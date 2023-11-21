#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _;

const RCC_ADDR: u32 = 0x4002_1000; // Reset & Clock Control
const RCC_AHBENR_OFFSET: u32 = 0x14; // Advanced High Performance Bus Enable Register
const RCC_AHBENR: u32 = RCC_ADDR + RCC_AHBENR_OFFSET;

const GPIOA_ADDR: u32 = 0x4800_0000; // General Purpuse IO A
const GPIOA_IDR_OFFSET: u32 = 0x10; // Input Data Register
const GPIOA_IDR_ADDR: u32 = GPIOA_ADDR + GPIOA_IDR_OFFSET;

const GPIOE_ADDR: u32 = 0x4800_1000; //General Purpuse IO E
const GPIO_MODER_OFFSET: u32 = 0x00; // Mode Register
const GPIO_BSRR_OFFSET: u32 = 0x18; // Bit Set/Reset Register
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

        // Enable the GPIOA peripheral
        let ahbenr = &*(RCC_AHBENR as *mut volatile_register::RW<u32>);
        ahbenr.modify(|r| r | (1 << 17)); // Bit 17 is the I/O port A clock enable
                                          // By default the pin is set to be an input, so no further config is needed

        // BSRR is the register used to set or clear individual pins
        let gpioe_bsrr = &*(GPIOE_BSRR_ADDR as *mut volatile_register::RW<u32>);

        // IDR is the value at the input of the port
        let gpioa_idr = &*(GPIOA_IDR_ADDR as *mut volatile_register::RW<u32>);
        // The entry point needs to be defined as never returning, so we have to loop forever
        loop {
            let value = gpioa_idr.read();
            let pin_state = (value & 0x1) > 0; // Apply a mask so that we only look at pin 0

            if pin_state {
                gpioe_bsrr.write(1 << output_pin); // A pin is set by setting the corresponding bit in the lower 16 bits of the BSRR
            } else {
                gpioe_bsrr.write(1 << 16 + output_pin); // A pin is cleared by setting the corresponding bit in the upper 16 bits of the BSRR
            }
        }
    }
}
