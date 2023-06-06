#[doc = "Register `SLP_TIMER1` reader"]
pub struct R(crate::R<SLP_TIMER1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SLP_TIMER1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SLP_TIMER1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SLP_TIMER1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SLP_TIMER1` writer"]
pub struct W(crate::W<SLP_TIMER1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SLP_TIMER1_SPEC>;
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
impl From<crate::W<SLP_TIMER1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SLP_TIMER1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLP_VAL_HI` reader - RTC sleep timer high 16 bits"]
pub type SLP_VAL_HI_R = crate::FieldReader<u16>;
#[doc = "Field `SLP_VAL_HI` writer - RTC sleep timer high 16 bits"]
pub type SLP_VAL_HI_W<'a, const O: u8> = crate::FieldWriter<'a, SLP_TIMER1_SPEC, 16, O, u16>;
#[doc = "Field `MAIN_TIMER_ALARM_EN` writer - timer alarm enable bit"]
pub type MAIN_TIMER_ALARM_EN_W<'a, const O: u8> = crate::BitWriter<'a, SLP_TIMER1_SPEC, O>;
impl R {
    #[doc = "Bits 0:15 - RTC sleep timer high 16 bits"]
    #[inline(always)]
    pub fn slp_val_hi(&self) -> SLP_VAL_HI_R {
        SLP_VAL_HI_R::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SLP_TIMER1")
            .field("slp_val_hi", &format_args!("{}", self.slp_val_hi().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SLP_TIMER1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:15 - RTC sleep timer high 16 bits"]
    #[inline(always)]
    #[must_use]
    pub fn slp_val_hi(&mut self) -> SLP_VAL_HI_W<0> {
        SLP_VAL_HI_W::new(self)
    }
    #[doc = "Bit 16 - timer alarm enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn main_timer_alarm_en(&mut self) -> MAIN_TIMER_ALARM_EN_W<16> {
        MAIN_TIMER_ALARM_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "configure sleep time hi\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [slp_timer1](index.html) module"]
pub struct SLP_TIMER1_SPEC;
impl crate::RegisterSpec for SLP_TIMER1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [slp_timer1::R](R) reader structure"]
impl crate::Readable for SLP_TIMER1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [slp_timer1::W](W) writer structure"]
impl crate::Writable for SLP_TIMER1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SLP_TIMER1 to value 0"]
impl crate::Resettable for SLP_TIMER1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
