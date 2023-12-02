#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _;

use volatile_register::{RO, RW, WO};

use cortex_m::{iprintln, Peripherals};

// Reset & Clock Control
const RCC_ADDR: u32 = 0x4002_1000;
const RCC_APB1RSTR_OFFSET: u32 = 0x10; // Peripheral Reset Register
const RCC_AHB1ENR_OFFSET: u32 = 0x14; // Advanced High Performance Bus Enable Register
const RCC_AHB2ENR_OFFSET: u32 = 0x18;
const RCC_APB1ENR_OFFSET: u32 = 0x1C; // Peripheral clock enable Register

// GPIO
const MODER_OFFSET: u32 = 0x0; // Mode Register
const OTYPER_OFFSET: u32 = 0x4; // Output Type Register
const OSPEEDR_OFFSET: u32 = 0x8; // Output Speed Register
const PUPDR_OFFSET: u32 = 0x0C; // Pull-up/Pull-down Register
const IDR_OFFSET: u32 = 0x10; // Input Data Register
const AFRL_OFFSET: u32 = 0x20; // Alternate Function Low Register
const AFRH_OFFSET: u32 = 0x24; // Alternate Function High Register

// General Purpose IO Port B
const GPIOB_ADDR: u32 = 0x4800_0400;

// I2C Peripheral
const I2C1_ADDR: u32 = 0x4000_5400;
const I2C_CR1_OFFSET: u32 = 0x00; // Control Register 1
const I2C_CR2_OFFSET: u32 = 0x04; // Control Register 2
const I2C_TIMINGR_OFFSET: u32 = 0x10; // Timing Register
const I2C_ISR_OFFSET: u32 = 0x18; // Interrupt and Status Register

const I2C_RXDR_OFFSET: u32 = 0x24;
const I2C_TXDR_OFFSET: u32 = 0x28;

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
        let apb1rstr = &*((RCC_ADDR + RCC_APB1RSTR_OFFSET) as *mut volatile_register::RW<u32>);
        apb1rstr.modify(|r| r | (1 << 21)); // Bit 21 is the software reset bit
        apb1rstr.modify(|r| r & !(1 << 21)); // clear the reset

        // Disable the peripheral
        let cr1 = &*((I2C1_ADDR + I2C_CR1_OFFSET) as *mut volatile_register::RW<u32>);
        cr1.modify(|r| r & !(1 << 0)); // Bit 0 is Peripheral Enable

        // Set up timing
        let timingr = &*((I2C1_ADDR + I2C_TIMINGR_OFFSET) as *mut volatile_register::RW<u32>);
        let mut value = 0 << 28; // Prescaler
        value |= 9; // SCL low period
        value |= 4 << 8; // SCL high period
        value |= 1 << 16; // SDA Data Hold time
        value |= 3 << 20; // SCL Data setup time

        timingr.write(value);

        // // Set clock to the the peripheral
        // let cr2 = &*((I2C1_ADDR + I2C_CR2_OFFSET) as *mut volatile_register::RW<u32>);
        // cr2.write(45); // 45 MHz

        // // Set clock to the the peripheral
        // let ccr = &*((I2C1_ADDR + I2C_CCR_OFFSET) as *mut volatile_register::RW<u32>);
        // ccr.write(225); //

        // // Set rise time
        // let trise = &*((I2C1_ADDR + I2C_TRISE_OFFSET) as *mut volatile_register::RW<u32>);
        // trise.write(46); //

        // Enable the peripheral
        value = 0;
        value |= 1 << 12; // Turn off Analogue noise filter
        value |= 1 << 2; // Enable RX interrupt
        value |= 1 << 1; // Enable TX interrupt

        cr1.write(value);

        cr1.modify(|r| r | (1 << 0)); // Bit 0 is Peripheral Enable
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

        // Set output speed
        let ospeedr = &*((GPIOB_ADDR + OSPEEDR_OFFSET) as *mut volatile_register::RW<u32>);
        let value = (0b11 << (pin * 2));
        ospeedr.modify(|r| r | value); // Set shigh speed

        // Set alternate function 4
        let afrl = &*((GPIOB_ADDR + AFRL_OFFSET) as *mut volatile_register::RW<u32>);
        let pin_shift = pin * 4; // Calculate the bit position based on pin number, there are 4 bits per pin
        let mask = 0b1111 << pin_shift; // Create a mask for the pin bits in the register (4 bits per pin)
        let set_afr4 = 0b0100 << pin_shift; // Alternate Function 4 (I2C)
        afrl.modify(|r| (r & !mask) | set_afr4); // First clear the four bits of this pins alternate function, then OR with the new (bit-shifted) value
    }
}

fn i2c_read(address: u32) -> u32 {
    unsafe {
        let isr = &*((I2C1_ADDR + I2C_ISR_OFFSET) as *mut volatile_register::RW<u32>);
        let cr2 = &*((I2C1_ADDR + I2C_CR2_OFFSET) as *mut volatile_register::RW<u32>);
        let txdr = &*((I2C1_ADDR + I2C_TXDR_OFFSET) as *mut volatile_register::RW<u32>);
        let rxdr = &*((I2C1_ADDR + I2C_RXDR_OFFSET) as *mut volatile_register::RO<u32>);

        let slave_address = 0b0011001;

        let mut value = slave_address << 1;
        value |= 1 << 16; // Number of bytes is 1
        value |= 0 << 10; // Write

        cr2.write(value); // Set slave address and number of bytes

        cr2.modify(|r| r | (1 << 13)); // Bit 13 is the start bit

        // // Loop until there is space in the transmit buffer
        // let txis = 0x02;
        // while isr.read() & txis == 1 {
        //     let _a = 0;
        // }

        txdr.write(address); // WHO_AM_I_A

        let txe = 0x01;
        // Loop until transmission has finished (TXE will be set high when tx is done)
        while isr.read() & txe == 0 {
            let a = 0;
            let b = a;
        }

        value = 0;

        value = 1 << 16; // Number of bytes
        value |= 1 << 10; // Request a read
        value |= 1 << 25; // Auto end

        cr2.modify(|r| r | value);

        cr2.modify(|r| r | (1 << 13)); // Bit 13 is the start bit

        let data_in = rxdr.read();
        let data2 = data_in + 1;
        let rxne = 1 << 2;
        // Loop until there is data ready (RXNE will go high)
        // while isr.read() & rxne == 0 {
        //     let a = 0;
        //     let b = a;
        // }

        data_in
    }
}

fn i2c_write(address: u32, data: u32) -> () {
    unsafe {
        let isr = &*((I2C1_ADDR + I2C_ISR_OFFSET) as *mut volatile_register::RW<u32>);
        let cr2 = &*((I2C1_ADDR + I2C_CR2_OFFSET) as *mut volatile_register::RW<u32>);
        let txdr = &*((I2C1_ADDR + I2C_TXDR_OFFSET) as *mut volatile_register::RW<u32>);
        let rxdr = &*((I2C1_ADDR + I2C_RXDR_OFFSET) as *mut volatile_register::RO<u32>);

        let slave_address = 0b0011001;

        let mut value = slave_address << 1;
        value |= 1 << 16; // Number of bytes is 1
        value |= 0 << 10; // Write

        cr2.write(value); // Set slave address and number of bytes

        cr2.modify(|r| r | (1 << 13)); // Bit 13 is the start bit

        // // Loop until there is space in the transmit buffer
        // let txis = 0x02;
        // while isr.read() & txis == 1 {
        //     let _a = 0;
        // }

        txdr.write(address);

        let txe = 0x01;
        // Loop until transmission has finished (TXE will be set high when tx is done)
        while isr.read() & txe == 0 {
            let a = 0;
            let b = a;
        }

        txdr.write(data);

        // Loop until transmission has finished (TXE will be set high when tx is done)
        // while isr.read() & txe == 0 {
        //     let a = 0;
        //     let b = a;
        // }

        cr2.modify(|r| r | (1 << 13)); // Bit 13 is the start bit

        // value = 0;

        // value = 1 << 16; // Number of bytes
        // value |= 1 << 10; // Request a read
        // value |= 1 << 25; // Auto end

        // cr2.modify(|r| r | value);

        // cr2.modify(|r| r | (1 << 13)); // Bit 13 is the start bit

        // let data_in = rxdr.read();
        // let data2 = data_in + 1;
        // let rxne = 1 << 2;
        // Loop until there is data ready (RXNE will go high)
        // while isr.read() & rxne == 0 {
        //     let a = 0;
        //     let b = a;
        // }
    }
}

#[entry]
fn main() -> ! {
    let mut p = Peripherals::take().unwrap();
    let stim = &mut p.ITM.stim[0];
    iprintln!(stim, "Hello, world!");

    setup_clocks();
    setup_i2c_peripheral();
    setup_i2c_pin(6); // SCL is Port B.6
    setup_i2c_pin(7); // SDA is Port B.7

    // i2c_write(0x20, 0x57); // 100 Hz mode, enable all axes

    loop {
        // let data_low = i2c_read(0x2C);
        // let data_high = i2c_read(0x2D);
        // let value = data_low | data_high << 8;

        let who_am_i = i2c_read(0x0f);
        iprintln!(stim, "WHO_AM_I: {}", who_am_i);
    }
}
