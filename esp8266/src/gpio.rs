#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00 - BT-Coexist Selection register"]
    pub gpio_out: GPIO_OUT,
    #[doc = "0x04 - GPIO_OUT_W1TS"]
    pub gpio_out_w1ts: GPIO_OUT_W1TS,
    #[doc = "0x08 - GPIO_OUT_W1TC"]
    pub gpio_out_w1tc: GPIO_OUT_W1TC,
    #[doc = "0x0c - GPIO_ENABLE"]
    pub gpio_enable: GPIO_ENABLE,
    #[doc = "0x10 - GPIO_ENABLE_W1TS"]
    pub gpio_enable_w1ts: GPIO_ENABLE_W1TS,
    #[doc = "0x14 - GPIO_ENABLE_W1TC"]
    pub gpio_enable_w1tc: GPIO_ENABLE_W1TC,
    #[doc = "0x18 - The values of the strapping pins."]
    pub gpio_in: GPIO_IN,
    #[doc = "0x1c - GPIO_STATUS"]
    pub gpio_status: GPIO_STATUS,
    #[doc = "0x20 - GPIO_STATUS_W1TS"]
    pub gpio_status_w1ts: GPIO_STATUS_W1TS,
    #[doc = "0x24 - GPIO_STATUS_W1TC"]
    pub gpio_status_w1tc: GPIO_STATUS_W1TC,
    #[doc = "0x28 - GPIO_PIN0"]
    pub gpio_pin0: GPIO_PIN0,
    #[doc = "0x2c - GPIO_PIN1"]
    pub gpio_pin1: GPIO_PIN1,
    #[doc = "0x30 - GPIO_PIN2"]
    pub gpio_pin2: GPIO_PIN2,
    #[doc = "0x34 - GPIO_PIN3"]
    pub gpio_pin3: GPIO_PIN3,
    #[doc = "0x38 - GPIO_PIN4"]
    pub gpio_pin4: GPIO_PIN4,
    #[doc = "0x3c - GPIO_PIN5"]
    pub gpio_pin5: GPIO_PIN5,
    #[doc = "0x40 - GPIO_PIN6"]
    pub gpio_pin6: GPIO_PIN6,
    #[doc = "0x44 - GPIO_PIN7"]
    pub gpio_pin7: GPIO_PIN7,
    #[doc = "0x48 - GPIO_PIN8"]
    pub gpio_pin8: GPIO_PIN8,
    #[doc = "0x4c - GPIO_PIN9"]
    pub gpio_pin9: GPIO_PIN9,
    #[doc = "0x50 - GPIO_PIN10"]
    pub gpio_pin10: GPIO_PIN10,
    #[doc = "0x54 - GPIO_PIN11"]
    pub gpio_pin11: GPIO_PIN11,
    #[doc = "0x58 - GPIO_PIN12"]
    pub gpio_pin12: GPIO_PIN12,
    #[doc = "0x5c - GPIO_PIN13"]
    pub gpio_pin13: GPIO_PIN13,
    #[doc = "0x60 - GPIO_PIN14"]
    pub gpio_pin14: GPIO_PIN14,
    #[doc = "0x64 - GPIO_PIN15"]
    pub gpio_pin15: GPIO_PIN15,
    #[doc = "0x68 - GPIO_SIGMA_DELTA"]
    pub gpio_sigma_delta: GPIO_SIGMA_DELTA,
    #[doc = "0x6c - Positvie edge of this bit will trigger the RTC-clock-calibration process."]
    pub gpio_rtc_calib_sync: GPIO_RTC_CALIB_SYNC,
    #[doc = "0x70 - 0: during RTC-clock-calibration; 1: RTC-clock-calibration is done"]
    pub gpio_rtc_calib_value: GPIO_RTC_CALIB_VALUE,
}
#[doc = "GPIO_OUT (rw) register accessor: an alias for `Reg<GPIO_OUT_SPEC>`"]
pub type GPIO_OUT = crate::Reg<gpio_out::GPIO_OUT_SPEC>;
#[doc = "BT-Coexist Selection register"]
pub mod gpio_out;
#[doc = "GPIO_OUT_W1TS (w) register accessor: an alias for `Reg<GPIO_OUT_W1TS_SPEC>`"]
pub type GPIO_OUT_W1TS = crate::Reg<gpio_out_w1ts::GPIO_OUT_W1TS_SPEC>;
#[doc = "GPIO_OUT_W1TS"]
pub mod gpio_out_w1ts;
#[doc = "GPIO_OUT_W1TC (w) register accessor: an alias for `Reg<GPIO_OUT_W1TC_SPEC>`"]
pub type GPIO_OUT_W1TC = crate::Reg<gpio_out_w1tc::GPIO_OUT_W1TC_SPEC>;
#[doc = "GPIO_OUT_W1TC"]
pub mod gpio_out_w1tc;
#[doc = "GPIO_ENABLE (rw) register accessor: an alias for `Reg<GPIO_ENABLE_SPEC>`"]
pub type GPIO_ENABLE = crate::Reg<gpio_enable::GPIO_ENABLE_SPEC>;
#[doc = "GPIO_ENABLE"]
pub mod gpio_enable;
#[doc = "GPIO_ENABLE_W1TS (w) register accessor: an alias for `Reg<GPIO_ENABLE_W1TS_SPEC>`"]
pub type GPIO_ENABLE_W1TS = crate::Reg<gpio_enable_w1ts::GPIO_ENABLE_W1TS_SPEC>;
#[doc = "GPIO_ENABLE_W1TS"]
pub mod gpio_enable_w1ts;
#[doc = "GPIO_ENABLE_W1TC (w) register accessor: an alias for `Reg<GPIO_ENABLE_W1TC_SPEC>`"]
pub type GPIO_ENABLE_W1TC = crate::Reg<gpio_enable_w1tc::GPIO_ENABLE_W1TC_SPEC>;
#[doc = "GPIO_ENABLE_W1TC"]
pub mod gpio_enable_w1tc;
#[doc = "GPIO_IN (rw) register accessor: an alias for `Reg<GPIO_IN_SPEC>`"]
pub type GPIO_IN = crate::Reg<gpio_in::GPIO_IN_SPEC>;
#[doc = "The values of the strapping pins."]
pub mod gpio_in;
#[doc = "GPIO_STATUS (rw) register accessor: an alias for `Reg<GPIO_STATUS_SPEC>`"]
pub type GPIO_STATUS = crate::Reg<gpio_status::GPIO_STATUS_SPEC>;
#[doc = "GPIO_STATUS"]
pub mod gpio_status;
#[doc = "GPIO_STATUS_W1TS (w) register accessor: an alias for `Reg<GPIO_STATUS_W1TS_SPEC>`"]
pub type GPIO_STATUS_W1TS = crate::Reg<gpio_status_w1ts::GPIO_STATUS_W1TS_SPEC>;
#[doc = "GPIO_STATUS_W1TS"]
pub mod gpio_status_w1ts;
#[doc = "GPIO_STATUS_W1TC (w) register accessor: an alias for `Reg<GPIO_STATUS_W1TC_SPEC>`"]
pub type GPIO_STATUS_W1TC = crate::Reg<gpio_status_w1tc::GPIO_STATUS_W1TC_SPEC>;
#[doc = "GPIO_STATUS_W1TC"]
pub mod gpio_status_w1tc;
#[doc = "GPIO_PIN0 (rw) register accessor: an alias for `Reg<GPIO_PIN0_SPEC>`"]
pub type GPIO_PIN0 = crate::Reg<gpio_pin0::GPIO_PIN0_SPEC>;
#[doc = "GPIO_PIN0"]
pub mod gpio_pin0;
#[doc = "GPIO_PIN1 (rw) register accessor: an alias for `Reg<GPIO_PIN1_SPEC>`"]
pub type GPIO_PIN1 = crate::Reg<gpio_pin1::GPIO_PIN1_SPEC>;
#[doc = "GPIO_PIN1"]
pub mod gpio_pin1;
#[doc = "GPIO_PIN2 (rw) register accessor: an alias for `Reg<GPIO_PIN2_SPEC>`"]
pub type GPIO_PIN2 = crate::Reg<gpio_pin2::GPIO_PIN2_SPEC>;
#[doc = "GPIO_PIN2"]
pub mod gpio_pin2;
#[doc = "GPIO_PIN3 (rw) register accessor: an alias for `Reg<GPIO_PIN3_SPEC>`"]
pub type GPIO_PIN3 = crate::Reg<gpio_pin3::GPIO_PIN3_SPEC>;
#[doc = "GPIO_PIN3"]
pub mod gpio_pin3;
#[doc = "GPIO_PIN4 (rw) register accessor: an alias for `Reg<GPIO_PIN4_SPEC>`"]
pub type GPIO_PIN4 = crate::Reg<gpio_pin4::GPIO_PIN4_SPEC>;
#[doc = "GPIO_PIN4"]
pub mod gpio_pin4;
#[doc = "GPIO_PIN5 (rw) register accessor: an alias for `Reg<GPIO_PIN5_SPEC>`"]
pub type GPIO_PIN5 = crate::Reg<gpio_pin5::GPIO_PIN5_SPEC>;
#[doc = "GPIO_PIN5"]
pub mod gpio_pin5;
#[doc = "GPIO_PIN6 (rw) register accessor: an alias for `Reg<GPIO_PIN6_SPEC>`"]
pub type GPIO_PIN6 = crate::Reg<gpio_pin6::GPIO_PIN6_SPEC>;
#[doc = "GPIO_PIN6"]
pub mod gpio_pin6;
#[doc = "GPIO_PIN7 (rw) register accessor: an alias for `Reg<GPIO_PIN7_SPEC>`"]
pub type GPIO_PIN7 = crate::Reg<gpio_pin7::GPIO_PIN7_SPEC>;
#[doc = "GPIO_PIN7"]
pub mod gpio_pin7;
#[doc = "GPIO_PIN8 (rw) register accessor: an alias for `Reg<GPIO_PIN8_SPEC>`"]
pub type GPIO_PIN8 = crate::Reg<gpio_pin8::GPIO_PIN8_SPEC>;
#[doc = "GPIO_PIN8"]
pub mod gpio_pin8;
#[doc = "GPIO_PIN9 (rw) register accessor: an alias for `Reg<GPIO_PIN9_SPEC>`"]
pub type GPIO_PIN9 = crate::Reg<gpio_pin9::GPIO_PIN9_SPEC>;
#[doc = "GPIO_PIN9"]
pub mod gpio_pin9;
#[doc = "GPIO_PIN10 (rw) register accessor: an alias for `Reg<GPIO_PIN10_SPEC>`"]
pub type GPIO_PIN10 = crate::Reg<gpio_pin10::GPIO_PIN10_SPEC>;
#[doc = "GPIO_PIN10"]
pub mod gpio_pin10;
#[doc = "GPIO_PIN11 (rw) register accessor: an alias for `Reg<GPIO_PIN11_SPEC>`"]
pub type GPIO_PIN11 = crate::Reg<gpio_pin11::GPIO_PIN11_SPEC>;
#[doc = "GPIO_PIN11"]
pub mod gpio_pin11;
#[doc = "GPIO_PIN12 (rw) register accessor: an alias for `Reg<GPIO_PIN12_SPEC>`"]
pub type GPIO_PIN12 = crate::Reg<gpio_pin12::GPIO_PIN12_SPEC>;
#[doc = "GPIO_PIN12"]
pub mod gpio_pin12;
#[doc = "GPIO_PIN13 (rw) register accessor: an alias for `Reg<GPIO_PIN13_SPEC>`"]
pub type GPIO_PIN13 = crate::Reg<gpio_pin13::GPIO_PIN13_SPEC>;
#[doc = "GPIO_PIN13"]
pub mod gpio_pin13;
#[doc = "GPIO_PIN14 (rw) register accessor: an alias for `Reg<GPIO_PIN14_SPEC>`"]
pub type GPIO_PIN14 = crate::Reg<gpio_pin14::GPIO_PIN14_SPEC>;
#[doc = "GPIO_PIN14"]
pub mod gpio_pin14;
#[doc = "GPIO_PIN15 (rw) register accessor: an alias for `Reg<GPIO_PIN15_SPEC>`"]
pub type GPIO_PIN15 = crate::Reg<gpio_pin15::GPIO_PIN15_SPEC>;
#[doc = "GPIO_PIN15"]
pub mod gpio_pin15;
#[doc = "GPIO_SIGMA_DELTA (rw) register accessor: an alias for `Reg<GPIO_SIGMA_DELTA_SPEC>`"]
pub type GPIO_SIGMA_DELTA = crate::Reg<gpio_sigma_delta::GPIO_SIGMA_DELTA_SPEC>;
#[doc = "GPIO_SIGMA_DELTA"]
pub mod gpio_sigma_delta;
#[doc = "GPIO_RTC_CALIB_SYNC (rw) register accessor: an alias for `Reg<GPIO_RTC_CALIB_SYNC_SPEC>`"]
pub type GPIO_RTC_CALIB_SYNC = crate::Reg<gpio_rtc_calib_sync::GPIO_RTC_CALIB_SYNC_SPEC>;
#[doc = "Positvie edge of this bit will trigger the RTC-clock-calibration process."]
pub mod gpio_rtc_calib_sync;
#[doc = "GPIO_RTC_CALIB_VALUE (rw) register accessor: an alias for `Reg<GPIO_RTC_CALIB_VALUE_SPEC>`"]
pub type GPIO_RTC_CALIB_VALUE = crate::Reg<gpio_rtc_calib_value::GPIO_RTC_CALIB_VALUE_SPEC>;
#[doc = "0: during RTC-clock-calibration; 1: RTC-clock-calibration is done"]
pub mod gpio_rtc_calib_value;
