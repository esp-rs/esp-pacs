#[doc = "Register `INT_STATUS_REG_2` reader"]
pub struct R(crate::R<INT_STATUS_REG_2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_STATUS_REG_2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_STATUS_REG_2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_STATUS_REG_2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `INT_STATUS_2` reader - Need add description"]
pub type INT_STATUS_2_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Need add description"]
    #[inline(always)]
    pub fn int_status_2(&self) -> INT_STATUS_2_R {
        INT_STATUS_2_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_STATUS_REG_2")
            .field(
                "int_status_2",
                &format_args!("{}", self.int_status_2().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_STATUS_REG_2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "register description\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_status_reg_2](index.html) module"]
pub struct INT_STATUS_REG_2_SPEC;
impl crate::RegisterSpec for INT_STATUS_REG_2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_status_reg_2::R](R) reader structure"]
impl crate::Readable for INT_STATUS_REG_2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INT_STATUS_REG_2 to value 0"]
impl crate::Resettable for INT_STATUS_REG_2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
