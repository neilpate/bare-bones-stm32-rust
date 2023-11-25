#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _;

// Reset & Clock Control
const RCC_ADDR: u32 = 0x4002_1000;
const RCC_AHB1ENR_OFFSET: u32 = 0x14; // Advanced High Performance Bus Enable Register
const RCC_AHB2ENR_OFFSET: u32 = 0x18;
const RCC_APB1ENR_OFFSET: u32 = 0x40; // Peripheral clock enable Register

// GPIO
const MODER_OFFSET: u32 = 0x0; // Mode Register
const OTYPER_OFFSET: u32 = 0x4; // Output Type Register
const PUPDR_OFFSET: u32 = 0x0C; // Pull-up/Pull-down Register
const IDR_OFFSET: u32 = 0x10; // Input Data Register
const AFRL_OFFSET: u32 = 0x20; // Alternate Function Low Register
const AFRH_OFFSET: u32 = 0x24; // Alternate Function High Register

// General Purpose IO Port B
const GPIOB_ADDR: u32 = 0x4800_0400;

// I2C Peripheral
const I2C1_ADDR: u32 = 0x4000_5400;
const I2C_CR1_OFFSET: u32 = 0x00;
const I2C_CR2_OFFSET: u32 = 0x04;
const I2C_CCR_OFFSET: u32 = 0x1c;
const I2C_TRISE_OFFSET: u32 = 0x20;

fn setup_clocks() -> () {
    unsafe {
        // Enable the GPIOB peripheral
        let ahbenr = &*((RCC_ADDR + RCC_AHB1ENR_OFFSET) as *mut volatile_register::RW<u32>);
        ahbenr.modify(|r| r | (1 << 18)); // Bit 18 is the I/O port B clock enable

        // Enable the I2C peripheral
        let apb1enr = &*((RCC_ADDR + RCC_APB1ENR_OFFSET) as *mut volatile_register::RW<u32>);
        apb1enr.modify(|r| r | (1 << 21)); // Bit 21 enables the clock to the I2C periphweral
    }
}

fn setup_i2c_peripheral() -> () {
    unsafe {
        // Reset the peripheral
        let cr1 = &*((I2C1_ADDR + I2C_CR1_OFFSET) as *mut volatile_register::RW<u32>);
        cr1.modify(|r| r | (1 << 15)); // Bit 15 is the software reset bit
        cr1.modify(|r| r | (0 << 15)); // clear the reset

        // Set clock to the the peripheral
        let cr2 = &*((I2C1_ADDR + I2C_CR2_OFFSET) as *mut volatile_register::RW<u32>);
        cr2.write(45); // 45 MHz

        // Set clock to the the peripheral
        let ccr = &*((I2C1_ADDR + I2C_CCR_OFFSET) as *mut volatile_register::RW<u32>);
        ccr.write(225); //

        // Set rise time
        let trise = &*((I2C1_ADDR + I2C_TRISE_OFFSET) as *mut volatile_register::RW<u32>);
        trise.write(46); //

        cr1.modify(|r| r | (1 << 0)); // Enable the peripheral
    }
}

fn setup_i2c_pin(pin: i32) -> () {
    // Note this will only work for pins 0..7

    unsafe {
        // Set pin as alternate function, controlled by MODER
        let moder = &*((GPIOB_ADDR + MODER_OFFSET) as *mut volatile_register::RW<u32>);
        let pin_shift = pin * 2; // Calculate the bit position based on pin number, 2 bits per pin
        let mask = 0b11 << pin_shift; // Create a mask for the pin bits in the register (2 bits per pin)
        let mode = 0b10; // Alternate function mode
        let set_mode = mode << pin_shift; // Shift the mode to the correct position
        moder.modify(|r| (r & !mask) | set_mode); // First clear the two bits of this pins mode, then OR with the new (bit-shifted) value

        // Set output to Open Drain
        let otyper = &*((GPIOB_ADDR + OTYPER_OFFSET) as *mut volatile_register::RW<u32>);
        otyper.modify(|r| r | (1 << pin)); // Set bit <pin>

        // No need to enable pull-up resistors as R27 and R28 are present
        //let pupdr = &*((GPIOB_ADDR + PUPDR_OFFSET) as *mut volatile_register::RW<u32>);

        // Set alternate function 4
        let afrl = &*((GPIOB_ADDR + AFRL_OFFSET) as *mut volatile_register::RW<u32>);
        let pin_shift = pin * 4; // Calculate the bit position based on pin number, there are 4 bits per pin
        let mask = 0b1111 << pin_shift; // Create a mask for the pin bits in the register (4 bits per pin)
        let set_afr4 = 0b0100; // Alternate Function 4 (I2C)
        afrl.modify(|r| (r & !mask) | set_afr4); // First clear the four bits of this pins alternate function, then OR with the new (bit-shifted) value
    }
}

fn i2c_start() -> () {
    unsafe {
        // ACK
        let cr1 = &*((I2C1_ADDR + I2C_CR1_OFFSET) as *mut volatile_register::RW<u32>);
        cr1.modify(|r| r | (1 << 10)); // Bit 10 is the Ack bit
        cr1.modify(|r| r | (1 << 8)); // Bit 8 is the start bit
    }
}

#[entry]
fn main() -> ! {
    setup_clocks();
    setup_i2c_peripheral();
    setup_i2c_pin(6); // SCL is Port B.6
    setup_i2c_pin(7); // SDA is Port B.7

    i2c_start();

    loop {}
}
