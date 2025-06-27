#[doc = "Register `SENSOR_PADS` reader"]
pub type R = crate::R<SENSOR_PADS_SPEC>;
#[doc = "Register `SENSOR_PADS` writer"]
pub type W = crate::W<SENSOR_PADS_SPEC>;
#[doc = "Field `SENSE4_FUN_IE` reader - the input enable of the pad"]
pub type SENSE4_FUN_IE_R = crate::BitReader;
#[doc = "Field `SENSE4_FUN_IE` writer - the input enable of the pad"]
pub type SENSE4_FUN_IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SENSE4_SLP_IE` reader - the input enable of the pad in sleep status"]
pub type SENSE4_SLP_IE_R = crate::BitReader;
#[doc = "Field `SENSE4_SLP_IE` writer - the input enable of the pad in sleep status"]
pub type SENSE4_SLP_IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SENSE4_SLP_SEL` reader - the sleep status selection signal of the pad"]
pub type SENSE4_SLP_SEL_R = crate::BitReader;
#[doc = "Field `SENSE4_SLP_SEL` writer - the sleep status selection signal of the pad"]
pub type SENSE4_SLP_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SENSE4_FUN_SEL` reader - the functional selection signal of the pad"]
pub type SENSE4_FUN_SEL_R = crate::FieldReader;
#[doc = "Field `SENSE4_FUN_SEL` writer - the functional selection signal of the pad"]
pub type SENSE4_FUN_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SENSE3_FUN_IE` reader - the input enable of the pad"]
pub type SENSE3_FUN_IE_R = crate::BitReader;
#[doc = "Field `SENSE3_FUN_IE` writer - the input enable of the pad"]
pub type SENSE3_FUN_IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SENSE3_SLP_IE` reader - the input enable of the pad in sleep status"]
pub type SENSE3_SLP_IE_R = crate::BitReader;
#[doc = "Field `SENSE3_SLP_IE` writer - the input enable of the pad in sleep status"]
pub type SENSE3_SLP_IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SENSE3_SLP_SEL` reader - the sleep status selection signal of the pad"]
pub type SENSE3_SLP_SEL_R = crate::BitReader;
#[doc = "Field `SENSE3_SLP_SEL` writer - the sleep status selection signal of the pad"]
pub type SENSE3_SLP_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SENSE3_FUN_SEL` reader - the functional selection signal of the pad"]
pub type SENSE3_FUN_SEL_R = crate::FieldReader;
#[doc = "Field `SENSE3_FUN_SEL` writer - the functional selection signal of the pad"]
pub type SENSE3_FUN_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SENSE2_FUN_IE` reader - the input enable of the pad"]
pub type SENSE2_FUN_IE_R = crate::BitReader;
#[doc = "Field `SENSE2_FUN_IE` writer - the input enable of the pad"]
pub type SENSE2_FUN_IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SENSE2_SLP_IE` reader - the input enable of the pad in sleep status"]
pub type SENSE2_SLP_IE_R = crate::BitReader;
#[doc = "Field `SENSE2_SLP_IE` writer - the input enable of the pad in sleep status"]
pub type SENSE2_SLP_IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SENSE2_SLP_SEL` reader - the sleep status selection signal of the pad"]
pub type SENSE2_SLP_SEL_R = crate::BitReader;
#[doc = "Field `SENSE2_SLP_SEL` writer - the sleep status selection signal of the pad"]
pub type SENSE2_SLP_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SENSE2_FUN_SEL` reader - the functional selection signal of the pad"]
pub type SENSE2_FUN_SEL_R = crate::FieldReader;
#[doc = "Field `SENSE2_FUN_SEL` writer - the functional selection signal of the pad"]
pub type SENSE2_FUN_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SENSE1_FUN_IE` reader - the input enable of the pad"]
pub type SENSE1_FUN_IE_R = crate::BitReader;
#[doc = "Field `SENSE1_FUN_IE` writer - the input enable of the pad"]
pub type SENSE1_FUN_IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SENSE1_SLP_IE` reader - the input enable of the pad in sleep status"]
pub type SENSE1_SLP_IE_R = crate::BitReader;
#[doc = "Field `SENSE1_SLP_IE` writer - the input enable of the pad in sleep status"]
pub type SENSE1_SLP_IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SENSE1_SLP_SEL` reader - the sleep status selection signal of the pad"]
pub type SENSE1_SLP_SEL_R = crate::BitReader;
#[doc = "Field `SENSE1_SLP_SEL` writer - the sleep status selection signal of the pad"]
pub type SENSE1_SLP_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SENSE1_FUN_SEL` reader - the functional selection signal of the pad"]
pub type SENSE1_FUN_SEL_R = crate::FieldReader;
#[doc = "Field `SENSE1_FUN_SEL` writer - the functional selection signal of the pad"]
pub type SENSE1_FUN_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SENSE4_MUX_SEL` reader - Ò1Ó select the digital function Ó0Óslection the rtc function"]
pub type SENSE4_MUX_SEL_R = crate::BitReader;
#[doc = "Field `SENSE4_MUX_SEL` writer - Ò1Ó select the digital function Ó0Óslection the rtc function"]
pub type SENSE4_MUX_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SENSE3_MUX_SEL` reader - Ò1Ó select the digital function Ó0Óslection the rtc function"]
pub type SENSE3_MUX_SEL_R = crate::BitReader;
#[doc = "Field `SENSE3_MUX_SEL` writer - Ò1Ó select the digital function Ó0Óslection the rtc function"]
pub type SENSE3_MUX_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SENSE2_MUX_SEL` reader - Ò1Ó select the digital function Ó0Óslection the rtc function"]
pub type SENSE2_MUX_SEL_R = crate::BitReader;
#[doc = "Field `SENSE2_MUX_SEL` writer - Ò1Ó select the digital function Ó0Óslection the rtc function"]
pub type SENSE2_MUX_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SENSE1_MUX_SEL` reader - Ò1Ó select the digital function Ó0Óslection the rtc function"]
pub type SENSE1_MUX_SEL_R = crate::BitReader;
#[doc = "Field `SENSE1_MUX_SEL` writer - Ò1Ó select the digital function Ó0Óslection the rtc function"]
pub type SENSE1_MUX_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SENSE4_HOLD` reader - hold the current value of the output when setting the hold to Ò1Ó"]
pub type SENSE4_HOLD_R = crate::BitReader;
#[doc = "Field `SENSE4_HOLD` writer - hold the current value of the output when setting the hold to Ò1Ó"]
pub type SENSE4_HOLD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SENSE3_HOLD` reader - hold the current value of the output when setting the hold to Ò1Ó"]
pub type SENSE3_HOLD_R = crate::BitReader;
#[doc = "Field `SENSE3_HOLD` writer - hold the current value of the output when setting the hold to Ò1Ó"]
pub type SENSE3_HOLD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SENSE2_HOLD` reader - hold the current value of the output when setting the hold to Ò1Ó"]
pub type SENSE2_HOLD_R = crate::BitReader;
#[doc = "Field `SENSE2_HOLD` writer - hold the current value of the output when setting the hold to Ò1Ó"]
pub type SENSE2_HOLD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SENSE1_HOLD` reader - hold the current value of the output when setting the hold to Ò1Ó"]
pub type SENSE1_HOLD_R = crate::BitReader;
#[doc = "Field `SENSE1_HOLD` writer - hold the current value of the output when setting the hold to Ò1Ó"]
pub type SENSE1_HOLD_W<'a, REG> = crate::BitWriter<'a, REG>;
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
            .field("sense4_fun_ie", &self.sense4_fun_ie())
            .field("sense4_slp_ie", &self.sense4_slp_ie())
            .field("sense4_slp_sel", &self.sense4_slp_sel())
            .field("sense4_fun_sel", &self.sense4_fun_sel())
            .field("sense3_fun_ie", &self.sense3_fun_ie())
            .field("sense3_slp_ie", &self.sense3_slp_ie())
            .field("sense3_slp_sel", &self.sense3_slp_sel())
            .field("sense3_fun_sel", &self.sense3_fun_sel())
            .field("sense2_fun_ie", &self.sense2_fun_ie())
            .field("sense2_slp_ie", &self.sense2_slp_ie())
            .field("sense2_slp_sel", &self.sense2_slp_sel())
            .field("sense2_fun_sel", &self.sense2_fun_sel())
            .field("sense1_fun_ie", &self.sense1_fun_ie())
            .field("sense1_slp_ie", &self.sense1_slp_ie())
            .field("sense1_slp_sel", &self.sense1_slp_sel())
            .field("sense1_fun_sel", &self.sense1_fun_sel())
            .field("sense4_mux_sel", &self.sense4_mux_sel())
            .field("sense3_mux_sel", &self.sense3_mux_sel())
            .field("sense2_mux_sel", &self.sense2_mux_sel())
            .field("sense1_mux_sel", &self.sense1_mux_sel())
            .field("sense4_hold", &self.sense4_hold())
            .field("sense3_hold", &self.sense3_hold())
            .field("sense2_hold", &self.sense2_hold())
            .field("sense1_hold", &self.sense1_hold())
            .finish()
    }
}
impl W {
    #[doc = "Bit 4 - the input enable of the pad"]
    #[inline(always)]
    pub fn sense4_fun_ie(&mut self) -> SENSE4_FUN_IE_W<SENSOR_PADS_SPEC> {
        SENSE4_FUN_IE_W::new(self, 4)
    }
    #[doc = "Bit 5 - the input enable of the pad in sleep status"]
    #[inline(always)]
    pub fn sense4_slp_ie(&mut self) -> SENSE4_SLP_IE_W<SENSOR_PADS_SPEC> {
        SENSE4_SLP_IE_W::new(self, 5)
    }
    #[doc = "Bit 6 - the sleep status selection signal of the pad"]
    #[inline(always)]
    pub fn sense4_slp_sel(&mut self) -> SENSE4_SLP_SEL_W<SENSOR_PADS_SPEC> {
        SENSE4_SLP_SEL_W::new(self, 6)
    }
    #[doc = "Bits 7:8 - the functional selection signal of the pad"]
    #[inline(always)]
    pub fn sense4_fun_sel(&mut self) -> SENSE4_FUN_SEL_W<SENSOR_PADS_SPEC> {
        SENSE4_FUN_SEL_W::new(self, 7)
    }
    #[doc = "Bit 9 - the input enable of the pad"]
    #[inline(always)]
    pub fn sense3_fun_ie(&mut self) -> SENSE3_FUN_IE_W<SENSOR_PADS_SPEC> {
        SENSE3_FUN_IE_W::new(self, 9)
    }
    #[doc = "Bit 10 - the input enable of the pad in sleep status"]
    #[inline(always)]
    pub fn sense3_slp_ie(&mut self) -> SENSE3_SLP_IE_W<SENSOR_PADS_SPEC> {
        SENSE3_SLP_IE_W::new(self, 10)
    }
    #[doc = "Bit 11 - the sleep status selection signal of the pad"]
    #[inline(always)]
    pub fn sense3_slp_sel(&mut self) -> SENSE3_SLP_SEL_W<SENSOR_PADS_SPEC> {
        SENSE3_SLP_SEL_W::new(self, 11)
    }
    #[doc = "Bits 12:13 - the functional selection signal of the pad"]
    #[inline(always)]
    pub fn sense3_fun_sel(&mut self) -> SENSE3_FUN_SEL_W<SENSOR_PADS_SPEC> {
        SENSE3_FUN_SEL_W::new(self, 12)
    }
    #[doc = "Bit 14 - the input enable of the pad"]
    #[inline(always)]
    pub fn sense2_fun_ie(&mut self) -> SENSE2_FUN_IE_W<SENSOR_PADS_SPEC> {
        SENSE2_FUN_IE_W::new(self, 14)
    }
    #[doc = "Bit 15 - the input enable of the pad in sleep status"]
    #[inline(always)]
    pub fn sense2_slp_ie(&mut self) -> SENSE2_SLP_IE_W<SENSOR_PADS_SPEC> {
        SENSE2_SLP_IE_W::new(self, 15)
    }
    #[doc = "Bit 16 - the sleep status selection signal of the pad"]
    #[inline(always)]
    pub fn sense2_slp_sel(&mut self) -> SENSE2_SLP_SEL_W<SENSOR_PADS_SPEC> {
        SENSE2_SLP_SEL_W::new(self, 16)
    }
    #[doc = "Bits 17:18 - the functional selection signal of the pad"]
    #[inline(always)]
    pub fn sense2_fun_sel(&mut self) -> SENSE2_FUN_SEL_W<SENSOR_PADS_SPEC> {
        SENSE2_FUN_SEL_W::new(self, 17)
    }
    #[doc = "Bit 19 - the input enable of the pad"]
    #[inline(always)]
    pub fn sense1_fun_ie(&mut self) -> SENSE1_FUN_IE_W<SENSOR_PADS_SPEC> {
        SENSE1_FUN_IE_W::new(self, 19)
    }
    #[doc = "Bit 20 - the input enable of the pad in sleep status"]
    #[inline(always)]
    pub fn sense1_slp_ie(&mut self) -> SENSE1_SLP_IE_W<SENSOR_PADS_SPEC> {
        SENSE1_SLP_IE_W::new(self, 20)
    }
    #[doc = "Bit 21 - the sleep status selection signal of the pad"]
    #[inline(always)]
    pub fn sense1_slp_sel(&mut self) -> SENSE1_SLP_SEL_W<SENSOR_PADS_SPEC> {
        SENSE1_SLP_SEL_W::new(self, 21)
    }
    #[doc = "Bits 22:23 - the functional selection signal of the pad"]
    #[inline(always)]
    pub fn sense1_fun_sel(&mut self) -> SENSE1_FUN_SEL_W<SENSOR_PADS_SPEC> {
        SENSE1_FUN_SEL_W::new(self, 22)
    }
    #[doc = "Bit 24 - Ò1Ó select the digital function Ó0Óslection the rtc function"]
    #[inline(always)]
    pub fn sense4_mux_sel(&mut self) -> SENSE4_MUX_SEL_W<SENSOR_PADS_SPEC> {
        SENSE4_MUX_SEL_W::new(self, 24)
    }
    #[doc = "Bit 25 - Ò1Ó select the digital function Ó0Óslection the rtc function"]
    #[inline(always)]
    pub fn sense3_mux_sel(&mut self) -> SENSE3_MUX_SEL_W<SENSOR_PADS_SPEC> {
        SENSE3_MUX_SEL_W::new(self, 25)
    }
    #[doc = "Bit 26 - Ò1Ó select the digital function Ó0Óslection the rtc function"]
    #[inline(always)]
    pub fn sense2_mux_sel(&mut self) -> SENSE2_MUX_SEL_W<SENSOR_PADS_SPEC> {
        SENSE2_MUX_SEL_W::new(self, 26)
    }
    #[doc = "Bit 27 - Ò1Ó select the digital function Ó0Óslection the rtc function"]
    #[inline(always)]
    pub fn sense1_mux_sel(&mut self) -> SENSE1_MUX_SEL_W<SENSOR_PADS_SPEC> {
        SENSE1_MUX_SEL_W::new(self, 27)
    }
    #[doc = "Bit 28 - hold the current value of the output when setting the hold to Ò1Ó"]
    #[inline(always)]
    pub fn sense4_hold(&mut self) -> SENSE4_HOLD_W<SENSOR_PADS_SPEC> {
        SENSE4_HOLD_W::new(self, 28)
    }
    #[doc = "Bit 29 - hold the current value of the output when setting the hold to Ò1Ó"]
    #[inline(always)]
    pub fn sense3_hold(&mut self) -> SENSE3_HOLD_W<SENSOR_PADS_SPEC> {
        SENSE3_HOLD_W::new(self, 29)
    }
    #[doc = "Bit 30 - hold the current value of the output when setting the hold to Ò1Ó"]
    #[inline(always)]
    pub fn sense2_hold(&mut self) -> SENSE2_HOLD_W<SENSOR_PADS_SPEC> {
        SENSE2_HOLD_W::new(self, 30)
    }
    #[doc = "Bit 31 - hold the current value of the output when setting the hold to Ò1Ó"]
    #[inline(always)]
    pub fn sense1_hold(&mut self) -> SENSE1_HOLD_W<SENSOR_PADS_SPEC> {
        SENSE1_HOLD_W::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`sensor_pads::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sensor_pads::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SENSOR_PADS_SPEC;
impl crate::RegisterSpec for SENSOR_PADS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sensor_pads::R`](R) reader structure"]
impl crate::Readable for SENSOR_PADS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sensor_pads::W`](W) writer structure"]
impl crate::Writable for SENSOR_PADS_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SENSOR_PADS to value 0"]
impl crate::Resettable for SENSOR_PADS_SPEC {}
