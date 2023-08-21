#[doc = "Register `M2_EXCEPTION_INFO1` reader"]
pub type R = crate::R<M2_EXCEPTION_INFO1_SPEC>;
#[doc = "Field `M2_EXCEPTION_ADDR` reader - Exception addr"]
pub type M2_EXCEPTION_ADDR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Exception addr"]
    #[inline(always)]
    pub fn m2_exception_addr(&self) -> M2_EXCEPTION_ADDR_R {
        M2_EXCEPTION_ADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("M2_EXCEPTION_INFO1")
            .field(
                "m2_exception_addr",
                &format_args!("{}", self.m2_exception_addr().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<M2_EXCEPTION_INFO1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "M2 exception_info1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m2_exception_info1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct M2_EXCEPTION_INFO1_SPEC;
impl crate::RegisterSpec for M2_EXCEPTION_INFO1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`m2_exception_info1::R`](R) reader structure"]
impl crate::Readable for M2_EXCEPTION_INFO1_SPEC {}
#[doc = "`reset()` method sets M2_EXCEPTION_INFO1 to value 0"]
impl crate::Resettable for M2_EXCEPTION_INFO1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
