///Register `CONTINUE` reader
pub type R = crate::R<CONTINUE_SPEC>;
///Field `CONTINUE` reader - Reserved.
pub type CONTINUE_R = crate::FieldReader<u32>;
impl R {
    ///Bits 1:31 - Reserved.
    #[inline(always)]
    pub fn continue_(&self) -> CONTINUE_R {
        CONTINUE_R::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONTINUE")
            .field("continue_", &self.continue_())
            .finish()
    }
}
/**Typical SHA configuration register 1.

You can [`read`](crate::generic::Reg::read) this register and get [`continue_::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CONTINUE_SPEC;
impl crate::RegisterSpec for CONTINUE_SPEC {
    type Ux = u32;
}
///`read()` method returns [`continue_::R`](R) reader structure
impl crate::Readable for CONTINUE_SPEC {}
///`reset()` method sets CONTINUE to value 0
impl crate::Resettable for CONTINUE_SPEC {
    const RESET_VALUE: u32 = 0;
}
