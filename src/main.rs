#![no_std]
#![no_main]


mod reader;
mod report;

use hal::block::ImageDef;
use panic_halt as _;
use rp235x_hal::{self as hal, adc::AdcPin, usb::UsbBus, Adc};

use usb_device::{bus::UsbBusAllocator, prelude::*};
use usbd_hid::{descriptor::SerializedDescriptor, hid_class::HIDClass};

use crate::{reader::HardwareReader, report::GamepadReport};

/// Tell the Boot ROM about our application
#[link_section = ".start_block"]
#[used]
pub static IMAGE_DEF: ImageDef = hal::block::ImageDef::secure_exe();

/// External high-speed crystal on the Raspberry Pi Pico 2 board is 12 MHz.
/// Adjust if your board has a different frequency
const XTAL_FREQ_HZ: u32 = 12_000_000u32;

#[hal::entry]
fn main() -> ! {
    // Grab our singleton objects
    let mut pac = hal::pac::Peripherals::take().unwrap();

    // Set up the watchdog driver - needed by the clock setup code
    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);

    // Configure the clocks
    //
    // The default is to generate a 125 MHz system clock
    let clocks = hal::clocks::init_clocks_and_plls(
        XTAL_FREQ_HZ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    // The single-cycle I/O block controls our GPIO pins

    let mut _timer = hal::Timer::new_timer0(pac.TIMER0, &mut pac.RESETS, &clocks);

    // Create USB bus allocator
    let usb_bus: UsbBus = UsbBus::new(
        pac.USB,
        pac.USB_DPRAM,
        clocks.usb_clock,
        true, // force VBUS detect
        &mut pac.RESETS,
    );

    let bus_allocator = UsbBusAllocator::new(usb_bus);

    let mut hid = HIDClass::new(&bus_allocator, GamepadReport::desc(), 5);

    // --- Build USB device ---
    let mut usb_dev = UsbDeviceBuilder::new(&bus_allocator, UsbVidPid(0x1209, 0x0001)).build();

    let sio = hal::Sio::new(pac.SIO);

    let pins = hal::gpio::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let mut adc = Adc::new(pac.ADC, &mut pac.RESETS);

    let whammy = AdcPin::new(pins.gpio26.into_floating_input()).unwrap();
    
    adc.free_running(&whammy);

    let mut reader = HardwareReader::new(
        pins.gpio1.as_input(),
        pins.gpio2.as_input(),
        pins.gpio3.as_input(),
        pins.gpio4.as_input(),
        pins.gpio5.as_input(),
        pins.gpio6.as_input(),
        pins.gpio7.as_input(),
        pins.gpio8.as_input(),
        pins.gpio9.as_input(),
        pins.gpio11.as_input(),
        pins.gpio10.as_input(),
        &mut adc,
    );

    let mut report = GamepadReport::default();
    loop {
        if usb_dev.poll(&mut [&mut hid]) {
            reader.read_to_report(&mut report);
            let _ = hid.push_input(&report);
        }
    }
}

// Program metadata for `picotool info`.
// This isn't needed, but it's recomended to have these minimal entries.
#[link_section = ".bi_entries"]
#[used]
pub static PICOTOOL_ENTRIES: [hal::binary_info::EntryAddr; 5] = [
    hal::binary_info::rp_cargo_bin_name!(),
    hal::binary_info::rp_cargo_version!(),
    hal::binary_info::rp_program_description!(
        c"A gamepad firmware for a guitar hero Live guitar (requires severe harware modding)."
    ),
    hal::binary_info::rp_cargo_homepage_url!(),
    hal::binary_info::rp_program_build_attribute!(),
];

// End of file
