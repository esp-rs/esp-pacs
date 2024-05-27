///Register `CORE_1_DRAM0_EXCEPTION_MONITOR_3` reader
pub type R = crate::R<CORE_1_DRAM0_EXCEPTION_MONITOR_3_SPEC>;
///Field `CORE_1_DRAM0_RECORDING_WR_1` reader - reg_core_1_dram0_recording_wr_1
pub type CORE_1_DRAM0_RECORDING_WR_1_R = crate::BitReader;
///Field `CORE_1_DRAM0_RECORDING_BYTEEN_1` reader - reg_core_1_dram0_recording_byteen_1
pub type CORE_1_DRAM0_RECORDING_BYTEEN_1_R = crate::FieldReader<u16>;
impl R {
    ///Bit 0 - reg_core_1_dram0_recording_wr_1
    #[inline(always)]
    pub fn core_1_dram0_recording_wr_1(&self) -> CORE_1_DRAM0_RECORDING_WR_1_R {
        CORE_1_DRAM0_RECORDING_WR_1_R::new((self.bits & 1) != 0)
    }
    ///Bits 1:16 - reg_core_1_dram0_recording_byteen_1
    #[inline(always)]
    pub fn core_1_dram0_recording_byteen_1(&self) -> CORE_1_DRAM0_RECORDING_BYTEEN_1_R {
        CORE_1_DRAM0_RECORDING_BYTEEN_1_R::new(((self.bits >> 1) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE_1_DRAM0_EXCEPTION_MONITOR_3")
            .field(
                "core_1_dram0_recording_wr_1",
                &self.core_1_dram0_recording_wr_1(),
            )
            .field(
                "core_1_dram0_recording_byteen_1",
                &self.core_1_dram0_recording_byteen_1(),
            )
            .finish()
    }
}
/**exception monitor status register5

You can [`read`](crate::generic::Reg::read) this register and get [`core_1_dram0_exception_monitor_3::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CORE_1_DRAM0_EXCEPTION_MONITOR_3_SPEC;
impl crate::RegisterSpec for CORE_1_DRAM0_EXCEPTION_MONITOR_3_SPEC {
    type Ux = u32;
}
///`read()` method returns [`core_1_dram0_exception_monitor_3::R`](R) reader structure
impl crate::Readable for CORE_1_DRAM0_EXCEPTION_MONITOR_3_SPEC {}
///`reset()` method sets CORE_1_DRAM0_EXCEPTION_MONITOR_3 to value 0
impl crate::Resettable for CORE_1_DRAM0_EXCEPTION_MONITOR_3_SPEC {
    const RESET_VALUE: u32 = 0;
}
