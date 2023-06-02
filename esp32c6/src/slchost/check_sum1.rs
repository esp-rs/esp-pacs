#[doc = "Register `CHECK_SUM1` reader"]
pub struct R(crate::R<CHECK_SUM1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHECK_SUM1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHECK_SUM1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHECK_SUM1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SLCHOST_CHECK_SUM1` reader - *******Description***********"]
pub type SLCHOST_CHECK_SUM1_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - *******Description***********"]
    #[inline(always)]
    pub fn slchost_check_sum1(&self) -> SLCHOST_CHECK_SUM1_R {
        SLCHOST_CHECK_SUM1_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CHECK_SUM1")
            .field(
                "slchost_check_sum1",
                &format_args!("{}", self.slchost_check_sum1().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CHECK_SUM1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "*******Description***********\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [check_sum1](index.html) module"]
pub struct CHECK_SUM1_SPEC;
impl crate::RegisterSpec for CHECK_SUM1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [check_sum1::R](R) reader structure"]
impl crate::Readable for CHECK_SUM1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CHECK_SUM1 to value 0x013f"]
impl crate::Resettable for CHECK_SUM1_SPEC {
    const RESET_VALUE: Self::Ux = 0x013f;
}
