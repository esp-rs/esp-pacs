///Register `CORE_1_IRAM0_EXCEPTION_MONITOR_0` reader
pub type R = crate::R<CORE_1_IRAM0_EXCEPTION_MONITOR_0_SPEC>;
///Field `CORE_1_IRAM0_RECORDING_ADDR_0` reader - The first iram0's addr\[25:2\] status when trigger IRAM busy interrupt
pub type CORE_1_IRAM0_RECORDING_ADDR_0_R = crate::FieldReader<u32>;
///Field `CORE_1_IRAM0_RECORDING_WR_0` reader - The first iram0's wr status when trigger IRAM busy interrupt
pub type CORE_1_IRAM0_RECORDING_WR_0_R = crate::BitReader;
///Field `CORE_1_IRAM0_RECORDING_LOADSTORE_0` reader - The first iram0's loadstore status when trigger IRAM busy interrupt
pub type CORE_1_IRAM0_RECORDING_LOADSTORE_0_R = crate::BitReader;
impl R {
    ///Bits 0:23 - The first iram0's addr\[25:2\] status when trigger IRAM busy interrupt
    #[inline(always)]
    pub fn core_1_iram0_recording_addr_0(&self) -> CORE_1_IRAM0_RECORDING_ADDR_0_R {
        CORE_1_IRAM0_RECORDING_ADDR_0_R::new(self.bits & 0x00ff_ffff)
    }
    ///Bit 24 - The first iram0's wr status when trigger IRAM busy interrupt
    #[inline(always)]
    pub fn core_1_iram0_recording_wr_0(&self) -> CORE_1_IRAM0_RECORDING_WR_0_R {
        CORE_1_IRAM0_RECORDING_WR_0_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - The first iram0's loadstore status when trigger IRAM busy interrupt
    #[inline(always)]
    pub fn core_1_iram0_recording_loadstore_0(&self) -> CORE_1_IRAM0_RECORDING_LOADSTORE_0_R {
        CORE_1_IRAM0_RECORDING_LOADSTORE_0_R::new(((self.bits >> 25) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE_1_IRAM0_EXCEPTION_MONITOR_0")
            .field(
                "core_1_iram0_recording_addr_0",
                &self.core_1_iram0_recording_addr_0(),
            )
            .field(
                "core_1_iram0_recording_wr_0",
                &self.core_1_iram0_recording_wr_0(),
            )
            .field(
                "core_1_iram0_recording_loadstore_0",
                &self.core_1_iram0_recording_loadstore_0(),
            )
            .finish()
    }
}
/**Core1 bus busy status regsiter

You can [`read`](crate::generic::Reg::read) this register and get [`core_1_iram0_exception_monitor_0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CORE_1_IRAM0_EXCEPTION_MONITOR_0_SPEC;
impl crate::RegisterSpec for CORE_1_IRAM0_EXCEPTION_MONITOR_0_SPEC {
    type Ux = u32;
}
///`read()` method returns [`core_1_iram0_exception_monitor_0::R`](R) reader structure
impl crate::Readable for CORE_1_IRAM0_EXCEPTION_MONITOR_0_SPEC {}
///`reset()` method sets CORE_1_IRAM0_EXCEPTION_MONITOR_0 to value 0
impl crate::Resettable for CORE_1_IRAM0_EXCEPTION_MONITOR_0_SPEC {
    const RESET_VALUE: u32 = 0;
}
