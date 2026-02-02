#[doc = "Register `EXCEPTION_INFO1` reader"]
pub type R = crate::R<EXCEPTION_INFO1_SPEC>;
#[doc = "Field `EXCEPTION_ADDR` reader - Represents the access address when an exception occurs."]
pub type EXCEPTION_ADDR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Represents the access address when an exception occurs."]
    #[inline(always)]
    pub fn exception_addr(&self) -> EXCEPTION_ADDR_R {
        EXCEPTION_ADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EXCEPTION_INFO1")
            .field("exception_addr", &self.exception_addr())
            .finish()
    }
}
#[doc = "LP_APM_CTRL M0 exception information register\n\nYou can [`read`](crate::Reg::read) this register and get [`exception_info1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EXCEPTION_INFO1_SPEC;
impl crate::RegisterSpec for EXCEPTION_INFO1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`exception_info1::R`](R) reader structure"]
impl crate::Readable for EXCEPTION_INFO1_SPEC {}
#[doc = "`reset()` method sets EXCEPTION_INFO1 to value 0"]
impl crate::Resettable for EXCEPTION_INFO1_SPEC {}
