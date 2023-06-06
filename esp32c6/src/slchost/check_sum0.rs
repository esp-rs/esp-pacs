#[doc = "Register `CHECK_SUM0` reader"]
pub struct R(crate::R<CHECK_SUM0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHECK_SUM0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHECK_SUM0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHECK_SUM0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SLCHOST_CHECK_SUM0` reader - *******Description***********"]
pub type SLCHOST_CHECK_SUM0_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - *******Description***********"]
    #[inline(always)]
    pub fn slchost_check_sum0(&self) -> SLCHOST_CHECK_SUM0_R {
        SLCHOST_CHECK_SUM0_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CHECK_SUM0")
            .field(
                "slchost_check_sum0",
                &format_args!("{}", self.slchost_check_sum0().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CHECK_SUM0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "*******Description***********\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [check_sum0](index.html) module"]
pub struct CHECK_SUM0_SPEC;
impl crate::RegisterSpec for CHECK_SUM0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [check_sum0::R](R) reader structure"]
impl crate::Readable for CHECK_SUM0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CHECK_SUM0 to value 0"]
impl crate::Resettable for CHECK_SUM0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
