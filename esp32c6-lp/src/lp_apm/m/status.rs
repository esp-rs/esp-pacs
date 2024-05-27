///Register `STATUS` reader
pub type R = crate::R<STATUS_SPEC>;
///Field `EXCEPTION_STATUS` reader - Exception status
pub type EXCEPTION_STATUS_R = crate::FieldReader;
impl R {
    ///Bits 0:1 - Exception status
    #[inline(always)]
    pub fn exception_status(&self) -> EXCEPTION_STATUS_R {
        EXCEPTION_STATUS_R::new((self.bits & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STATUS")
            .field("exception_status", &self.exception_status())
            .finish()
    }
}
/**M0 status register

You can [`read`](crate::generic::Reg::read) this register and get [`status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
///`read()` method returns [`status::R`](R) reader structure
impl crate::Readable for STATUS_SPEC {}
///`reset()` method sets STATUS to value 0
impl crate::Resettable for STATUS_SPEC {
    const RESET_VALUE: u32 = 0;
}
