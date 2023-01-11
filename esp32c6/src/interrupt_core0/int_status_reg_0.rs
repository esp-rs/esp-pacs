#[doc = "Register `INT_STATUS_REG_0` reader"]
pub struct R(crate::R<INT_STATUS_REG_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_STATUS_REG_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_STATUS_REG_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_STATUS_REG_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `INT_STATUS_0` reader - Need add description"]
pub type INT_STATUS_0_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Need add description"]
    #[inline(always)]
    pub fn int_status_0(&self) -> INT_STATUS_0_R {
        INT_STATUS_0_R::new(self.bits)
    }
}
#[doc = "register description\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_status_reg_0](index.html) module"]
pub struct INT_STATUS_REG_0_SPEC;
impl crate::RegisterSpec for INT_STATUS_REG_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_status_reg_0::R](R) reader structure"]
impl crate::Readable for INT_STATUS_REG_0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INT_STATUS_REG_0 to value 0"]
impl crate::Resettable for INT_STATUS_REG_0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
