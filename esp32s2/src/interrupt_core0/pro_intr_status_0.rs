#[doc = "Register `PRO_INTR_STATUS_0` reader"]
pub struct R(crate::R<PRO_INTR_STATUS_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRO_INTR_STATUS_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRO_INTR_STATUS_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRO_INTR_STATUS_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PRO_INTR_STATUS_0` reader - This register stores the status of the first 32 input interrupt sources."]
pub type PRO_INTR_STATUS_0_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - This register stores the status of the first 32 input interrupt sources."]
    #[inline(always)]
    pub fn pro_intr_status_0(&self) -> PRO_INTR_STATUS_0_R {
        PRO_INTR_STATUS_0_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRO_INTR_STATUS_0")
            .field(
                "pro_intr_status_0",
                &format_args!("{}", self.pro_intr_status_0().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PRO_INTR_STATUS_0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Interrupt status register 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pro_intr_status_0](index.html) module"]
pub struct PRO_INTR_STATUS_0_SPEC;
impl crate::RegisterSpec for PRO_INTR_STATUS_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pro_intr_status_0::R](R) reader structure"]
impl crate::Readable for PRO_INTR_STATUS_0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PRO_INTR_STATUS_0 to value 0"]
impl crate::Resettable for PRO_INTR_STATUS_0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
