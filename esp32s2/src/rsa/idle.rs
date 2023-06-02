#[doc = "Register `IDLE` reader"]
pub struct R(crate::R<IDLE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IDLE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IDLE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IDLE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `IDLE` reader - The content of this bit is 1 when the RSA accelerator is idle."]
pub type IDLE_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - The content of this bit is 1 when the RSA accelerator is idle."]
    #[inline(always)]
    pub fn idle(&self) -> IDLE_R {
        IDLE_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IDLE")
            .field("idle", &format_args!("{}", self.idle().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<IDLE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "RSA idle register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idle](index.html) module"]
pub struct IDLE_SPEC;
impl crate::RegisterSpec for IDLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [idle::R](R) reader structure"]
impl crate::Readable for IDLE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IDLE to value 0"]
impl crate::Resettable for IDLE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
