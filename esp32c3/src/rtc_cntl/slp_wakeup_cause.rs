#[doc = "Register `SLP_WAKEUP_CAUSE` reader"]
pub struct R(crate::R<SLP_WAKEUP_CAUSE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SLP_WAKEUP_CAUSE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SLP_WAKEUP_CAUSE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SLP_WAKEUP_CAUSE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `WAKEUP_CAUSE` reader - sleep wakeup cause"]
pub type WAKEUP_CAUSE_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:16 - sleep wakeup cause"]
    #[inline(always)]
    pub fn wakeup_cause(&self) -> WAKEUP_CAUSE_R {
        WAKEUP_CAUSE_R::new(self.bits & 0x0001_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SLP_WAKEUP_CAUSE")
            .field(
                "wakeup_cause",
                &format_args!("{}", self.wakeup_cause().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SLP_WAKEUP_CAUSE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "RTC_CNTL_RTC_SLP_WAKEUP_CAUSE_REG\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [slp_wakeup_cause](index.html) module"]
pub struct SLP_WAKEUP_CAUSE_SPEC;
impl crate::RegisterSpec for SLP_WAKEUP_CAUSE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [slp_wakeup_cause::R](R) reader structure"]
impl crate::Readable for SLP_WAKEUP_CAUSE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SLP_WAKEUP_CAUSE to value 0"]
impl crate::Resettable for SLP_WAKEUP_CAUSE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
