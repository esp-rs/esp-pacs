///Register `CORE_0_DRAM0_EXCEPTION_MONITOR_1` reader
pub type R = crate::R<CORE_0_DRAM0_EXCEPTION_MONITOR_1_SPEC>;
///Field `CORE_0_DRAM0_RECORDING_PC_0` reader - reg_core_0_dram0_recording_pc_0
pub type CORE_0_DRAM0_RECORDING_PC_0_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - reg_core_0_dram0_recording_pc_0
    #[inline(always)]
    pub fn core_0_dram0_recording_pc_0(&self) -> CORE_0_DRAM0_RECORDING_PC_0_R {
        CORE_0_DRAM0_RECORDING_PC_0_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE_0_DRAM0_EXCEPTION_MONITOR_1")
            .field(
                "core_0_dram0_recording_pc_0",
                &self.core_0_dram0_recording_pc_0(),
            )
            .finish()
    }
}
/**ASSIST_DEBUG_CORE_0_DRAM0_EXCEPTION_MONITOR_1_REG

You can [`read`](crate::generic::Reg::read) this register and get [`core_0_dram0_exception_monitor_1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CORE_0_DRAM0_EXCEPTION_MONITOR_1_SPEC;
impl crate::RegisterSpec for CORE_0_DRAM0_EXCEPTION_MONITOR_1_SPEC {
    type Ux = u32;
}
///`read()` method returns [`core_0_dram0_exception_monitor_1::R`](R) reader structure
impl crate::Readable for CORE_0_DRAM0_EXCEPTION_MONITOR_1_SPEC {}
///`reset()` method sets CORE_0_DRAM0_EXCEPTION_MONITOR_1 to value 0
impl crate::Resettable for CORE_0_DRAM0_EXCEPTION_MONITOR_1_SPEC {
    const RESET_VALUE: u32 = 0;
}
