#[doc = "Register `HSCH%s_CONF0` reader"]
pub struct R(crate::R<HSCH_CONF0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HSCH_CONF0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HSCH_CONF0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HSCH_CONF0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HSCH%s_CONF0` writer"]
pub struct W(crate::W<HSCH_CONF0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HSCH_CONF0_SPEC>;
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
impl From<crate::W<HSCH_CONF0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HSCH_CONF0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIMER_SEL` reader - There are four high speed timers the two bits are used to select one of them for high speed channel0. 2'b00: seletc hstimer0. 2'b01: select hstimer1. 2'b10: select hstimer2. 2'b11: select hstimer3."]
pub type TIMER_SEL_R = crate::FieldReader;
#[doc = "Field `TIMER_SEL` writer - There are four high speed timers the two bits are used to select one of them for high speed channel0. 2'b00: seletc hstimer0. 2'b01: select hstimer1. 2'b10: select hstimer2. 2'b11: select hstimer3."]
pub type TIMER_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, HSCH_CONF0_SPEC, 2, O>;
#[doc = "Field `SIG_OUT_EN` reader - This is the output enable control bit for high speed channel0"]
pub type SIG_OUT_EN_R = crate::BitReader;
#[doc = "Field `SIG_OUT_EN` writer - This is the output enable control bit for high speed channel0"]
pub type SIG_OUT_EN_W<'a, const O: u8> = crate::BitWriter<'a, HSCH_CONF0_SPEC, O>;
#[doc = "Field `IDLE_LV` reader - This bit is used to control the output value when high speed channel0 is off."]
pub type IDLE_LV_R = crate::BitReader;
#[doc = "Field `IDLE_LV` writer - This bit is used to control the output value when high speed channel0 is off."]
pub type IDLE_LV_W<'a, const O: u8> = crate::BitWriter<'a, HSCH_CONF0_SPEC, O>;
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
        f.debug_struct("HSCH_CONF0")
            .field("timer_sel", &format_args!("{}", self.timer_sel().bits()))
            .field("sig_out_en", &format_args!("{}", self.sig_out_en().bit()))
            .field("idle_lv", &format_args!("{}", self.idle_lv().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HSCH_CONF0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:1 - There are four high speed timers the two bits are used to select one of them for high speed channel0. 2'b00: seletc hstimer0. 2'b01: select hstimer1. 2'b10: select hstimer2. 2'b11: select hstimer3."]
    #[inline(always)]
    #[must_use]
    pub fn timer_sel(&mut self) -> TIMER_SEL_W<0> {
        TIMER_SEL_W::new(self)
    }
    #[doc = "Bit 2 - This is the output enable control bit for high speed channel0"]
    #[inline(always)]
    #[must_use]
    pub fn sig_out_en(&mut self) -> SIG_OUT_EN_W<2> {
        SIG_OUT_EN_W::new(self)
    }
    #[doc = "Bit 3 - This bit is used to control the output value when high speed channel0 is off."]
    #[inline(always)]
    #[must_use]
    pub fn idle_lv(&mut self) -> IDLE_LV_W<3> {
        IDLE_LV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hsch_conf0](index.html) module"]
pub struct HSCH_CONF0_SPEC;
impl crate::RegisterSpec for HSCH_CONF0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hsch_conf0::R](R) reader structure"]
impl crate::Readable for HSCH_CONF0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hsch_conf0::W](W) writer structure"]
impl crate::Writable for HSCH_CONF0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HSCH%s_CONF0 to value 0"]
impl crate::Resettable for HSCH_CONF0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
