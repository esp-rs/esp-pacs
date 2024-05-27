#[doc = "Register `CONF0` reader"]
pub type R = crate::R<CONF0_SPEC>;
#[doc = "Register `CONF0` writer"]
pub type W = crate::W<CONF0_SPEC>;
#[doc = "Field `TIMER_SEL` reader - There are four high speed timers the two bits are used to select one of them for high speed channel0. 2'b00: seletc hstimer0. 2'b01: select hstimer1. 2'b10: select hstimer2. 2'b11: select hstimer3."]
pub type TIMER_SEL_R = crate::FieldReader;
#[doc = "Field `TIMER_SEL` writer - There are four high speed timers the two bits are used to select one of them for high speed channel0. 2'b00: seletc hstimer0. 2'b01: select hstimer1. 2'b10: select hstimer2. 2'b11: select hstimer3."]
pub type TIMER_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SIG_OUT_EN` reader - This is the output enable control bit for high speed channel0"]
pub type SIG_OUT_EN_R = crate::BitReader;
#[doc = "Field `SIG_OUT_EN` writer - This is the output enable control bit for high speed channel0"]
pub type SIG_OUT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDLE_LV` reader - This bit is used to control the output value when high speed channel0 is off."]
pub type IDLE_LV_R = crate::BitReader;
#[doc = "Field `IDLE_LV` writer - This bit is used to control the output value when high speed channel0 is off."]
pub type IDLE_LV_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - There are four high speed timers the two bits are used to select one of them for high speed channel0. 2'b00: seletc hstimer0. 2'b01: select hstimer1. 2'b10: select hstimer2. 2'b11: select hstimer3."]
    #[inline(always)]
    pub fn timer_sel(&self) -> TIMER_SEL_R {
        TIMER_SEL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - This is the output enable control bit for high speed channel0"]
    #[inline(always)]
    pub fn sig_out_en(&self) -> SIG_OUT_EN_R {
        SIG_OUT_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - This bit is used to control the output value when high speed channel0 is off."]
    #[inline(always)]
    pub fn idle_lv(&self) -> IDLE_LV_R {
        IDLE_LV_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONF0")
            .field("timer_sel", &self.timer_sel())
            .field("sig_out_en", &self.sig_out_en())
            .field("idle_lv", &self.idle_lv())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - There are four high speed timers the two bits are used to select one of them for high speed channel0. 2'b00: seletc hstimer0. 2'b01: select hstimer1. 2'b10: select hstimer2. 2'b11: select hstimer3."]
    #[inline(always)]
    #[must_use]
    pub fn timer_sel(&mut self) -> TIMER_SEL_W<CONF0_SPEC> {
        TIMER_SEL_W::new(self, 0)
    }
    #[doc = "Bit 2 - This is the output enable control bit for high speed channel0"]
    #[inline(always)]
    #[must_use]
    pub fn sig_out_en(&mut self) -> SIG_OUT_EN_W<CONF0_SPEC> {
        SIG_OUT_EN_W::new(self, 2)
    }
    #[doc = "Bit 3 - This bit is used to control the output value when high speed channel0 is off."]
    #[inline(always)]
    #[must_use]
    pub fn idle_lv(&mut self) -> IDLE_LV_W<CONF0_SPEC> {
        IDLE_LV_W::new(self, 3)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`conf0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`conf0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CONF0_SPEC;
impl crate::RegisterSpec for CONF0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`conf0::R`](R) reader structure"]
impl crate::Readable for CONF0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`conf0::W`](W) writer structure"]
impl crate::Writable for CONF0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CONF0 to value 0"]
impl crate::Resettable for CONF0_SPEC {
    const RESET_VALUE: u32 = 0;
}
