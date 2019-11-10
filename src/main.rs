#![no_std]
#![no_main]

extern crate panic_semihosting;

use cortex_m_semihosting::hprintln;
use hal::gpio::p0::*;
use hal::gpio::*;

use nrf52832_hal as hal;

const BUTTON1: u8 = 13;

#[rtfm::app(device = crate::hal::target, peripherals = true)]
const APP: () = {
    struct Resources {
        gpiote: hal::target::GPIOTE,
    }

    #[init]
    fn init(ctx: init::Context) -> init::LateResources {
        hprintln!("init started").unwrap();
        // Configure GPIOTE Channel 0 to trigger an interrupt on any value change for button 1
        ctx.device
            .GPIOTE
            .config
            .iter()
            .nth(0)
            .unwrap()
            .write(|w| unsafe { w.mode().event().polarity().toggle().psel().bits(BUTTON1) });

        // Enable GPIOTE Channel 0 as interupt
        ctx.device.GPIOTE.intenset.write(|w| w.in0().set_bit());

        hprintln!("init done").unwrap();
        init::LateResources {
            gpiote: ctx.device.GPIOTE,
        }
    }

    #[idle]
    fn idle(_: idle::Context) -> ! {
        hprintln!("idle").unwrap();
        loop {}
    }

    // TODO Does not work properly - this interrupt in continuously triggered after the button is
    // only pressed once
    #[task(binds = GPIOTE, resources = [gpiote])]
    fn gpiote(ctx: gpiote::Context) {
        // Tried disabling and renabling GPIOTE channel 0 event to no avail
        ctx.resources.gpiote.intenclr.write(|w| w.in0().set_bit());
        ctx.resources.gpiote.intenset.write(|w| w.in0().set_bit());
        hprintln!("GPIO Interrupt Triggered").unwrap();
    }
};
