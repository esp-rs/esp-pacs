///Register `CORE_1_IRAM0_EXCEPTION_MONITOR_1` reader
pub type R = crate::R<CORE_1_IRAM0_EXCEPTION_MONITOR_1_SPEC>;
///Field `CORE_1_IRAM0_RECORDING_ADDR_1` reader - reg_core_1_iram0_recording_addr_1
pub type CORE_1_IRAM0_RECORDING_ADDR_1_R = crate::FieldReader<u32>;
///Field `CORE_1_IRAM0_RECORDING_WR_1` reader - reg_core_1_iram0_recording_wr_1
pub type CORE_1_IRAM0_RECORDING_WR_1_R = crate::BitReader;
///Field `CORE_1_IRAM0_RECORDING_LOADSTORE_1` reader - reg_core_1_iram0_recording_loadstore_1
pub type CORE_1_IRAM0_RECORDING_LOADSTORE_1_R = crate::BitReader;
impl R {
    ///Bits 0:23 - reg_core_1_iram0_recording_addr_1
    #[inline(always)]
    pub fn core_1_iram0_recording_addr_1(&self) -> CORE_1_IRAM0_RECORDING_ADDR_1_R {
        CORE_1_IRAM0_RECORDING_ADDR_1_R::new(self.bits & 0x00ff_ffff)
    }
    ///Bit 24 - reg_core_1_iram0_recording_wr_1
    #[inline(always)]
    pub fn core_1_iram0_recording_wr_1(&self) -> CORE_1_IRAM0_RECORDING_WR_1_R {
        CORE_1_IRAM0_RECORDING_WR_1_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - reg_core_1_iram0_recording_loadstore_1
    #[inline(always)]
    pub fn core_1_iram0_recording_loadstore_1(&self) -> CORE_1_IRAM0_RECORDING_LOADSTORE_1_R {
        CORE_1_IRAM0_RECORDING_LOADSTORE_1_R::new(((self.bits >> 25) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE_1_IRAM0_EXCEPTION_MONITOR_1")
            .field(
                "core_1_iram0_recording_addr_1",
                &self.core_1_iram0_recording_addr_1(),
            )
            .field(
                "core_1_iram0_recording_wr_1",
                &self.core_1_iram0_recording_wr_1(),
            )
            .field(
                "core_1_iram0_recording_loadstore_1",
                &self.core_1_iram0_recording_loadstore_1(),
            )
            .finish()
    }
}
/**exception monitor status register1

You can [`read`](crate::generic::Reg::read) this register and get [`core_1_iram0_exception_monitor_1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CORE_1_IRAM0_EXCEPTION_MONITOR_1_SPEC;
impl crate::RegisterSpec for CORE_1_IRAM0_EXCEPTION_MONITOR_1_SPEC {
    type Ux = u32;
}
///`read()` method returns [`core_1_iram0_exception_monitor_1::R`](R) reader structure
impl crate::Readable for CORE_1_IRAM0_EXCEPTION_MONITOR_1_SPEC {}
///`reset()` method sets CORE_1_IRAM0_EXCEPTION_MONITOR_1 to value 0
impl crate::Resettable for CORE_1_IRAM0_EXCEPTION_MONITOR_1_SPEC {
    const RESET_VALUE: u32 = 0;
}
