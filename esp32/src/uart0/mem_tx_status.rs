///Register `MEM_TX_STATUS` reader
pub type R = crate::R<MEM_TX_STATUS_SPEC>;
///Field `MEM_TX_STATUS` reader -
pub type MEM_TX_STATUS_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:23
    #[inline(always)]
    pub fn mem_tx_status(&self) -> MEM_TX_STATUS_R {
        MEM_TX_STATUS_R::new(self.bits & 0x00ff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MEM_TX_STATUS")
            .field("mem_tx_status", &self.mem_tx_status())
            .finish()
    }
}
/**

You can [`read`](crate::generic::Reg::read) this register and get [`mem_tx_status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct MEM_TX_STATUS_SPEC;
impl crate::RegisterSpec for MEM_TX_STATUS_SPEC {
    type Ux = u32;
}
///`read()` method returns [`mem_tx_status::R`](R) reader structure
impl crate::Readable for MEM_TX_STATUS_SPEC {}
///`reset()` method sets MEM_TX_STATUS to value 0
impl crate::Resettable for MEM_TX_STATUS_SPEC {
    const RESET_VALUE: u32 = 0;
}
