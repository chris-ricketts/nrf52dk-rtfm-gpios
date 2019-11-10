#![no_std]
#![no_main]

extern crate panic_semihosting;

use nrf52832_hal as hal;

use cortex_m_semihosting::hprintln;
use hal::gpio::*;
use hal::prelude::*;

const BUTTON1: u8 = 13;

#[rtfm::app(device = crate::hal::target, peripherals = true)]
const APP: () = {
    struct Resources {
        gpiote: hal::target::GPIOTE,
        led1: Pin<Output<PushPull>>,
        button1: Pin<Input<PullUp>>,
    }

    #[init]
    fn init(ctx: init::Context) -> init::LateResources {
        hprintln!("init started").unwrap();
        let gpios = ctx.device.P0.split();

        let led1 = gpios.p0_17.into_push_pull_output(Level::High).degrade();
        let button1 = gpios.p0_13.into_pullup_input().degrade();

        // Configure GPIOTE Channel 0 to trigger an interrupt on any value change for button 1
        ctx.device
            .GPIOTE
            .config
            .iter()
            .nth(0)
            .unwrap()
            .write(|w| unsafe { w.mode().event().polarity().hi_to_lo().psel().bits(BUTTON1) });

        // Enable GPIOTE Channel 0 as interupt
        ctx.device.GPIOTE.intenset.write(|w| w.in0().set_bit());

        hprintln!("init done").unwrap();

        init::LateResources {
            gpiote: ctx.device.GPIOTE,
            led1,
            button1,
        }
    }

    #[task(binds = GPIOTE, resources = [gpiote, led1])]
    fn gpiote(ctx: gpiote::Context) {
        static mut LED1_STATE: bool = true;
        ctx.resources
            .gpiote
            .events_in
            .iter()
            .nth(0)
            .unwrap()
            .reset();

        hprintln!("Button 1 Interrupt - toggling LED 1").unwrap();

        if *LED1_STATE {
            ctx.resources.led1.set_low();
        } else {
            ctx.resources.led1.set_high();
        }

        *LED1_STATE = !*LED1_STATE;
    }
};
