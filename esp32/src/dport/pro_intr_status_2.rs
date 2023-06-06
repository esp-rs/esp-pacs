#[doc = "Register `PRO_INTR_STATUS_2` reader"]
pub struct R(crate::R<PRO_INTR_STATUS_2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRO_INTR_STATUS_2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRO_INTR_STATUS_2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRO_INTR_STATUS_2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PRO_INTR_STATUS_2` reader - "]
pub type PRO_INTR_STATUS_2_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn pro_intr_status_2(&self) -> PRO_INTR_STATUS_2_R {
        PRO_INTR_STATUS_2_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRO_INTR_STATUS_2")
            .field(
                "pro_intr_status_2",
                &format_args!("{}", self.pro_intr_status_2().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PRO_INTR_STATUS_2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pro_intr_status_2](index.html) module"]
pub struct PRO_INTR_STATUS_2_SPEC;
impl crate::RegisterSpec for PRO_INTR_STATUS_2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pro_intr_status_2::R](R) reader structure"]
impl crate::Readable for PRO_INTR_STATUS_2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PRO_INTR_STATUS_2 to value 0"]
impl crate::Resettable for PRO_INTR_STATUS_2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
