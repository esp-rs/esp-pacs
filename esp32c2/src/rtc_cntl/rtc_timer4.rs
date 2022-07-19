#[doc = "Register `RTC_TIMER4` reader"]
pub struct R(crate::R<RTC_TIMER4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTC_TIMER4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTC_TIMER4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTC_TIMER4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTC_TIMER4` writer"]
pub struct W(crate::W<RTC_TIMER4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTC_TIMER4_SPEC>;
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
impl From<crate::W<RTC_TIMER4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTC_TIMER4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DG_WRAP_WAIT_TIMER` reader - Need add desc"]
pub type DG_WRAP_WAIT_TIMER_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DG_WRAP_WAIT_TIMER` writer - Need add desc"]
pub type DG_WRAP_WAIT_TIMER_W<'a> = crate::FieldWriter<'a, u32, RTC_TIMER4_SPEC, u16, u16, 9, 16>;
#[doc = "Field `DG_WRAP_POWERUP_TIMER` reader - Need add desc"]
pub type DG_WRAP_POWERUP_TIMER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DG_WRAP_POWERUP_TIMER` writer - Need add desc"]
pub type DG_WRAP_POWERUP_TIMER_W<'a> = crate::FieldWriter<'a, u32, RTC_TIMER4_SPEC, u8, u8, 7, 25>;
impl R {
    #[doc = "Bits 16:24 - Need add desc"]
    #[inline(always)]
    pub fn dg_wrap_wait_timer(&self) -> DG_WRAP_WAIT_TIMER_R {
        DG_WRAP_WAIT_TIMER_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
    #[doc = "Bits 25:31 - Need add desc"]
    #[inline(always)]
    pub fn dg_wrap_powerup_timer(&self) -> DG_WRAP_POWERUP_TIMER_R {
        DG_WRAP_POWERUP_TIMER_R::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 16:24 - Need add desc"]
    #[inline(always)]
    pub fn dg_wrap_wait_timer(&mut self) -> DG_WRAP_WAIT_TIMER_W {
        DG_WRAP_WAIT_TIMER_W::new(self)
    }
    #[doc = "Bits 25:31 - Need add desc"]
    #[inline(always)]
    pub fn dg_wrap_powerup_timer(&mut self) -> DG_WRAP_POWERUP_TIMER_W {
        DG_WRAP_POWERUP_TIMER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "register description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_timer4](index.html) module"]
pub struct RTC_TIMER4_SPEC;
impl crate::RegisterSpec for RTC_TIMER4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtc_timer4::R](R) reader structure"]
impl crate::Readable for RTC_TIMER4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtc_timer4::W](W) writer structure"]
impl crate::Writable for RTC_TIMER4_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTC_TIMER4 to value 0x1020_0000"]
impl crate::Resettable for RTC_TIMER4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1020_0000
    }
}
