///Register `EXCEPTION_INFO1` reader
pub type R = crate::R<EXCEPTION_INFO1_SPEC>;
///Field `EXCEPTION_ADDR` reader - Exception addr
pub type EXCEPTION_ADDR_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Exception addr
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
/**M0 exception_info1 register

You can [`read`](crate::generic::Reg::read) this register and get [`exception_info1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct EXCEPTION_INFO1_SPEC;
impl crate::RegisterSpec for EXCEPTION_INFO1_SPEC {
    type Ux = u32;
}
///`read()` method returns [`exception_info1::R`](R) reader structure
impl crate::Readable for EXCEPTION_INFO1_SPEC {}
///`reset()` method sets EXCEPTION_INFO1 to value 0
impl crate::Resettable for EXCEPTION_INFO1_SPEC {
    const RESET_VALUE: u32 = 0;
}
