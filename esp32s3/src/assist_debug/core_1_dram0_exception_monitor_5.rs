///Register `CORE_1_DRAM0_EXCEPTION_MONITOR_5` reader
pub type R = crate::R<CORE_1_DRAM0_EXCEPTION_MONITOR_5_SPEC>;
///Field `CORE_1_DRAM0_RECORDING_PC_1` reader - The second dram0's PC status when trigger DRAM busy interrupt
pub type CORE_1_DRAM0_RECORDING_PC_1_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - The second dram0's PC status when trigger DRAM busy interrupt
    #[inline(always)]
    pub fn core_1_dram0_recording_pc_1(&self) -> CORE_1_DRAM0_RECORDING_PC_1_R {
        CORE_1_DRAM0_RECORDING_PC_1_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE_1_DRAM0_EXCEPTION_MONITOR_5")
            .field(
                "core_1_dram0_recording_pc_1",
                &self.core_1_dram0_recording_pc_1(),
            )
            .finish()
    }
}
/**Core1 bus busy status regsiter

You can [`read`](crate::generic::Reg::read) this register and get [`core_1_dram0_exception_monitor_5::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CORE_1_DRAM0_EXCEPTION_MONITOR_5_SPEC;
impl crate::RegisterSpec for CORE_1_DRAM0_EXCEPTION_MONITOR_5_SPEC {
    type Ux = u32;
}
///`read()` method returns [`core_1_dram0_exception_monitor_5::R`](R) reader structure
impl crate::Readable for CORE_1_DRAM0_EXCEPTION_MONITOR_5_SPEC {}
///`reset()` method sets CORE_1_DRAM0_EXCEPTION_MONITOR_5 to value 0xffff_ffff
impl crate::Resettable for CORE_1_DRAM0_EXCEPTION_MONITOR_5_SPEC {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
