#[doc = "Register `TIMERS_DATE` reader"]
pub struct R(crate::R<TIMERS_DATE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMERS_DATE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMERS_DATE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMERS_DATE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMERS_DATE` writer"]
pub struct W(crate::W<TIMERS_DATE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMERS_DATE_SPEC>;
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
impl From<crate::W<TIMERS_DATE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMERS_DATE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIMERS_DATE` reader - Version control register."]
pub type TIMERS_DATE_R = crate::FieldReader<u32, u32>;
#[doc = "Field `TIMERS_DATE` writer - Version control register."]
pub type TIMERS_DATE_W<'a, const O: u8> = crate::FieldWriter<'a, TIMERS_DATE_SPEC, 28, O, u32, u32>;
impl R {
    #[doc = "Bits 0:27 - Version control register."]
    #[inline(always)]
    pub fn timers_date(&self) -> TIMERS_DATE_R {
        TIMERS_DATE_R::new(self.bits & 0x0fff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMERS_DATE")
            .field(
                "timers_date",
                &format_args!("{}", self.timers_date().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TIMERS_DATE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:27 - Version control register."]
    #[inline(always)]
    #[must_use]
    pub fn timers_date(&mut self) -> TIMERS_DATE_W<0> {
        TIMERS_DATE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Version control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timers_date](index.html) module"]
pub struct TIMERS_DATE_SPEC;
impl crate::RegisterSpec for TIMERS_DATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timers_date::R](R) reader structure"]
impl crate::Readable for TIMERS_DATE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timers_date::W](W) writer structure"]
impl crate::Writable for TIMERS_DATE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIMERS_DATE to value 0x0190_7261"]
impl crate::Resettable for TIMERS_DATE_SPEC {
    const RESET_VALUE: Self::Ux = 0x0190_7261;
}
