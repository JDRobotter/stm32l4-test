#![no_main]
#![no_std]

use stm32l4xx_hal::pac;
use stm32l4xx_hal::prelude::*;

use core::panic::PanicInfo;
#[panic_handler]
fn panic(_pi: &PanicInfo) -> ! {
    loop {}
}

// Setup startup code and minimal runtime for uC
// (check https://docs.rs/cortex-m-rt/latest/cortex_m_rt/)
use cortex_m_rt::entry;
#[entry]
fn main() -> ! {
    let p = pac::Peripherals::take().unwrap();
    let _cp = cortex_m::peripheral::Peripherals::take().unwrap();

    let _flash = p.FLASH.constrain();
    let mut rcc = p.RCC.constrain();
    let _pwr = p.PWR.constrain(&mut rcc.apb1r1);

    // split gpio port to pins
    let mut gpioa = p.GPIOA.split(&mut rcc.ahb2);

    // configure GPIOs
    let mut uart_tx = gpioa
        .pa2
        .into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper);
    let mut uart_rx = gpioa
        .pa3
        .into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper);

    loop {
        // toggle pins and loop
        uart_rx.toggle();
        uart_tx.toggle();
        // NOTE delay is roughly expressed in CPU cycles
        cortex_m::asm::delay(100_000);
    }
}
