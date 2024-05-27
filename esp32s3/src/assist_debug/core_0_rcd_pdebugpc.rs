///Register `CORE_0_RCD_PDEBUGPC` reader
pub type R = crate::R<CORE_0_RCD_PDEBUGPC_SPEC>;
///Field `CORE_0_RCD_PDEBUGPC` reader - core0_pdebugPC
pub type CORE_0_RCD_PDEBUGPC_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - core0_pdebugPC
    #[inline(always)]
    pub fn core_0_rcd_pdebugpc(&self) -> CORE_0_RCD_PDEBUGPC_R {
        CORE_0_RCD_PDEBUGPC_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE_0_RCD_PDEBUGPC")
            .field("core_0_rcd_pdebugpc", &self.core_0_rcd_pdebugpc())
            .finish()
    }
}
/**core0 pdebug status register

You can [`read`](crate::generic::Reg::read) this register and get [`core_0_rcd_pdebugpc::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CORE_0_RCD_PDEBUGPC_SPEC;
impl crate::RegisterSpec for CORE_0_RCD_PDEBUGPC_SPEC {
    type Ux = u32;
}
///`read()` method returns [`core_0_rcd_pdebugpc::R`](R) reader structure
impl crate::Readable for CORE_0_RCD_PDEBUGPC_SPEC {}
///`reset()` method sets CORE_0_RCD_PDEBUGPC to value 0
impl crate::Resettable for CORE_0_RCD_PDEBUGPC_SPEC {
    const RESET_VALUE: u32 = 0;
}
