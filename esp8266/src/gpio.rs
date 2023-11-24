#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    gpio_out: GPIO_OUT,
    gpio_out_w1ts: GPIO_OUT_W1TS,
    gpio_out_w1tc: GPIO_OUT_W1TC,
    gpio_enable: GPIO_ENABLE,
    gpio_enable_w1ts: GPIO_ENABLE_W1TS,
    gpio_enable_w1tc: GPIO_ENABLE_W1TC,
    gpio_in: GPIO_IN,
    gpio_status: GPIO_STATUS,
    gpio_status_w1ts: GPIO_STATUS_W1TS,
    gpio_status_w1tc: GPIO_STATUS_W1TC,
    gpio_pin0: GPIO_PIN0,
    gpio_pin1: GPIO_PIN1,
    gpio_pin2: GPIO_PIN2,
    gpio_pin3: GPIO_PIN3,
    gpio_pin4: GPIO_PIN4,
    gpio_pin5: GPIO_PIN5,
    gpio_pin6: GPIO_PIN6,
    gpio_pin7: GPIO_PIN7,
    gpio_pin8: GPIO_PIN8,
    gpio_pin9: GPIO_PIN9,
    gpio_pin10: GPIO_PIN10,
    gpio_pin11: GPIO_PIN11,
    gpio_pin12: GPIO_PIN12,
    gpio_pin13: GPIO_PIN13,
    gpio_pin14: GPIO_PIN14,
    gpio_pin15: GPIO_PIN15,
    gpio_sigma_delta: GPIO_SIGMA_DELTA,
    gpio_rtc_calib_sync: GPIO_RTC_CALIB_SYNC,
    gpio_rtc_calib_value: GPIO_RTC_CALIB_VALUE,
}
impl RegisterBlock {
    #[doc = "0x00 - BT-Coexist Selection register"]
    #[inline(always)]
    pub const fn gpio_out(&self) -> &GPIO_OUT {
        &self.gpio_out
    }
    #[doc = "0x04 - GPIO_OUT_W1TS"]
    #[inline(always)]
    pub const fn gpio_out_w1ts(&self) -> &GPIO_OUT_W1TS {
        &self.gpio_out_w1ts
    }
    #[doc = "0x08 - GPIO_OUT_W1TC"]
    #[inline(always)]
    pub const fn gpio_out_w1tc(&self) -> &GPIO_OUT_W1TC {
        &self.gpio_out_w1tc
    }
    #[doc = "0x0c - GPIO_ENABLE"]
    #[inline(always)]
    pub const fn gpio_enable(&self) -> &GPIO_ENABLE {
        &self.gpio_enable
    }
    #[doc = "0x10 - GPIO_ENABLE_W1TS"]
    #[inline(always)]
    pub const fn gpio_enable_w1ts(&self) -> &GPIO_ENABLE_W1TS {
        &self.gpio_enable_w1ts
    }
    #[doc = "0x14 - GPIO_ENABLE_W1TC"]
    #[inline(always)]
    pub const fn gpio_enable_w1tc(&self) -> &GPIO_ENABLE_W1TC {
        &self.gpio_enable_w1tc
    }
    #[doc = "0x18 - The values of the strapping pins."]
    #[inline(always)]
    pub const fn gpio_in(&self) -> &GPIO_IN {
        &self.gpio_in
    }
    #[doc = "0x1c - GPIO_STATUS"]
    #[inline(always)]
    pub const fn gpio_status(&self) -> &GPIO_STATUS {
        &self.gpio_status
    }
    #[doc = "0x20 - GPIO_STATUS_W1TS"]
    #[inline(always)]
    pub const fn gpio_status_w1ts(&self) -> &GPIO_STATUS_W1TS {
        &self.gpio_status_w1ts
    }
    #[doc = "0x24 - GPIO_STATUS_W1TC"]
    #[inline(always)]
    pub const fn gpio_status_w1tc(&self) -> &GPIO_STATUS_W1TC {
        &self.gpio_status_w1tc
    }
    #[doc = "0x28 - GPIO_PIN0"]
    #[inline(always)]
    pub const fn gpio_pin0(&self) -> &GPIO_PIN0 {
        &self.gpio_pin0
    }
    #[doc = "0x2c - GPIO_PIN1"]
    #[inline(always)]
    pub const fn gpio_pin1(&self) -> &GPIO_PIN1 {
        &self.gpio_pin1
    }
    #[doc = "0x30 - GPIO_PIN2"]
    #[inline(always)]
    pub const fn gpio_pin2(&self) -> &GPIO_PIN2 {
        &self.gpio_pin2
    }
    #[doc = "0x34 - GPIO_PIN3"]
    #[inline(always)]
    pub const fn gpio_pin3(&self) -> &GPIO_PIN3 {
        &self.gpio_pin3
    }
    #[doc = "0x38 - GPIO_PIN4"]
    #[inline(always)]
    pub const fn gpio_pin4(&self) -> &GPIO_PIN4 {
        &self.gpio_pin4
    }
    #[doc = "0x3c - GPIO_PIN5"]
    #[inline(always)]
    pub const fn gpio_pin5(&self) -> &GPIO_PIN5 {
        &self.gpio_pin5
    }
    #[doc = "0x40 - GPIO_PIN6"]
    #[inline(always)]
    pub const fn gpio_pin6(&self) -> &GPIO_PIN6 {
        &self.gpio_pin6
    }
    #[doc = "0x44 - GPIO_PIN7"]
    #[inline(always)]
    pub const fn gpio_pin7(&self) -> &GPIO_PIN7 {
        &self.gpio_pin7
    }
    #[doc = "0x48 - GPIO_PIN8"]
    #[inline(always)]
    pub const fn gpio_pin8(&self) -> &GPIO_PIN8 {
        &self.gpio_pin8
    }
    #[doc = "0x4c - GPIO_PIN9"]
    #[inline(always)]
    pub const fn gpio_pin9(&self) -> &GPIO_PIN9 {
        &self.gpio_pin9
    }
    #[doc = "0x50 - GPIO_PIN10"]
    #[inline(always)]
    pub const fn gpio_pin10(&self) -> &GPIO_PIN10 {
        &self.gpio_pin10
    }
    #[doc = "0x54 - GPIO_PIN11"]
    #[inline(always)]
    pub const fn gpio_pin11(&self) -> &GPIO_PIN11 {
        &self.gpio_pin11
    }
    #[doc = "0x58 - GPIO_PIN12"]
    #[inline(always)]
    pub const fn gpio_pin12(&self) -> &GPIO_PIN12 {
        &self.gpio_pin12
    }
    #[doc = "0x5c - GPIO_PIN13"]
    #[inline(always)]
    pub const fn gpio_pin13(&self) -> &GPIO_PIN13 {
        &self.gpio_pin13
    }
    #[doc = "0x60 - GPIO_PIN14"]
    #[inline(always)]
    pub const fn gpio_pin14(&self) -> &GPIO_PIN14 {
        &self.gpio_pin14
    }
    #[doc = "0x64 - GPIO_PIN15"]
    #[inline(always)]
    pub const fn gpio_pin15(&self) -> &GPIO_PIN15 {
        &self.gpio_pin15
    }
    #[doc = "0x68 - GPIO_SIGMA_DELTA"]
    #[inline(always)]
    pub const fn gpio_sigma_delta(&self) -> &GPIO_SIGMA_DELTA {
        &self.gpio_sigma_delta
    }
    #[doc = "0x6c - Positvie edge of this bit will trigger the RTC-clock-calibration process."]
    #[inline(always)]
    pub const fn gpio_rtc_calib_sync(&self) -> &GPIO_RTC_CALIB_SYNC {
        &self.gpio_rtc_calib_sync
    }
    #[doc = "0x70 - 0: during RTC-clock-calibration; 1: RTC-clock-calibration is done"]
    #[inline(always)]
    pub const fn gpio_rtc_calib_value(&self) -> &GPIO_RTC_CALIB_VALUE {
        &self.gpio_rtc_calib_value
    }
}
#[doc = "GPIO_OUT (rw) register accessor: BT-Coexist Selection register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_out::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_out::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_out`] module"]
pub type GPIO_OUT = crate::Reg<gpio_out::GPIO_OUT_SPEC>;
#[doc = "BT-Coexist Selection register"]
pub mod gpio_out;
#[doc = "GPIO_OUT_W1TS (w) register accessor: GPIO_OUT_W1TS\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_out_w1ts::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_out_w1ts`] module"]
pub type GPIO_OUT_W1TS = crate::Reg<gpio_out_w1ts::GPIO_OUT_W1TS_SPEC>;
#[doc = "GPIO_OUT_W1TS"]
pub mod gpio_out_w1ts;
#[doc = "GPIO_OUT_W1TC (w) register accessor: GPIO_OUT_W1TC\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_out_w1tc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_out_w1tc`] module"]
pub type GPIO_OUT_W1TC = crate::Reg<gpio_out_w1tc::GPIO_OUT_W1TC_SPEC>;
#[doc = "GPIO_OUT_W1TC"]
pub mod gpio_out_w1tc;
#[doc = "GPIO_ENABLE (rw) register accessor: GPIO_ENABLE\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_enable::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_enable::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_enable`] module"]
pub type GPIO_ENABLE = crate::Reg<gpio_enable::GPIO_ENABLE_SPEC>;
#[doc = "GPIO_ENABLE"]
pub mod gpio_enable;
#[doc = "GPIO_ENABLE_W1TS (w) register accessor: GPIO_ENABLE_W1TS\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_enable_w1ts::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_enable_w1ts`] module"]
pub type GPIO_ENABLE_W1TS = crate::Reg<gpio_enable_w1ts::GPIO_ENABLE_W1TS_SPEC>;
#[doc = "GPIO_ENABLE_W1TS"]
pub mod gpio_enable_w1ts;
#[doc = "GPIO_ENABLE_W1TC (w) register accessor: GPIO_ENABLE_W1TC\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_enable_w1tc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_enable_w1tc`] module"]
pub type GPIO_ENABLE_W1TC = crate::Reg<gpio_enable_w1tc::GPIO_ENABLE_W1TC_SPEC>;
#[doc = "GPIO_ENABLE_W1TC"]
pub mod gpio_enable_w1tc;
#[doc = "GPIO_IN (rw) register accessor: The values of the strapping pins.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_in::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_in::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_in`] module"]
pub type GPIO_IN = crate::Reg<gpio_in::GPIO_IN_SPEC>;
#[doc = "The values of the strapping pins."]
pub mod gpio_in;
#[doc = "GPIO_STATUS (rw) register accessor: GPIO_STATUS\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_status`] module"]
pub type GPIO_STATUS = crate::Reg<gpio_status::GPIO_STATUS_SPEC>;
#[doc = "GPIO_STATUS"]
pub mod gpio_status;
#[doc = "GPIO_STATUS_W1TS (w) register accessor: GPIO_STATUS_W1TS\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_status_w1ts::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_status_w1ts`] module"]
pub type GPIO_STATUS_W1TS = crate::Reg<gpio_status_w1ts::GPIO_STATUS_W1TS_SPEC>;
#[doc = "GPIO_STATUS_W1TS"]
pub mod gpio_status_w1ts;
#[doc = "GPIO_STATUS_W1TC (w) register accessor: GPIO_STATUS_W1TC\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_status_w1tc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_status_w1tc`] module"]
pub type GPIO_STATUS_W1TC = crate::Reg<gpio_status_w1tc::GPIO_STATUS_W1TC_SPEC>;
#[doc = "GPIO_STATUS_W1TC"]
pub mod gpio_status_w1tc;
#[doc = "GPIO_PIN0 (rw) register accessor: GPIO_PIN0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_pin0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_pin0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_pin0`] module"]
pub type GPIO_PIN0 = crate::Reg<gpio_pin0::GPIO_PIN0_SPEC>;
#[doc = "GPIO_PIN0"]
pub mod gpio_pin0;
#[doc = "GPIO_PIN1 (rw) register accessor: GPIO_PIN1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_pin1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_pin1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_pin1`] module"]
pub type GPIO_PIN1 = crate::Reg<gpio_pin1::GPIO_PIN1_SPEC>;
#[doc = "GPIO_PIN1"]
pub mod gpio_pin1;
#[doc = "GPIO_PIN2 (rw) register accessor: GPIO_PIN2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_pin2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_pin2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_pin2`] module"]
pub type GPIO_PIN2 = crate::Reg<gpio_pin2::GPIO_PIN2_SPEC>;
#[doc = "GPIO_PIN2"]
pub mod gpio_pin2;
#[doc = "GPIO_PIN3 (rw) register accessor: GPIO_PIN3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_pin3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_pin3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_pin3`] module"]
pub type GPIO_PIN3 = crate::Reg<gpio_pin3::GPIO_PIN3_SPEC>;
#[doc = "GPIO_PIN3"]
pub mod gpio_pin3;
#[doc = "GPIO_PIN4 (rw) register accessor: GPIO_PIN4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_pin4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_pin4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_pin4`] module"]
pub type GPIO_PIN4 = crate::Reg<gpio_pin4::GPIO_PIN4_SPEC>;
#[doc = "GPIO_PIN4"]
pub mod gpio_pin4;
#[doc = "GPIO_PIN5 (rw) register accessor: GPIO_PIN5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_pin5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_pin5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_pin5`] module"]
pub type GPIO_PIN5 = crate::Reg<gpio_pin5::GPIO_PIN5_SPEC>;
#[doc = "GPIO_PIN5"]
pub mod gpio_pin5;
#[doc = "GPIO_PIN6 (rw) register accessor: GPIO_PIN6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_pin6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_pin6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_pin6`] module"]
pub type GPIO_PIN6 = crate::Reg<gpio_pin6::GPIO_PIN6_SPEC>;
#[doc = "GPIO_PIN6"]
pub mod gpio_pin6;
#[doc = "GPIO_PIN7 (rw) register accessor: GPIO_PIN7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_pin7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_pin7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_pin7`] module"]
pub type GPIO_PIN7 = crate::Reg<gpio_pin7::GPIO_PIN7_SPEC>;
#[doc = "GPIO_PIN7"]
pub mod gpio_pin7;
#[doc = "GPIO_PIN8 (rw) register accessor: GPIO_PIN8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_pin8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_pin8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_pin8`] module"]
pub type GPIO_PIN8 = crate::Reg<gpio_pin8::GPIO_PIN8_SPEC>;
#[doc = "GPIO_PIN8"]
pub mod gpio_pin8;
#[doc = "GPIO_PIN9 (rw) register accessor: GPIO_PIN9\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_pin9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_pin9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_pin9`] module"]
pub type GPIO_PIN9 = crate::Reg<gpio_pin9::GPIO_PIN9_SPEC>;
#[doc = "GPIO_PIN9"]
pub mod gpio_pin9;
#[doc = "GPIO_PIN10 (rw) register accessor: GPIO_PIN10\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_pin10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_pin10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_pin10`] module"]
pub type GPIO_PIN10 = crate::Reg<gpio_pin10::GPIO_PIN10_SPEC>;
#[doc = "GPIO_PIN10"]
pub mod gpio_pin10;
#[doc = "GPIO_PIN11 (rw) register accessor: GPIO_PIN11\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_pin11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_pin11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_pin11`] module"]
pub type GPIO_PIN11 = crate::Reg<gpio_pin11::GPIO_PIN11_SPEC>;
#[doc = "GPIO_PIN11"]
pub mod gpio_pin11;
#[doc = "GPIO_PIN12 (rw) register accessor: GPIO_PIN12\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_pin12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_pin12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_pin12`] module"]
pub type GPIO_PIN12 = crate::Reg<gpio_pin12::GPIO_PIN12_SPEC>;
#[doc = "GPIO_PIN12"]
pub mod gpio_pin12;
#[doc = "GPIO_PIN13 (rw) register accessor: GPIO_PIN13\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_pin13::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_pin13::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_pin13`] module"]
pub type GPIO_PIN13 = crate::Reg<gpio_pin13::GPIO_PIN13_SPEC>;
#[doc = "GPIO_PIN13"]
pub mod gpio_pin13;
#[doc = "GPIO_PIN14 (rw) register accessor: GPIO_PIN14\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_pin14::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_pin14::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_pin14`] module"]
pub type GPIO_PIN14 = crate::Reg<gpio_pin14::GPIO_PIN14_SPEC>;
#[doc = "GPIO_PIN14"]
pub mod gpio_pin14;
#[doc = "GPIO_PIN15 (rw) register accessor: GPIO_PIN15\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_pin15::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_pin15::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_pin15`] module"]
pub type GPIO_PIN15 = crate::Reg<gpio_pin15::GPIO_PIN15_SPEC>;
#[doc = "GPIO_PIN15"]
pub mod gpio_pin15;
#[doc = "GPIO_SIGMA_DELTA (rw) register accessor: GPIO_SIGMA_DELTA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_sigma_delta::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_sigma_delta::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_sigma_delta`] module"]
pub type GPIO_SIGMA_DELTA = crate::Reg<gpio_sigma_delta::GPIO_SIGMA_DELTA_SPEC>;
#[doc = "GPIO_SIGMA_DELTA"]
pub mod gpio_sigma_delta;
#[doc = "GPIO_RTC_CALIB_SYNC (rw) register accessor: Positvie edge of this bit will trigger the RTC-clock-calibration process.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_rtc_calib_sync::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_rtc_calib_sync::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_rtc_calib_sync`] module"]
pub type GPIO_RTC_CALIB_SYNC = crate::Reg<gpio_rtc_calib_sync::GPIO_RTC_CALIB_SYNC_SPEC>;
#[doc = "Positvie edge of this bit will trigger the RTC-clock-calibration process."]
pub mod gpio_rtc_calib_sync;
#[doc = "GPIO_RTC_CALIB_VALUE (rw) register accessor: 0: during RTC-clock-calibration; 1: RTC-clock-calibration is done\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_rtc_calib_value::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_rtc_calib_value::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_rtc_calib_value`] module"]
pub type GPIO_RTC_CALIB_VALUE = crate::Reg<gpio_rtc_calib_value::GPIO_RTC_CALIB_VALUE_SPEC>;
#[doc = "0: during RTC-clock-calibration; 1: RTC-clock-calibration is done"]
pub mod gpio_rtc_calib_value;
