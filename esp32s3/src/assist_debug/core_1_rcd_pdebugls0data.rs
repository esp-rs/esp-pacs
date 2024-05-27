///Register `CORE_1_RCD_PDEBUGLS0DATA` reader
pub type R = crate::R<CORE_1_RCD_PDEBUGLS0DATA_SPEC>;
///Field `CORE_1_RCD_PDEBUGLS0DATA` reader - Core1_pdebug_s0data
pub type CORE_1_RCD_PDEBUGLS0DATA_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Core1_pdebug_s0data
    #[inline(always)]
    pub fn core_1_rcd_pdebugls0data(&self) -> CORE_1_RCD_PDEBUGLS0DATA_R {
        CORE_1_RCD_PDEBUGLS0DATA_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE_1_RCD_PDEBUGLS0DATA")
            .field("core_1_rcd_pdebugls0data", &self.core_1_rcd_pdebugls0data())
            .finish()
    }
}
/**Core1 pdebug status register

You can [`read`](crate::generic::Reg::read) this register and get [`core_1_rcd_pdebugls0data::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CORE_1_RCD_PDEBUGLS0DATA_SPEC;
impl crate::RegisterSpec for CORE_1_RCD_PDEBUGLS0DATA_SPEC {
    type Ux = u32;
}
///`read()` method returns [`core_1_rcd_pdebugls0data::R`](R) reader structure
impl crate::Readable for CORE_1_RCD_PDEBUGLS0DATA_SPEC {}
///`reset()` method sets CORE_1_RCD_PDEBUGLS0DATA to value 0
impl crate::Resettable for CORE_1_RCD_PDEBUGLS0DATA_SPEC {
    const RESET_VALUE: u32 = 0;
}
