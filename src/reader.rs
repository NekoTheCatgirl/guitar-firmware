use embedded_hal::digital::InputPin;
use rp235x_hal::{gpio::{AsInputPin, FunctionNull, PinId, PullDown}, Adc};

use crate::report::GamepadReport;

pub struct HardwareReader<'a, F1, F2, F3, F4, F5, F6, SU, SD, ST, SL, HR> 
where 
    F1: PinId,
    F2: PinId,
    F3: PinId,
    F4: PinId,
    F5: PinId,
    F6: PinId,
    
    SU: PinId,
    SD: PinId,

    ST: PinId,
    SL: PinId,
    
    HR: PinId,
{
    fret1: AsInputPin<'a, F1, FunctionNull, PullDown>,
    fret2: AsInputPin<'a, F2, FunctionNull, PullDown>,
    fret3: AsInputPin<'a, F3, FunctionNull, PullDown>,
    fret4: AsInputPin<'a, F4, FunctionNull, PullDown>,
    fret5: AsInputPin<'a, F5, FunctionNull, PullDown>,
    fret6: AsInputPin<'a, F6, FunctionNull, PullDown>,

    strum_up: AsInputPin<'a, SU, FunctionNull, PullDown>,
    strum_down: AsInputPin<'a, SD, FunctionNull, PullDown>,
    start: AsInputPin<'a, ST, FunctionNull, PullDown>,
    hero: AsInputPin<'a, HR, FunctionNull, PullDown>,
    select: AsInputPin<'a, SL, FunctionNull, PullDown>,

    adc: &'a mut Adc,
}

impl<'a, F1, F2, F3, F4, F5, F6, SU, SD, ST, SL, HR> HardwareReader<'a, F1, F2, F3, F4, F5, F6, SU, SD, ST, SL, HR> 
where 
    F1: PinId,
    F2: PinId,
    F3: PinId,
    F4: PinId,
    F5: PinId,
    F6: PinId,
    
    SU: PinId,
    SD: PinId,

    ST: PinId,
    SL: PinId,
    
    HR: PinId,
{
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        fret1: AsInputPin<'a, F1, FunctionNull, PullDown>,
        fret2: AsInputPin<'a, F2, FunctionNull, PullDown>,
        fret3: AsInputPin<'a, F3, FunctionNull, PullDown>,
        fret4: AsInputPin<'a, F4, FunctionNull, PullDown>,
        fret5: AsInputPin<'a, F5, FunctionNull, PullDown>,
        fret6: AsInputPin<'a, F6, FunctionNull, PullDown>,

        strum_up: AsInputPin<'a, SU, FunctionNull, PullDown>,
        strum_down: AsInputPin<'a, SD, FunctionNull, PullDown>,
        start: AsInputPin<'a, ST, FunctionNull, PullDown>,
        hero: AsInputPin<'a, HR, FunctionNull, PullDown>,
        select: AsInputPin<'a, SL, FunctionNull, PullDown>,
        
        adc: &'a mut Adc
    ) -> Self {
        Self {
            fret1,
            fret2,
            fret3,
            fret4,
            fret5,
            fret6,

            strum_up,
            strum_down,
            start,
            hero,
            select,

            adc,
        }
    }

    pub fn read_to_report(&mut self, report: &mut GamepadReport) {
        // Read frets
        set_bit_u8(&mut report.frets, 0, read_pin(&mut self.fret1));
        set_bit_u8(&mut report.frets, 1, read_pin(&mut self.fret2));
        set_bit_u8(&mut report.frets, 2, read_pin(&mut self.fret3));
        set_bit_u8(&mut report.frets, 3, read_pin(&mut self.fret4));
        set_bit_u8(&mut report.frets, 4, read_pin(&mut self.fret5));
        set_bit_u8(&mut report.frets, 5, read_pin(&mut self.fret6));

        // Continue reading the rest....
        set_bit_u8(&mut report.buttons, 0, read_pin(&mut self.strum_up));
        set_bit_u8(&mut report.buttons, 1, read_pin(&mut self.strum_down));
        set_bit_u8(&mut report.buttons, 2, read_pin(&mut self.start));
        set_bit_u8(&mut report.buttons, 3, read_pin(&mut self.select));
        set_bit_u8(&mut report.buttons, 4, read_pin(&mut self.hero));

        // Read the whammy
        report.whammy = self.adc.read_single();
    }
}

fn set_bit_u8(value: &mut u8, bit: u8, state: bool) {
    if state {
        *value |= 1 << bit; // set bit
    } else {
        *value &= !(1 << bit); // clear bit
    }
}

fn read_pin(pin: &mut impl InputPin) -> bool {
    pin.is_high().unwrap_or(false)
}
