#[doc = "Register `HOST_SLCHOST_CHECK_SUM1` reader"]
pub struct R(crate::R<HOST_SLCHOST_CHECK_SUM1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HOST_SLCHOST_CHECK_SUM1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HOST_SLCHOST_CHECK_SUM1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HOST_SLCHOST_CHECK_SUM1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `HOST_SLCHOST_CHECK_SUM1` reader - "]
pub type HOST_SLCHOST_CHECK_SUM1_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn host_slchost_check_sum1(&self) -> HOST_SLCHOST_CHECK_SUM1_R {
        HOST_SLCHOST_CHECK_SUM1_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HOST_SLCHOST_CHECK_SUM1")
            .field(
                "host_slchost_check_sum1",
                &format_args!("{}", self.host_slchost_check_sum1().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HOST_SLCHOST_CHECK_SUM1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [host_slchost_check_sum1](index.html) module"]
pub struct HOST_SLCHOST_CHECK_SUM1_SPEC;
impl crate::RegisterSpec for HOST_SLCHOST_CHECK_SUM1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [host_slchost_check_sum1::R](R) reader structure"]
impl crate::Readable for HOST_SLCHOST_CHECK_SUM1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets HOST_SLCHOST_CHECK_SUM1 to value 0"]
impl crate::Resettable for HOST_SLCHOST_CHECK_SUM1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
