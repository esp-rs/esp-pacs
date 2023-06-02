#[doc = "Register `INTR_STATUS_REG_0` reader"]
pub struct R(crate::R<INTR_STATUS_REG_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTR_STATUS_REG_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTR_STATUS_REG_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTR_STATUS_REG_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `INTR_STATUS_0` reader - Need add description"]
pub type INTR_STATUS_0_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Need add description"]
    #[inline(always)]
    pub fn intr_status_0(&self) -> INTR_STATUS_0_R {
        INTR_STATUS_0_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTR_STATUS_REG_0")
            .field(
                "intr_status_0",
                &format_args!("{}", self.intr_status_0().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INTR_STATUS_REG_0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "register description\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_status_reg_0](index.html) module"]
pub struct INTR_STATUS_REG_0_SPEC;
impl crate::RegisterSpec for INTR_STATUS_REG_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intr_status_reg_0::R](R) reader structure"]
impl crate::Readable for INTR_STATUS_REG_0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INTR_STATUS_REG_0 to value 0"]
impl crate::Resettable for INTR_STATUS_REG_0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
