#[doc = "Register `TIMER5` reader"]
pub struct R(crate::R<TIMER5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMER5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMER5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMER5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMER5` writer"]
pub struct W(crate::W<TIMER5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMER5_SPEC>;
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
impl From<crate::W<TIMER5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMER5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ULP_CP_SUBTIMER_PREDIV` reader - "]
pub type ULP_CP_SUBTIMER_PREDIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ULP_CP_SUBTIMER_PREDIV` writer - "]
pub type ULP_CP_SUBTIMER_PREDIV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TIMER5_SPEC, u8, u8, 8, O>;
#[doc = "Field `MIN_SLP_VAL` reader - minimal sleep cycles in slow_clk_rtc"]
pub type MIN_SLP_VAL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MIN_SLP_VAL` writer - minimal sleep cycles in slow_clk_rtc"]
pub type MIN_SLP_VAL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIMER5_SPEC, u8, u8, 8, O>;
#[doc = "Field `RTCMEM_WAIT_TIMER` reader - "]
pub type RTCMEM_WAIT_TIMER_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RTCMEM_WAIT_TIMER` writer - "]
pub type RTCMEM_WAIT_TIMER_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TIMER5_SPEC, u16, u16, 9, O>;
#[doc = "Field `RTCMEM_POWERUP_TIMER` reader - "]
pub type RTCMEM_POWERUP_TIMER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RTCMEM_POWERUP_TIMER` writer - "]
pub type RTCMEM_POWERUP_TIMER_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TIMER5_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn ulp_cp_subtimer_prediv(&self) -> ULP_CP_SUBTIMER_PREDIV_R {
        ULP_CP_SUBTIMER_PREDIV_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - minimal sleep cycles in slow_clk_rtc"]
    #[inline(always)]
    pub fn min_slp_val(&self) -> MIN_SLP_VAL_R {
        MIN_SLP_VAL_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:24"]
    #[inline(always)]
    pub fn rtcmem_wait_timer(&self) -> RTCMEM_WAIT_TIMER_R {
        RTCMEM_WAIT_TIMER_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
    #[doc = "Bits 25:31"]
    #[inline(always)]
    pub fn rtcmem_powerup_timer(&self) -> RTCMEM_POWERUP_TIMER_R {
        RTCMEM_POWERUP_TIMER_R::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn ulp_cp_subtimer_prediv(&mut self) -> ULP_CP_SUBTIMER_PREDIV_W<0> {
        ULP_CP_SUBTIMER_PREDIV_W::new(self)
    }
    #[doc = "Bits 8:15 - minimal sleep cycles in slow_clk_rtc"]
    #[inline(always)]
    pub fn min_slp_val(&mut self) -> MIN_SLP_VAL_W<8> {
        MIN_SLP_VAL_W::new(self)
    }
    #[doc = "Bits 16:24"]
    #[inline(always)]
    pub fn rtcmem_wait_timer(&mut self) -> RTCMEM_WAIT_TIMER_W<16> {
        RTCMEM_WAIT_TIMER_W::new(self)
    }
    #[doc = "Bits 25:31"]
    #[inline(always)]
    pub fn rtcmem_powerup_timer(&mut self) -> RTCMEM_POWERUP_TIMER_W<25> {
        RTCMEM_POWERUP_TIMER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timer5](index.html) module"]
pub struct TIMER5_SPEC;
impl crate::RegisterSpec for TIMER5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timer5::R](R) reader structure"]
impl crate::Readable for TIMER5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timer5::W](W) writer structure"]
impl crate::Writable for TIMER5_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TIMER5 to value 0x1214_8001"]
impl crate::Resettable for TIMER5_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1214_8001
    }
}
