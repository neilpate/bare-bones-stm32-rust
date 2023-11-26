pub fn new(
    rcc: &stm32f3::stm32f303::RCC,
    gpiob: &stm32f3::stm32f303::GPIOB,
    i2c: &stm32f3::stm32f303::I2C1,
) -> Self {
    rcc.ahbenr.modify(|_, w| w.iopben().set_bit()); //enable gpiob clock
    rcc.apb1enr.modify(|_, w| w.i2c1en().set_bit()); //enable i2c1 clock
    rcc.apb1rstr.modify(|_, w| w.i2c1rst().set_bit());
    rcc.apb1rstr.modify(|_, w| w.i2c1rst().clear_bit());

    gpiob
        .moder
        .modify(|_, w| w.moder6().bits(0b10).moder7().bits(0b10)); //alternate function mode
    gpiob
        .pupdr
        .modify(|_, w| unsafe { w.pupdr6().bits(0b01).pupdr7().bits(0b01) }); //pull up resister
    gpiob
        .otyper
        .modify(|_, w| w.ot6().set_bit().ot7().set_bit()); //open drain output
    gpiob
        .ospeedr
        .modify(|_, w| w.ospeedr6().bits(0b11).ospeedr7().bits(0b11)); //high speed
    gpiob
        .afrl
        .modify(|_, w| w.afrl6().bits(0b0100).afrl7().bits(0b0100)); //alternate function 4

    i2c.cr1.modify(|_, w| w.pe().clear_bit());

    i2c.timingr.modify(|_, w| {
        w.presc().bits(0); // all settings from page 849 on port mapping
        w.scll().bits(9); // standard mode at 8MHz cpu and 100kHz i2c
        w.sclh().bits(4);
        w.sdadel().bits(1);
        w.scldel().bits(3)
    });

    i2c.cr1.write(|w| {
        w.anfoff().clear_bit(); //enable analogue filter
        w.nostretch().clear_bit();
        w.txie().set_bit(); //enable interrupt registers
        w.rxie().set_bit()
    });

    i2c.cr1.modify(|_, w| w.pe().set_bit()); //enable preipheral

    i2c_func {}
}

pub fn read(
    &self,
    i2c: &stm32f3::stm32f303::I2C1,
    device_address: u8,
    register_address: u8,
    request_length: u8,
    rx_data: &mut [u8],
) {
    let mut _test_register: bool = false;
    i2c.cr2.modify(|_, w| {
        w.sadd().bits(u16::from(device_address << 1)); //set device address
        w.nbytes().bits(1); //amount of bytes to send
        w.rd_wrn().clear_bit(); //set as a read operation
        w.autoend().clear_bit()
    });

    i2c.cr2.modify(|_, w| w.start().set_bit()); //send start signal

    while i2c.isr.read().txis().bit_is_set() {} //wait for txis to register to be set

    i2c.txdr.modify(|_, w| w.txdata().bits(register_address)); // Send the address of the register that we want to read: IRA_REG_M

    while i2c.isr.read().txe().bit_is_clear() {
        _test_register = i2c.isr.read().nackf().bits();
    } // Wait until transfer complete

    i2c.cr2.modify(|_, w| {
        w.nbytes().bits(request_length); //set
        w.rd_wrn().set_bit();
        w.autoend().set_bit()
    });

    i2c.cr2.modify(|_, w| w.start().set_bit());

    for count in 0..request_length {
        // Wait until we have received the contents of the register
        while i2c.isr.read().rxne().bit_is_clear() {}

        // Broadcast STOP (automatic because of `AUTOEND = 1`)
        rx_data[count as usize] = i2c.rxdr.read().rxdata().bits()
    }
}

pub fn write(
    &self,
    i2c: &stm32f3::stm32f303::I2C1,
    device_address: u8,
    register_address: u8,
    tx_data: u8,
) {
    let mut _test_register: bool = false;
    i2c.cr2.modify(|_, w| {
        w.sadd().bits(u16::from(device_address << 1)); //set device address
        w.nbytes().bits(2); //amount of bytes to send
        w.rd_wrn().clear_bit(); //set as a read operation
        w.autoend().clear_bit()
    });

    i2c.cr2.modify(|_, w| w.start().set_bit()); //send start signal

    while i2c.isr.read().txis().bit_is_set() {} //wait for txis to register to be set

    i2c.txdr.modify(|_, w| w.txdata().bits(register_address)); // Send the address of the register that we want to read: IRA_REG_M

    while i2c.isr.read().txe().bit_is_clear() {
        _test_register = i2c.isr.read().nackf().bits();
    } // Wait until transfer complete

    i2c.txdr.modify(|_, w| w.txdata().bits(tx_data)); // Send the address of the register that we want to read: IRA_REG_M

    while i2c.isr.read().txe().bit_is_clear() {
        _test_register = i2c.isr.read().nackf().bits();
    } // Wait until transfer complete

    i2c.cr2.modify(|_, w| w.stop().set_bit()); //send start signal
}
