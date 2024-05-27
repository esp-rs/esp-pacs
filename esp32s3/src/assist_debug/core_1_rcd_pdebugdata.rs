///Register `CORE_1_RCD_PDEBUGDATA` reader
pub type R = crate::R<CORE_1_RCD_PDEBUGDATA_SPEC>;
///Field `CORE_1_RCD_PDEBUGDATA` reader - Core1_pdebugdata
pub type CORE_1_RCD_PDEBUGDATA_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Core1_pdebugdata
    #[inline(always)]
    pub fn core_1_rcd_pdebugdata(&self) -> CORE_1_RCD_PDEBUGDATA_R {
        CORE_1_RCD_PDEBUGDATA_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE_1_RCD_PDEBUGDATA")
            .field("core_1_rcd_pdebugdata", &self.core_1_rcd_pdebugdata())
            .finish()
    }
}
/**Core1 pdebug status register

You can [`read`](crate::generic::Reg::read) this register and get [`core_1_rcd_pdebugdata::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CORE_1_RCD_PDEBUGDATA_SPEC;
impl crate::RegisterSpec for CORE_1_RCD_PDEBUGDATA_SPEC {
    type Ux = u32;
}
///`read()` method returns [`core_1_rcd_pdebugdata::R`](R) reader structure
impl crate::Readable for CORE_1_RCD_PDEBUGDATA_SPEC {}
///`reset()` method sets CORE_1_RCD_PDEBUGDATA to value 0
impl crate::Resettable for CORE_1_RCD_PDEBUGDATA_SPEC {
    const RESET_VALUE: u32 = 0;
}
