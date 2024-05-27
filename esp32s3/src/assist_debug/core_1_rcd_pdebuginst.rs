///Register `CORE_1_RCD_PDEBUGINST` reader
pub type R = crate::R<CORE_1_RCD_PDEBUGINST_SPEC>;
///Field `CORE_1_RCD_PDEBUGINST` reader - Core1 pdebuginst
pub type CORE_1_RCD_PDEBUGINST_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Core1 pdebuginst
    #[inline(always)]
    pub fn core_1_rcd_pdebuginst(&self) -> CORE_1_RCD_PDEBUGINST_R {
        CORE_1_RCD_PDEBUGINST_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE_1_RCD_PDEBUGINST")
            .field("core_1_rcd_pdebuginst", &self.core_1_rcd_pdebuginst())
            .finish()
    }
}
/**Core1 pdebug status register

You can [`read`](crate::generic::Reg::read) this register and get [`core_1_rcd_pdebuginst::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CORE_1_RCD_PDEBUGINST_SPEC;
impl crate::RegisterSpec for CORE_1_RCD_PDEBUGINST_SPEC {
    type Ux = u32;
}
///`read()` method returns [`core_1_rcd_pdebuginst::R`](R) reader structure
impl crate::Readable for CORE_1_RCD_PDEBUGINST_SPEC {}
///`reset()` method sets CORE_1_RCD_PDEBUGINST to value 0
impl crate::Resettable for CORE_1_RCD_PDEBUGINST_SPEC {
    const RESET_VALUE: u32 = 0;
}
