//! Operations on UART and GPIO of RK3588 board

/// RK3588 UART7_M2 Switch iomux
pub fn iomux_uart7_m2(addr: usize) {
    //const BUS_IOC_BASE: u32 =  0xfd5f8000; // 0x0 ~ 0x9C
    let gpio1b_iomux_sel_h: usize = 0x002C; /* Address Offset: 0x002C */

    let GENMASK = |h: u64, l: u64| (!(0 as u64) << l) & (!(0 as u64) >> (64 - 1 - h));

    let GPIO1B4_MASK: u64 = GENMASK(3, 0);
    let GPIO1B5_MASK: u64 = GENMASK(7, 4);
    const GPIO1B4_UART7_RX_M2: u64 = 10;
    const GPIO1B4_SHIFT: u64 = 0;
    const GPIO1B5_SHIFT: u64 = 4;
    const GPIO1B5_UART7_TX_M2: u64 = 10;

    rk_clrsetreg(
        (addr + gpio1b_iomux_sel_h) as u64,
        GPIO1B4_MASK | GPIO1B5_MASK,
        GPIO1B4_UART7_RX_M2 << GPIO1B4_SHIFT | GPIO1B5_UART7_TX_M2 << GPIO1B5_SHIFT,
    );
}

fn rk_clrsetreg(addr: u64, clr: u64, set: u64) {
    let value = (((clr | set) << 16) | set) as u32;
    unsafe {
        core::ptr::write_volatile(addr as *mut u32, value);
    }
    //writel(value, addr)
}

// Enable GPIO3_C6_u, RK_PC6=22;
// Pin = bank(3)*32 + number( C(2)*8 + 6 ) = 96 + 22 = 118
// Data direction(GPIO_SWPORT_DDR_H): Output; Output data: High, Low;
pub fn gpio_output(gpio_bank: usize, number: usize, is_high: bool) {
    // number = [0, 31]
    let (base_offset, num_shift) = if number < 16 {
        // Output data for low 16 bits
        (0, number)
    } else {
        // Output data for high 16 bits
        (0x4, number % 16)
    };
    let data_base = gpio_bank + base_offset;
    let direction_base = gpio_bank + 0x8 + base_offset;
    let mask = (1 << num_shift) << 16;
    let direction = if is_high { 1 } else { 0 };
    unsafe {
        // Set data direction: output
        core::ptr::write_volatile(direction_base as *mut u32, (1 << num_shift) | mask);

        // Set data: high/low; high = 1, low = 0;
        core::ptr::write_volatile(data_base as *mut u32, (direction << num_shift) | mask);
    }
}

pub fn gpio_output_clear(gpio_bank: usize) {
    unsafe {
        core::ptr::write_volatile(gpio_bank as *mut u32, 0 | (0xffff << 16));
        core::ptr::write_volatile((gpio_bank + 0x4) as *mut u32, 0 | (0xffff << 16));
    }
}
