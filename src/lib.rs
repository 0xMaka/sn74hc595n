/*--------------------------------------------------------------------------------*/
//  simple module for pushing data through an 8 bit shift register          - maka
/*--------------------------------------------------------------------------------*/
#![no_std]
  
use rp_pico::hal::gpio::{
  DynPinId, FunctionSioOutput, Pin, PullUp, PullDown
};
use rp_pico::hal::Timer;
use embedded_hal::delay::DelayNs;
use embedded_hal::digital::OutputPin;
const WAIT : u32 = 8000;

pub struct ShiftRegister {
  data  : Pin<DynPinId, FunctionSioOutput, PullUp>,
  clock : Pin<DynPinId, FunctionSioOutput, PullUp>,
  latch : Pin<DynPinId, FunctionSioOutput, PullUp>, 
  clear : Pin<DynPinId, FunctionSioOutput, PullDown>,
  timer : Timer
}

impl ShiftRegister {
  pub fn new(
    data  : Pin<DynPinId, FunctionSioOutput, PullUp>, 
    clock : Pin<DynPinId, FunctionSioOutput, PullUp>,
    latch : Pin<DynPinId, FunctionSioOutput, PullUp>,
    clear : Pin<DynPinId, FunctionSioOutput, PullDown>,
    timer : Timer
  ) -> ShiftRegister {
    Self {
      data  : data ,
      clock : clock, 
      latch : latch,
      clear : clear,
      timer : timer
    }
  }

  pub fn clear_register(&mut self) -> () {
    self.clear.set_low()  . unwrap();
    self.timer.delay_us(WAIT);
    self.clear.set_high() . unwrap();
  }
  
  pub fn clock_pulse(&mut self) -> () {
    self.clock.set_high() . unwrap();
    self.timer.delay_us(WAIT);
    self.clock.set_low()  . unwrap();
  }
  
  pub fn toggle_latch(&mut self) -> () {
    self.latch.set_high() . unwrap();
    self.timer.delay_us(WAIT);
    self.latch.set_low()  . unwrap();
  }
  
  pub fn put_bit(&mut self, x : u8, y : u8) -> () {
    let data = x >> y & 1;
    match data == 1{
      true  => self.data.set_high() . unwrap(),
      false => self.data.set_low()  . unwrap()
    }
    self.clock_pulse();
  }

  pub fn push_byte(&mut self, x : u8) {
    self.clock_pulse();
    self.toggle_latch();
    (0..7).for_each(|y| { self.put_bit(x,y); });
    self.clock_pulse();
    self.toggle_latch();
    self.timer.delay_us(WAIT);
  
  }
}

