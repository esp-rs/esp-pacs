#[doc = "Register `SENSOR_PADS` reader"]
pub struct R(crate::R<SENSOR_PADS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SENSOR_PADS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SENSOR_PADS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SENSOR_PADS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SENSOR_PADS` writer"]
pub struct W(crate::W<SENSOR_PADS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SENSOR_PADS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<SENSOR_PADS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SENSOR_PADS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SENSE4_FUN_IE` reader - the input enable of the pad"]
pub type SENSE4_FUN_IE_R = crate::BitReader;
#[doc = "Field `SENSE4_FUN_IE` writer - the input enable of the pad"]
pub type SENSE4_FUN_IE_W<'a, const O: u8> = crate::BitWriter<'a, SENSOR_PADS_SPEC, O>;
#[doc = "Field `SENSE4_SLP_IE` reader - the input enable of the pad in sleep status"]
pub type SENSE4_SLP_IE_R = crate::BitReader;
#[doc = "Field `SENSE4_SLP_IE` writer - the input enable of the pad in sleep status"]
pub type SENSE4_SLP_IE_W<'a, const O: u8> = crate::BitWriter<'a, SENSOR_PADS_SPEC, O>;
#[doc = "Field `SENSE4_SLP_SEL` reader - the sleep status selection signal of the pad"]
pub type SENSE4_SLP_SEL_R = crate::BitReader;
#[doc = "Field `SENSE4_SLP_SEL` writer - the sleep status selection signal of the pad"]
pub type SENSE4_SLP_SEL_W<'a, const O: u8> = crate::BitWriter<'a, SENSOR_PADS_SPEC, O>;
#[doc = "Field `SENSE4_FUN_SEL` reader - the functional selection signal of the pad"]
pub type SENSE4_FUN_SEL_R = crate::FieldReader;
#[doc = "Field `SENSE4_FUN_SEL` writer - the functional selection signal of the pad"]
pub type SENSE4_FUN_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, SENSOR_PADS_SPEC, 2, O>;
#[doc = "Field `SENSE3_FUN_IE` reader - the input enable of the pad"]
pub type SENSE3_FUN_IE_R = crate::BitReader;
#[doc = "Field `SENSE3_FUN_IE` writer - the input enable of the pad"]
pub type SENSE3_FUN_IE_W<'a, const O: u8> = crate::BitWriter<'a, SENSOR_PADS_SPEC, O>;
#[doc = "Field `SENSE3_SLP_IE` reader - the input enable of the pad in sleep status"]
pub type SENSE3_SLP_IE_R = crate::BitReader;
#[doc = "Field `SENSE3_SLP_IE` writer - the input enable of the pad in sleep status"]
pub type SENSE3_SLP_IE_W<'a, const O: u8> = crate::BitWriter<'a, SENSOR_PADS_SPEC, O>;
#[doc = "Field `SENSE3_SLP_SEL` reader - the sleep status selection signal of the pad"]
pub type SENSE3_SLP_SEL_R = crate::BitReader;
#[doc = "Field `SENSE3_SLP_SEL` writer - the sleep status selection signal of the pad"]
pub type SENSE3_SLP_SEL_W<'a, const O: u8> = crate::BitWriter<'a, SENSOR_PADS_SPEC, O>;
#[doc = "Field `SENSE3_FUN_SEL` reader - the functional selection signal of the pad"]
pub type SENSE3_FUN_SEL_R = crate::FieldReader;
#[doc = "Field `SENSE3_FUN_SEL` writer - the functional selection signal of the pad"]
pub type SENSE3_FUN_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, SENSOR_PADS_SPEC, 2, O>;
#[doc = "Field `SENSE2_FUN_IE` reader - the input enable of the pad"]
pub type SENSE2_FUN_IE_R = crate::BitReader;
#[doc = "Field `SENSE2_FUN_IE` writer - the input enable of the pad"]
pub type SENSE2_FUN_IE_W<'a, const O: u8> = crate::BitWriter<'a, SENSOR_PADS_SPEC, O>;
#[doc = "Field `SENSE2_SLP_IE` reader - the input enable of the pad in sleep status"]
pub type SENSE2_SLP_IE_R = crate::BitReader;
#[doc = "Field `SENSE2_SLP_IE` writer - the input enable of the pad in sleep status"]
pub type SENSE2_SLP_IE_W<'a, const O: u8> = crate::BitWriter<'a, SENSOR_PADS_SPEC, O>;
#[doc = "Field `SENSE2_SLP_SEL` reader - the sleep status selection signal of the pad"]
pub type SENSE2_SLP_SEL_R = crate::BitReader;
#[doc = "Field `SENSE2_SLP_SEL` writer - the sleep status selection signal of the pad"]
pub type SENSE2_SLP_SEL_W<'a, const O: u8> = crate::BitWriter<'a, SENSOR_PADS_SPEC, O>;
#[doc = "Field `SENSE2_FUN_SEL` reader - the functional selection signal of the pad"]
pub type SENSE2_FUN_SEL_R = crate::FieldReader;
#[doc = "Field `SENSE2_FUN_SEL` writer - the functional selection signal of the pad"]
pub type SENSE2_FUN_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, SENSOR_PADS_SPEC, 2, O>;
#[doc = "Field `SENSE1_FUN_IE` reader - the input enable of the pad"]
pub type SENSE1_FUN_IE_R = crate::BitReader;
#[doc = "Field `SENSE1_FUN_IE` writer - the input enable of the pad"]
pub type SENSE1_FUN_IE_W<'a, const O: u8> = crate::BitWriter<'a, SENSOR_PADS_SPEC, O>;
#[doc = "Field `SENSE1_SLP_IE` reader - the input enable of the pad in sleep status"]
pub type SENSE1_SLP_IE_R = crate::BitReader;
#[doc = "Field `SENSE1_SLP_IE` writer - the input enable of the pad in sleep status"]
pub type SENSE1_SLP_IE_W<'a, const O: u8> = crate::BitWriter<'a, SENSOR_PADS_SPEC, O>;
#[doc = "Field `SENSE1_SLP_SEL` reader - the sleep status selection signal of the pad"]
pub type SENSE1_SLP_SEL_R = crate::BitReader;
#[doc = "Field `SENSE1_SLP_SEL` writer - the sleep status selection signal of the pad"]
pub type SENSE1_SLP_SEL_W<'a, const O: u8> = crate::BitWriter<'a, SENSOR_PADS_SPEC, O>;
#[doc = "Field `SENSE1_FUN_SEL` reader - the functional selection signal of the pad"]
pub type SENSE1_FUN_SEL_R = crate::FieldReader;
#[doc = "Field `SENSE1_FUN_SEL` writer - the functional selection signal of the pad"]
pub type SENSE1_FUN_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, SENSOR_PADS_SPEC, 2, O>;
#[doc = "Field `SENSE4_MUX_SEL` reader - Ò1Ó select the digital function Ó0Óslection the rtc function"]
pub type SENSE4_MUX_SEL_R = crate::BitReader;
#[doc = "Field `SENSE4_MUX_SEL` writer - Ò1Ó select the digital function Ó0Óslection the rtc function"]
pub type SENSE4_MUX_SEL_W<'a, const O: u8> = crate::BitWriter<'a, SENSOR_PADS_SPEC, O>;
#[doc = "Field `SENSE3_MUX_SEL` reader - Ò1Ó select the digital function Ó0Óslection the rtc function"]
pub type SENSE3_MUX_SEL_R = crate::BitReader;
#[doc = "Field `SENSE3_MUX_SEL` writer - Ò1Ó select the digital function Ó0Óslection the rtc function"]
pub type SENSE3_MUX_SEL_W<'a, const O: u8> = crate::BitWriter<'a, SENSOR_PADS_SPEC, O>;
#[doc = "Field `SENSE2_MUX_SEL` reader - Ò1Ó select the digital function Ó0Óslection the rtc function"]
pub type SENSE2_MUX_SEL_R = crate::BitReader;
#[doc = "Field `SENSE2_MUX_SEL` writer - Ò1Ó select the digital function Ó0Óslection the rtc function"]
pub type SENSE2_MUX_SEL_W<'a, const O: u8> = crate::BitWriter<'a, SENSOR_PADS_SPEC, O>;
#[doc = "Field `SENSE1_MUX_SEL` reader - Ò1Ó select the digital function Ó0Óslection the rtc function"]
pub type SENSE1_MUX_SEL_R = crate::BitReader;
#[doc = "Field `SENSE1_MUX_SEL` writer - Ò1Ó select the digital function Ó0Óslection the rtc function"]
pub type SENSE1_MUX_SEL_W<'a, const O: u8> = crate::BitWriter<'a, SENSOR_PADS_SPEC, O>;
#[doc = "Field `SENSE4_HOLD` reader - hold the current value of the output when setting the hold to Ò1Ó"]
pub type SENSE4_HOLD_R = crate::BitReader;
#[doc = "Field `SENSE4_HOLD` writer - hold the current value of the output when setting the hold to Ò1Ó"]
pub type SENSE4_HOLD_W<'a, const O: u8> = crate::BitWriter<'a, SENSOR_PADS_SPEC, O>;
#[doc = "Field `SENSE3_HOLD` reader - hold the current value of the output when setting the hold to Ò1Ó"]
pub type SENSE3_HOLD_R = crate::BitReader;
#[doc = "Field `SENSE3_HOLD` writer - hold the current value of the output when setting the hold to Ò1Ó"]
pub type SENSE3_HOLD_W<'a, const O: u8> = crate::BitWriter<'a, SENSOR_PADS_SPEC, O>;
#[doc = "Field `SENSE2_HOLD` reader - hold the current value of the output when setting the hold to Ò1Ó"]
pub type SENSE2_HOLD_R = crate::BitReader;
#[doc = "Field `SENSE2_HOLD` writer - hold the current value of the output when setting the hold to Ò1Ó"]
pub type SENSE2_HOLD_W<'a, const O: u8> = crate::BitWriter<'a, SENSOR_PADS_SPEC, O>;
#[doc = "Field `SENSE1_HOLD` reader - hold the current value of the output when setting the hold to Ò1Ó"]
pub type SENSE1_HOLD_R = crate::BitReader;
#[doc = "Field `SENSE1_HOLD` writer - hold the current value of the output when setting the hold to Ò1Ó"]
pub type SENSE1_HOLD_W<'a, const O: u8> = crate::BitWriter<'a, SENSOR_PADS_SPEC, O>;
impl R {
    #[doc = "Bit 4 - the input enable of the pad"]
    #[inline(always)]
    pub fn sense4_fun_ie(&self) -> SENSE4_FUN_IE_R {
        SENSE4_FUN_IE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - the input enable of the pad in sleep status"]
    #[inline(always)]
    pub fn sense4_slp_ie(&self) -> SENSE4_SLP_IE_R {
        SENSE4_SLP_IE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - the sleep status selection signal of the pad"]
    #[inline(always)]
    pub fn sense4_slp_sel(&self) -> SENSE4_SLP_SEL_R {
        SENSE4_SLP_SEL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:8 - the functional selection signal of the pad"]
    #[inline(always)]
    pub fn sense4_fun_sel(&self) -> SENSE4_FUN_SEL_R {
        SENSE4_FUN_SEL_R::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bit 9 - the input enable of the pad"]
    #[inline(always)]
    pub fn sense3_fun_ie(&self) -> SENSE3_FUN_IE_R {
        SENSE3_FUN_IE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - the input enable of the pad in sleep status"]
    #[inline(always)]
    pub fn sense3_slp_ie(&self) -> SENSE3_SLP_IE_R {
        SENSE3_SLP_IE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - the sleep status selection signal of the pad"]
    #[inline(always)]
    pub fn sense3_slp_sel(&self) -> SENSE3_SLP_SEL_R {
        SENSE3_SLP_SEL_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - the functional selection signal of the pad"]
    #[inline(always)]
    pub fn sense3_fun_sel(&self) -> SENSE3_FUN_SEL_R {
        SENSE3_FUN_SEL_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - the input enable of the pad"]
    #[inline(always)]
    pub fn sense2_fun_ie(&self) -> SENSE2_FUN_IE_R {
        SENSE2_FUN_IE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - the input enable of the pad in sleep status"]
    #[inline(always)]
    pub fn sense2_slp_ie(&self) -> SENSE2_SLP_IE_R {
        SENSE2_SLP_IE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - the sleep status selection signal of the pad"]
    #[inline(always)]
    pub fn sense2_slp_sel(&self) -> SENSE2_SLP_SEL_R {
        SENSE2_SLP_SEL_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:18 - the functional selection signal of the pad"]
    #[inline(always)]
    pub fn sense2_fun_sel(&self) -> SENSE2_FUN_SEL_R {
        SENSE2_FUN_SEL_R::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bit 19 - the input enable of the pad"]
    #[inline(always)]
    pub fn sense1_fun_ie(&self) -> SENSE1_FUN_IE_R {
        SENSE1_FUN_IE_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - the input enable of the pad in sleep status"]
    #[inline(always)]
    pub fn sense1_slp_ie(&self) -> SENSE1_SLP_IE_R {
        SENSE1_SLP_IE_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - the sleep status selection signal of the pad"]
    #[inline(always)]
    pub fn sense1_slp_sel(&self) -> SENSE1_SLP_SEL_R {
        SENSE1_SLP_SEL_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 22:23 - the functional selection signal of the pad"]
    #[inline(always)]
    pub fn sense1_fun_sel(&self) -> SENSE1_FUN_SEL_R {
        SENSE1_FUN_SEL_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bit 24 - Ò1Ó select the digital function Ó0Óslection the rtc function"]
    #[inline(always)]
    pub fn sense4_mux_sel(&self) -> SENSE4_MUX_SEL_R {
        SENSE4_MUX_SEL_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Ò1Ó select the digital function Ó0Óslection the rtc function"]
    #[inline(always)]
    pub fn sense3_mux_sel(&self) -> SENSE3_MUX_SEL_R {
        SENSE3_MUX_SEL_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Ò1Ó select the digital function Ó0Óslection the rtc function"]
    #[inline(always)]
    pub fn sense2_mux_sel(&self) -> SENSE2_MUX_SEL_R {
        SENSE2_MUX_SEL_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Ò1Ó select the digital function Ó0Óslection the rtc function"]
    #[inline(always)]
    pub fn sense1_mux_sel(&self) -> SENSE1_MUX_SEL_R {
        SENSE1_MUX_SEL_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - hold the current value of the output when setting the hold to Ò1Ó"]
    #[inline(always)]
    pub fn sense4_hold(&self) -> SENSE4_HOLD_R {
        SENSE4_HOLD_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - hold the current value of the output when setting the hold to Ò1Ó"]
    #[inline(always)]
    pub fn sense3_hold(&self) -> SENSE3_HOLD_R {
        SENSE3_HOLD_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - hold the current value of the output when setting the hold to Ò1Ó"]
    #[inline(always)]
    pub fn sense2_hold(&self) -> SENSE2_HOLD_R {
        SENSE2_HOLD_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - hold the current value of the output when setting the hold to Ò1Ó"]
    #[inline(always)]
    pub fn sense1_hold(&self) -> SENSE1_HOLD_R {
        SENSE1_HOLD_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SENSOR_PADS")
            .field(
                "sense4_fun_ie",
                &format_args!("{}", self.sense4_fun_ie().bit()),
            )
            .field(
                "sense4_slp_ie",
                &format_args!("{}", self.sense4_slp_ie().bit()),
            )
            .field(
                "sense4_slp_sel",
                &format_args!("{}", self.sense4_slp_sel().bit()),
            )
            .field(
                "sense4_fun_sel",
                &format_args!("{}", self.sense4_fun_sel().bits()),
            )
            .field(
                "sense3_fun_ie",
                &format_args!("{}", self.sense3_fun_ie().bit()),
            )
            .field(
                "sense3_slp_ie",
                &format_args!("{}", self.sense3_slp_ie().bit()),
            )
            .field(
                "sense3_slp_sel",
                &format_args!("{}", self.sense3_slp_sel().bit()),
            )
            .field(
                "sense3_fun_sel",
                &format_args!("{}", self.sense3_fun_sel().bits()),
            )
            .field(
                "sense2_fun_ie",
                &format_args!("{}", self.sense2_fun_ie().bit()),
            )
            .field(
                "sense2_slp_ie",
                &format_args!("{}", self.sense2_slp_ie().bit()),
            )
            .field(
                "sense2_slp_sel",
                &format_args!("{}", self.sense2_slp_sel().bit()),
            )
            .field(
                "sense2_fun_sel",
                &format_args!("{}", self.sense2_fun_sel().bits()),
            )
            .field(
                "sense1_fun_ie",
                &format_args!("{}", self.sense1_fun_ie().bit()),
            )
            .field(
                "sense1_slp_ie",
                &format_args!("{}", self.sense1_slp_ie().bit()),
            )
            .field(
                "sense1_slp_sel",
                &format_args!("{}", self.sense1_slp_sel().bit()),
            )
            .field(
                "sense1_fun_sel",
                &format_args!("{}", self.sense1_fun_sel().bits()),
            )
            .field(
                "sense4_mux_sel",
                &format_args!("{}", self.sense4_mux_sel().bit()),
            )
            .field(
                "sense3_mux_sel",
                &format_args!("{}", self.sense3_mux_sel().bit()),
            )
            .field(
                "sense2_mux_sel",
                &format_args!("{}", self.sense2_mux_sel().bit()),
            )
            .field(
                "sense1_mux_sel",
                &format_args!("{}", self.sense1_mux_sel().bit()),
            )
            .field("sense4_hold", &format_args!("{}", self.sense4_hold().bit()))
            .field("sense3_hold", &format_args!("{}", self.sense3_hold().bit()))
            .field("sense2_hold", &format_args!("{}", self.sense2_hold().bit()))
            .field("sense1_hold", &format_args!("{}", self.sense1_hold().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SENSOR_PADS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 4 - the input enable of the pad"]
    #[inline(always)]
    #[must_use]
    pub fn sense4_fun_ie(&mut self) -> SENSE4_FUN_IE_W<4> {
        SENSE4_FUN_IE_W::new(self)
    }
    #[doc = "Bit 5 - the input enable of the pad in sleep status"]
    #[inline(always)]
    #[must_use]
    pub fn sense4_slp_ie(&mut self) -> SENSE4_SLP_IE_W<5> {
        SENSE4_SLP_IE_W::new(self)
    }
    #[doc = "Bit 6 - the sleep status selection signal of the pad"]
    #[inline(always)]
    #[must_use]
    pub fn sense4_slp_sel(&mut self) -> SENSE4_SLP_SEL_W<6> {
        SENSE4_SLP_SEL_W::new(self)
    }
    #[doc = "Bits 7:8 - the functional selection signal of the pad"]
    #[inline(always)]
    #[must_use]
    pub fn sense4_fun_sel(&mut self) -> SENSE4_FUN_SEL_W<7> {
        SENSE4_FUN_SEL_W::new(self)
    }
    #[doc = "Bit 9 - the input enable of the pad"]
    #[inline(always)]
    #[must_use]
    pub fn sense3_fun_ie(&mut self) -> SENSE3_FUN_IE_W<9> {
        SENSE3_FUN_IE_W::new(self)
    }
    #[doc = "Bit 10 - the input enable of the pad in sleep status"]
    #[inline(always)]
    #[must_use]
    pub fn sense3_slp_ie(&mut self) -> SENSE3_SLP_IE_W<10> {
        SENSE3_SLP_IE_W::new(self)
    }
    #[doc = "Bit 11 - the sleep status selection signal of the pad"]
    #[inline(always)]
    #[must_use]
    pub fn sense3_slp_sel(&mut self) -> SENSE3_SLP_SEL_W<11> {
        SENSE3_SLP_SEL_W::new(self)
    }
    #[doc = "Bits 12:13 - the functional selection signal of the pad"]
    #[inline(always)]
    #[must_use]
    pub fn sense3_fun_sel(&mut self) -> SENSE3_FUN_SEL_W<12> {
        SENSE3_FUN_SEL_W::new(self)
    }
    #[doc = "Bit 14 - the input enable of the pad"]
    #[inline(always)]
    #[must_use]
    pub fn sense2_fun_ie(&mut self) -> SENSE2_FUN_IE_W<14> {
        SENSE2_FUN_IE_W::new(self)
    }
    #[doc = "Bit 15 - the input enable of the pad in sleep status"]
    #[inline(always)]
    #[must_use]
    pub fn sense2_slp_ie(&mut self) -> SENSE2_SLP_IE_W<15> {
        SENSE2_SLP_IE_W::new(self)
    }
    #[doc = "Bit 16 - the sleep status selection signal of the pad"]
    #[inline(always)]
    #[must_use]
    pub fn sense2_slp_sel(&mut self) -> SENSE2_SLP_SEL_W<16> {
        SENSE2_SLP_SEL_W::new(self)
    }
    #[doc = "Bits 17:18 - the functional selection signal of the pad"]
    #[inline(always)]
    #[must_use]
    pub fn sense2_fun_sel(&mut self) -> SENSE2_FUN_SEL_W<17> {
        SENSE2_FUN_SEL_W::new(self)
    }
    #[doc = "Bit 19 - the input enable of the pad"]
    #[inline(always)]
    #[must_use]
    pub fn sense1_fun_ie(&mut self) -> SENSE1_FUN_IE_W<19> {
        SENSE1_FUN_IE_W::new(self)
    }
    #[doc = "Bit 20 - the input enable of the pad in sleep status"]
    #[inline(always)]
    #[must_use]
    pub fn sense1_slp_ie(&mut self) -> SENSE1_SLP_IE_W<20> {
        SENSE1_SLP_IE_W::new(self)
    }
    #[doc = "Bit 21 - the sleep status selection signal of the pad"]
    #[inline(always)]
    #[must_use]
    pub fn sense1_slp_sel(&mut self) -> SENSE1_SLP_SEL_W<21> {
        SENSE1_SLP_SEL_W::new(self)
    }
    #[doc = "Bits 22:23 - the functional selection signal of the pad"]
    #[inline(always)]
    #[must_use]
    pub fn sense1_fun_sel(&mut self) -> SENSE1_FUN_SEL_W<22> {
        SENSE1_FUN_SEL_W::new(self)
    }
    #[doc = "Bit 24 - Ò1Ó select the digital function Ó0Óslection the rtc function"]
    #[inline(always)]
    #[must_use]
    pub fn sense4_mux_sel(&mut self) -> SENSE4_MUX_SEL_W<24> {
        SENSE4_MUX_SEL_W::new(self)
    }
    #[doc = "Bit 25 - Ò1Ó select the digital function Ó0Óslection the rtc function"]
    #[inline(always)]
    #[must_use]
    pub fn sense3_mux_sel(&mut self) -> SENSE3_MUX_SEL_W<25> {
        SENSE3_MUX_SEL_W::new(self)
    }
    #[doc = "Bit 26 - Ò1Ó select the digital function Ó0Óslection the rtc function"]
    #[inline(always)]
    #[must_use]
    pub fn sense2_mux_sel(&mut self) -> SENSE2_MUX_SEL_W<26> {
        SENSE2_MUX_SEL_W::new(self)
    }
    #[doc = "Bit 27 - Ò1Ó select the digital function Ó0Óslection the rtc function"]
    #[inline(always)]
    #[must_use]
    pub fn sense1_mux_sel(&mut self) -> SENSE1_MUX_SEL_W<27> {
        SENSE1_MUX_SEL_W::new(self)
    }
    #[doc = "Bit 28 - hold the current value of the output when setting the hold to Ò1Ó"]
    #[inline(always)]
    #[must_use]
    pub fn sense4_hold(&mut self) -> SENSE4_HOLD_W<28> {
        SENSE4_HOLD_W::new(self)
    }
    #[doc = "Bit 29 - hold the current value of the output when setting the hold to Ò1Ó"]
    #[inline(always)]
    #[must_use]
    pub fn sense3_hold(&mut self) -> SENSE3_HOLD_W<29> {
        SENSE3_HOLD_W::new(self)
    }
    #[doc = "Bit 30 - hold the current value of the output when setting the hold to Ò1Ó"]
    #[inline(always)]
    #[must_use]
    pub fn sense2_hold(&mut self) -> SENSE2_HOLD_W<30> {
        SENSE2_HOLD_W::new(self)
    }
    #[doc = "Bit 31 - hold the current value of the output when setting the hold to Ò1Ó"]
    #[inline(always)]
    #[must_use]
    pub fn sense1_hold(&mut self) -> SENSE1_HOLD_W<31> {
        SENSE1_HOLD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sensor_pads](index.html) module"]
pub struct SENSOR_PADS_SPEC;
impl crate::RegisterSpec for SENSOR_PADS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sensor_pads::R](R) reader structure"]
impl crate::Readable for SENSOR_PADS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sensor_pads::W](W) writer structure"]
impl crate::Writable for SENSOR_PADS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SENSOR_PADS to value 0"]
impl crate::Resettable for SENSOR_PADS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
