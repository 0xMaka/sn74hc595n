#![no_std]
#![no_main]

use panic_halt as _;
use cortex_m_rt as _;

use rp_pico::entry;
use rp_pico::hal::pac;
use rp_pico::hal;
use embedded_hal::delay::DelayNs;

// --

const XTAL_FREQ_HZ : u32 = 12_000_000u32;

// --

use sn74hc595n;

#[entry]
fn main() -> ! {

  // grab the peripheral access object
  let mut pac = pac::Peripherals::take().unwrap();
  // set up watchdog driver
  let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);
  // configure the clocks
  let clocks = hal::clocks::init_clocks_and_plls(
    XTAL_FREQ_HZ, pac.XOSC, pac.CLOCKS, pac.PLL_SYS, 
    pac.PLL_USB, &mut pac.RESETS, &mut watchdog
  ).unwrap();
  // set up the timer
  let mut timer = hal::Timer::new(pac.TIMER, &mut pac.RESETS, &clocks);
  // gpio pin control
  let sio = hal::Sio::new(pac.SIO);
  // set pin bank to default
  let pins = hal::gpio::Pins::new(
    pac.IO_BANK0, pac.PADS_BANK0, sio.gpio_bank0, &mut pac.RESETS
  );

  let data_pin  = pins.gpio16 . reconfigure() . into_dyn_pin();
  let clock_pin = pins.gpio17 . reconfigure() . into_dyn_pin();
  let latch_pin = pins.gpio18 . reconfigure() . into_dyn_pin();
  let clear_pin = pins.gpio19 . reconfigure() . into_dyn_pin();

  let mut sr = sn74hc595n::ShiftRegister::new(
    data_pin, clock_pin, latch_pin, clear_pin, timer
  );
  
  let delay : u32 = 200;
  
  loop { 
    timer.delay_ms(delay);
    sr.clear_register();
    (0..255).for_each(|x| { 
      sr.push_byte(x); timer.delay_ms(delay); 
    })
  }
}
