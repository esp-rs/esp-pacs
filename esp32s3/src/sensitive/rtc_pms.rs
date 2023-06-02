#[doc = "Register `RTC_PMS` reader"]
pub struct R(crate::R<RTC_PMS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTC_PMS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTC_PMS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTC_PMS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTC_PMS` writer"]
pub struct W(crate::W<RTC_PMS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTC_PMS_SPEC>;
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
impl From<crate::W<RTC_PMS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTC_PMS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIS_RTC_CPU` reader - Set 1 to disable rtc coprocessor."]
pub type DIS_RTC_CPU_R = crate::BitReader;
#[doc = "Field `DIS_RTC_CPU` writer - Set 1 to disable rtc coprocessor."]
pub type DIS_RTC_CPU_W<'a, const O: u8> = crate::BitWriter<'a, RTC_PMS_SPEC, O>;
impl R {
    #[doc = "Bit 0 - Set 1 to disable rtc coprocessor."]
    #[inline(always)]
    pub fn dis_rtc_cpu(&self) -> DIS_RTC_CPU_R {
        DIS_RTC_CPU_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RTC_PMS")
            .field("dis_rtc_cpu", &format_args!("{}", self.dis_rtc_cpu().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RTC_PMS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Set 1 to disable rtc coprocessor."]
    #[inline(always)]
    #[must_use]
    pub fn dis_rtc_cpu(&mut self) -> DIS_RTC_CPU_W<0> {
        DIS_RTC_CPU_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC coprocessor permission register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_pms](index.html) module"]
pub struct RTC_PMS_SPEC;
impl crate::RegisterSpec for RTC_PMS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtc_pms::R](R) reader structure"]
impl crate::Readable for RTC_PMS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtc_pms::W](W) writer structure"]
impl crate::Writable for RTC_PMS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RTC_PMS to value 0"]
impl crate::Resettable for RTC_PMS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
