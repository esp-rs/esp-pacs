#[doc = "Register `ARB_LOST_CAP` reader"]
pub struct R(crate::R<ARB_LOST_CAP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ARB_LOST_CAP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ARB_LOST_CAP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ARB_LOST_CAP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ARBITRATION_LOST_CAPTURE` reader - This register contains information about the bit position of losing arbitration."]
pub type ARBITRATION_LOST_CAPTURE_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:4 - This register contains information about the bit position of losing arbitration."]
    #[inline(always)]
    pub fn arbitration_lost_capture(&self) -> ARBITRATION_LOST_CAPTURE_R {
        ARBITRATION_LOST_CAPTURE_R::new((self.bits & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ARB_LOST_CAP")
            .field(
                "arbitration_lost_capture",
                &format_args!("{}", self.arbitration_lost_capture().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<ARB_LOST_CAP_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "TWAI arbiter lost capture register.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [arb_lost_cap](index.html) module"]
pub struct ARB_LOST_CAP_SPEC;
impl crate::RegisterSpec for ARB_LOST_CAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [arb_lost_cap::R](R) reader structure"]
impl crate::Readable for ARB_LOST_CAP_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ARB_LOST_CAP to value 0"]
impl crate::Resettable for ARB_LOST_CAP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
