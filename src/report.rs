use usbd_hid::descriptor::{gen_hid_descriptor, AsInputReport, SerializedDescriptor};
use serde::{Serialize, Serializer};
use serde::ser::SerializeTuple;

#[gen_hid_descriptor(
    (collection = APPLICATION, usage_page = GENERIC_DESKTOP, usage = GAMEPAD) = {
        (usage = Z,) = {
            #[item_settings data, variable, absolute] whammy=input;
        };
        (usage_page = BUTTON, usage_min = 1, usage_max = 6) = {
            #[packed_bits 6] frets=input;
        };
        (usage_page = BUTTON, usage_min = 1, usage_max = 8) = {
            #[packed_bits 8] buttons=input;
        };
    }
)]
#[derive(Default)]
pub struct GamepadReport {
    pub whammy: u16,
    pub frets: u8,
    pub buttons: u8,
}