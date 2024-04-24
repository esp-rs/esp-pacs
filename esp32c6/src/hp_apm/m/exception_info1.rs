#[doc = "Register `EXCEPTION_INFO1` reader"]
pub type R = crate::R<EXCEPTION_INFO1_SPEC>;
#[doc = "Field `EXCEPTION_ADDR` reader - Exception addr"]
pub type EXCEPTION_ADDR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Exception addr"]
    #[inline(always)]
    pub fn exception_addr(&self) -> EXCEPTION_ADDR_R {
        EXCEPTION_ADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EXCEPTION_INFO1")
            .field(
                "exception_addr",
                &format_args!("{}", self.exception_addr().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<EXCEPTION_INFO1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "M0 exception_info1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exception_info1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EXCEPTION_INFO1_SPEC;
impl crate::RegisterSpec for EXCEPTION_INFO1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`exception_info1::R`](R) reader structure"]
impl crate::Readable for EXCEPTION_INFO1_SPEC {}
#[doc = "`reset()` method sets EXCEPTION_INFO1 to value 0"]
impl crate::Resettable for EXCEPTION_INFO1_SPEC {
    const RESET_VALUE: u32 = 0;
}
