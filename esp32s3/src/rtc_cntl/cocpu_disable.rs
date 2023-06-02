#[doc = "Register `COCPU_DISABLE` reader"]
pub struct R(crate::R<COCPU_DISABLE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COCPU_DISABLE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COCPU_DISABLE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COCPU_DISABLE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `COCPU_DISABLE` writer"]
pub struct W(crate::W<COCPU_DISABLE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COCPU_DISABLE_SPEC>;
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
impl From<crate::W<COCPU_DISABLE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COCPU_DISABLE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DISABLE_RTC_CPU` reader - configure ulp diable"]
pub type DISABLE_RTC_CPU_R = crate::BitReader;
#[doc = "Field `DISABLE_RTC_CPU` writer - configure ulp diable"]
pub type DISABLE_RTC_CPU_W<'a, const O: u8> = crate::BitWriter<'a, COCPU_DISABLE_SPEC, O>;
impl R {
    #[doc = "Bit 31 - configure ulp diable"]
    #[inline(always)]
    pub fn disable_rtc_cpu(&self) -> DISABLE_RTC_CPU_R {
        DISABLE_RTC_CPU_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("COCPU_DISABLE")
            .field(
                "disable_rtc_cpu",
                &format_args!("{}", self.disable_rtc_cpu().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<COCPU_DISABLE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 31 - configure ulp diable"]
    #[inline(always)]
    #[must_use]
    pub fn disable_rtc_cpu(&mut self) -> DISABLE_RTC_CPU_W<31> {
        DISABLE_RTC_CPU_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "configure ulp diable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cocpu_disable](index.html) module"]
pub struct COCPU_DISABLE_SPEC;
impl crate::RegisterSpec for COCPU_DISABLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cocpu_disable::R](R) reader structure"]
impl crate::Readable for COCPU_DISABLE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cocpu_disable::W](W) writer structure"]
impl crate::Writable for COCPU_DISABLE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets COCPU_DISABLE to value 0"]
impl crate::Resettable for COCPU_DISABLE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
