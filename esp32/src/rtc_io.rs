#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00 - "]
    pub out: OUT,
    #[doc = "0x04 - "]
    pub out_w1ts: OUT_W1TS,
    #[doc = "0x08 - "]
    pub out_w1tc: OUT_W1TC,
    #[doc = "0x0c - "]
    pub enable: ENABLE,
    #[doc = "0x10 - "]
    pub enable_w1ts: ENABLE_W1TS,
    #[doc = "0x14 - "]
    pub enable_w1tc: ENABLE_W1TC,
    #[doc = "0x18 - "]
    pub status: STATUS,
    #[doc = "0x1c - "]
    pub status_w1ts: STATUS_W1TS,
    #[doc = "0x20 - "]
    pub status_w1tc: STATUS_W1TC,
    #[doc = "0x24 - "]
    pub in_: IN,
    #[doc = "0x28..0x70 - "]
    pub pin: [PIN; 18],
    #[doc = "0x70 - "]
    pub rtc_debug_sel: RTC_DEBUG_SEL,
    #[doc = "0x74 - "]
    pub dig_pad_hold: DIG_PAD_HOLD,
    #[doc = "0x78 - "]
    pub hall_sens: HALL_SENS,
    #[doc = "0x7c - "]
    pub sensor_pads: SENSOR_PADS,
    #[doc = "0x80 - "]
    pub adc_pad: ADC_PAD,
    #[doc = "0x84 - "]
    pub pad_dac1: PAD_DAC1,
    #[doc = "0x88 - "]
    pub pad_dac2: PAD_DAC2,
    #[doc = "0x8c - "]
    pub xtal_32k_pad: XTAL_32K_PAD,
    #[doc = "0x90 - "]
    pub touch_cfg: TOUCH_CFG,
    #[doc = "0x94 - "]
    pub touch_pad0: TOUCH_PAD0,
    #[doc = "0x98 - "]
    pub touch_pad1: TOUCH_PAD1,
    #[doc = "0x9c - "]
    pub touch_pad2: TOUCH_PAD2,
    #[doc = "0xa0 - "]
    pub touch_pad3: TOUCH_PAD3,
    #[doc = "0xa4 - "]
    pub touch_pad4: TOUCH_PAD4,
    #[doc = "0xa8 - "]
    pub touch_pad5: TOUCH_PAD5,
    #[doc = "0xac - "]
    pub touch_pad6: TOUCH_PAD6,
    #[doc = "0xb0 - "]
    pub touch_pad7: TOUCH_PAD7,
    #[doc = "0xb4 - "]
    pub touch_pad8: TOUCH_PAD8,
    #[doc = "0xb8 - "]
    pub touch_pad9: TOUCH_PAD9,
    #[doc = "0xbc - "]
    pub ext_wakeup0: EXT_WAKEUP0,
    #[doc = "0xc0 - "]
    pub xtl_ext_ctr: XTL_EXT_CTR,
    #[doc = "0xc4 - "]
    pub sar_i2c_io: SAR_I2C_IO,
    #[doc = "0xc8 - "]
    pub date: DATE,
}
#[doc = "OUT (rw) register accessor: an alias for `Reg<OUT_SPEC>`"]
pub type OUT = crate::Reg<out::OUT_SPEC>;
#[doc = ""]
pub mod out;
#[doc = "OUT_W1TS (w) register accessor: an alias for `Reg<OUT_W1TS_SPEC>`"]
pub type OUT_W1TS = crate::Reg<out_w1ts::OUT_W1TS_SPEC>;
#[doc = ""]
pub mod out_w1ts;
#[doc = "OUT_W1TC (w) register accessor: an alias for `Reg<OUT_W1TC_SPEC>`"]
pub type OUT_W1TC = crate::Reg<out_w1tc::OUT_W1TC_SPEC>;
#[doc = ""]
pub mod out_w1tc;
#[doc = "ENABLE (rw) register accessor: an alias for `Reg<ENABLE_SPEC>`"]
pub type ENABLE = crate::Reg<enable::ENABLE_SPEC>;
#[doc = ""]
pub mod enable;
#[doc = "ENABLE_W1TS (w) register accessor: an alias for `Reg<ENABLE_W1TS_SPEC>`"]
pub type ENABLE_W1TS = crate::Reg<enable_w1ts::ENABLE_W1TS_SPEC>;
#[doc = ""]
pub mod enable_w1ts;
#[doc = "ENABLE_W1TC (w) register accessor: an alias for `Reg<ENABLE_W1TC_SPEC>`"]
pub type ENABLE_W1TC = crate::Reg<enable_w1tc::ENABLE_W1TC_SPEC>;
#[doc = ""]
pub mod enable_w1tc;
#[doc = "STATUS (rw) register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = ""]
pub mod status;
#[doc = "STATUS_W1TS (w) register accessor: an alias for `Reg<STATUS_W1TS_SPEC>`"]
pub type STATUS_W1TS = crate::Reg<status_w1ts::STATUS_W1TS_SPEC>;
#[doc = ""]
pub mod status_w1ts;
#[doc = "STATUS_W1TC (w) register accessor: an alias for `Reg<STATUS_W1TC_SPEC>`"]
pub type STATUS_W1TC = crate::Reg<status_w1tc::STATUS_W1TC_SPEC>;
#[doc = ""]
pub mod status_w1tc;
#[doc = "IN (r) register accessor: an alias for `Reg<IN_SPEC>`"]
pub type IN = crate::Reg<in_::IN_SPEC>;
#[doc = ""]
pub mod in_;
#[doc = "PIN (rw) register accessor: an alias for `Reg<PIN_SPEC>`"]
pub type PIN = crate::Reg<pin::PIN_SPEC>;
#[doc = ""]
pub mod pin;
#[doc = "RTC_DEBUG_SEL (rw) register accessor: an alias for `Reg<RTC_DEBUG_SEL_SPEC>`"]
pub type RTC_DEBUG_SEL = crate::Reg<rtc_debug_sel::RTC_DEBUG_SEL_SPEC>;
#[doc = ""]
pub mod rtc_debug_sel;
#[doc = "DIG_PAD_HOLD (rw) register accessor: an alias for `Reg<DIG_PAD_HOLD_SPEC>`"]
pub type DIG_PAD_HOLD = crate::Reg<dig_pad_hold::DIG_PAD_HOLD_SPEC>;
#[doc = ""]
pub mod dig_pad_hold;
#[doc = "HALL_SENS (rw) register accessor: an alias for `Reg<HALL_SENS_SPEC>`"]
pub type HALL_SENS = crate::Reg<hall_sens::HALL_SENS_SPEC>;
#[doc = ""]
pub mod hall_sens;
#[doc = "SENSOR_PADS (rw) register accessor: an alias for `Reg<SENSOR_PADS_SPEC>`"]
pub type SENSOR_PADS = crate::Reg<sensor_pads::SENSOR_PADS_SPEC>;
#[doc = ""]
pub mod sensor_pads;
#[doc = "ADC_PAD (rw) register accessor: an alias for `Reg<ADC_PAD_SPEC>`"]
pub type ADC_PAD = crate::Reg<adc_pad::ADC_PAD_SPEC>;
#[doc = ""]
pub mod adc_pad;
#[doc = "PAD_DAC1 (rw) register accessor: an alias for `Reg<PAD_DAC1_SPEC>`"]
pub type PAD_DAC1 = crate::Reg<pad_dac1::PAD_DAC1_SPEC>;
#[doc = ""]
pub mod pad_dac1;
#[doc = "PAD_DAC2 (rw) register accessor: an alias for `Reg<PAD_DAC2_SPEC>`"]
pub type PAD_DAC2 = crate::Reg<pad_dac2::PAD_DAC2_SPEC>;
#[doc = ""]
pub mod pad_dac2;
#[doc = "XTAL_32K_PAD (rw) register accessor: an alias for `Reg<XTAL_32K_PAD_SPEC>`"]
pub type XTAL_32K_PAD = crate::Reg<xtal_32k_pad::XTAL_32K_PAD_SPEC>;
#[doc = ""]
pub mod xtal_32k_pad;
#[doc = "TOUCH_CFG (rw) register accessor: an alias for `Reg<TOUCH_CFG_SPEC>`"]
pub type TOUCH_CFG = crate::Reg<touch_cfg::TOUCH_CFG_SPEC>;
#[doc = ""]
pub mod touch_cfg;
#[doc = "TOUCH_PAD0 (rw) register accessor: an alias for `Reg<TOUCH_PAD0_SPEC>`"]
pub type TOUCH_PAD0 = crate::Reg<touch_pad0::TOUCH_PAD0_SPEC>;
#[doc = ""]
pub mod touch_pad0;
#[doc = "TOUCH_PAD1 (rw) register accessor: an alias for `Reg<TOUCH_PAD1_SPEC>`"]
pub type TOUCH_PAD1 = crate::Reg<touch_pad1::TOUCH_PAD1_SPEC>;
#[doc = ""]
pub mod touch_pad1;
#[doc = "TOUCH_PAD2 (rw) register accessor: an alias for `Reg<TOUCH_PAD2_SPEC>`"]
pub type TOUCH_PAD2 = crate::Reg<touch_pad2::TOUCH_PAD2_SPEC>;
#[doc = ""]
pub mod touch_pad2;
#[doc = "TOUCH_PAD3 (rw) register accessor: an alias for `Reg<TOUCH_PAD3_SPEC>`"]
pub type TOUCH_PAD3 = crate::Reg<touch_pad3::TOUCH_PAD3_SPEC>;
#[doc = ""]
pub mod touch_pad3;
#[doc = "TOUCH_PAD4 (rw) register accessor: an alias for `Reg<TOUCH_PAD4_SPEC>`"]
pub type TOUCH_PAD4 = crate::Reg<touch_pad4::TOUCH_PAD4_SPEC>;
#[doc = ""]
pub mod touch_pad4;
#[doc = "TOUCH_PAD5 (rw) register accessor: an alias for `Reg<TOUCH_PAD5_SPEC>`"]
pub type TOUCH_PAD5 = crate::Reg<touch_pad5::TOUCH_PAD5_SPEC>;
#[doc = ""]
pub mod touch_pad5;
#[doc = "TOUCH_PAD6 (rw) register accessor: an alias for `Reg<TOUCH_PAD6_SPEC>`"]
pub type TOUCH_PAD6 = crate::Reg<touch_pad6::TOUCH_PAD6_SPEC>;
#[doc = ""]
pub mod touch_pad6;
#[doc = "TOUCH_PAD7 (rw) register accessor: an alias for `Reg<TOUCH_PAD7_SPEC>`"]
pub type TOUCH_PAD7 = crate::Reg<touch_pad7::TOUCH_PAD7_SPEC>;
#[doc = ""]
pub mod touch_pad7;
#[doc = "TOUCH_PAD8 (rw) register accessor: an alias for `Reg<TOUCH_PAD8_SPEC>`"]
pub type TOUCH_PAD8 = crate::Reg<touch_pad8::TOUCH_PAD8_SPEC>;
#[doc = ""]
pub mod touch_pad8;
#[doc = "TOUCH_PAD9 (rw) register accessor: an alias for `Reg<TOUCH_PAD9_SPEC>`"]
pub type TOUCH_PAD9 = crate::Reg<touch_pad9::TOUCH_PAD9_SPEC>;
#[doc = ""]
pub mod touch_pad9;
#[doc = "EXT_WAKEUP0 (rw) register accessor: an alias for `Reg<EXT_WAKEUP0_SPEC>`"]
pub type EXT_WAKEUP0 = crate::Reg<ext_wakeup0::EXT_WAKEUP0_SPEC>;
#[doc = ""]
pub mod ext_wakeup0;
#[doc = "XTL_EXT_CTR (rw) register accessor: an alias for `Reg<XTL_EXT_CTR_SPEC>`"]
pub type XTL_EXT_CTR = crate::Reg<xtl_ext_ctr::XTL_EXT_CTR_SPEC>;
#[doc = ""]
pub mod xtl_ext_ctr;
#[doc = "SAR_I2C_IO (rw) register accessor: an alias for `Reg<SAR_I2C_IO_SPEC>`"]
pub type SAR_I2C_IO = crate::Reg<sar_i2c_io::SAR_I2C_IO_SPEC>;
#[doc = ""]
pub mod sar_i2c_io;
#[doc = "DATE (rw) register accessor: an alias for `Reg<DATE_SPEC>`"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = ""]
pub mod date;
