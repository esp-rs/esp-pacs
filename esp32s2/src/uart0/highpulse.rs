#[doc = "Register `HIGHPULSE` reader"]
pub struct R(crate::R<HIGHPULSE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HIGHPULSE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HIGHPULSE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HIGHPULSE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MIN_CNT` reader - This register stores the value of the maximum duration time for the high level pulse. It is used in baud rate detection."]
pub type MIN_CNT_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:19 - This register stores the value of the maximum duration time for the high level pulse. It is used in baud rate detection."]
    #[inline(always)]
    pub fn min_cnt(&self) -> MIN_CNT_R {
        MIN_CNT_R::new(self.bits & 0x000f_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HIGHPULSE")
            .field("min_cnt", &format_args!("{}", self.min_cnt().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HIGHPULSE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Autobaud minimum high pulse duration register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [highpulse](index.html) module"]
pub struct HIGHPULSE_SPEC;
impl crate::RegisterSpec for HIGHPULSE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [highpulse::R](R) reader structure"]
impl crate::Readable for HIGHPULSE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets HIGHPULSE to value 0x000f_ffff"]
impl crate::Resettable for HIGHPULSE_SPEC {
    const RESET_VALUE: Self::Ux = 0x000f_ffff;
}
