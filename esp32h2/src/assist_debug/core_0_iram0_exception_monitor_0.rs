///Register `CORE_0_IRAM0_EXCEPTION_MONITOR_0` reader
pub type R = crate::R<CORE_0_IRAM0_EXCEPTION_MONITOR_0_SPEC>;
///Field `CORE_0_IRAM0_RECORDING_ADDR_0` reader - reg_core_0_iram0_recording_addr_0
pub type CORE_0_IRAM0_RECORDING_ADDR_0_R = crate::FieldReader<u32>;
///Field `CORE_0_IRAM0_RECORDING_WR_0` reader - reg_core_0_iram0_recording_wr_0
pub type CORE_0_IRAM0_RECORDING_WR_0_R = crate::BitReader;
///Field `CORE_0_IRAM0_RECORDING_LOADSTORE_0` reader - reg_core_0_iram0_recording_loadstore_0
pub type CORE_0_IRAM0_RECORDING_LOADSTORE_0_R = crate::BitReader;
impl R {
    ///Bits 0:23 - reg_core_0_iram0_recording_addr_0
    #[inline(always)]
    pub fn core_0_iram0_recording_addr_0(&self) -> CORE_0_IRAM0_RECORDING_ADDR_0_R {
        CORE_0_IRAM0_RECORDING_ADDR_0_R::new(self.bits & 0x00ff_ffff)
    }
    ///Bit 24 - reg_core_0_iram0_recording_wr_0
    #[inline(always)]
    pub fn core_0_iram0_recording_wr_0(&self) -> CORE_0_IRAM0_RECORDING_WR_0_R {
        CORE_0_IRAM0_RECORDING_WR_0_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - reg_core_0_iram0_recording_loadstore_0
    #[inline(always)]
    pub fn core_0_iram0_recording_loadstore_0(&self) -> CORE_0_IRAM0_RECORDING_LOADSTORE_0_R {
        CORE_0_IRAM0_RECORDING_LOADSTORE_0_R::new(((self.bits >> 25) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE_0_IRAM0_EXCEPTION_MONITOR_0")
            .field(
                "core_0_iram0_recording_addr_0",
                &self.core_0_iram0_recording_addr_0(),
            )
            .field(
                "core_0_iram0_recording_wr_0",
                &self.core_0_iram0_recording_wr_0(),
            )
            .field(
                "core_0_iram0_recording_loadstore_0",
                &self.core_0_iram0_recording_loadstore_0(),
            )
            .finish()
    }
}
/**exception monitor status register0

You can [`read`](crate::generic::Reg::read) this register and get [`core_0_iram0_exception_monitor_0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CORE_0_IRAM0_EXCEPTION_MONITOR_0_SPEC;
impl crate::RegisterSpec for CORE_0_IRAM0_EXCEPTION_MONITOR_0_SPEC {
    type Ux = u32;
}
///`read()` method returns [`core_0_iram0_exception_monitor_0::R`](R) reader structure
impl crate::Readable for CORE_0_IRAM0_EXCEPTION_MONITOR_0_SPEC {}
///`reset()` method sets CORE_0_IRAM0_EXCEPTION_MONITOR_0 to value 0
impl crate::Resettable for CORE_0_IRAM0_EXCEPTION_MONITOR_0_SPEC {
    const RESET_VALUE: u32 = 0;
}
