#[doc = "Register `CORE_0_DRAM0_EXCEPTION_MONITOR_5` reader"]
pub type R = crate::R<CORE_0_DRAM0_EXCEPTION_MONITOR_5_SPEC>;
#[doc = "Field `CORE_0_DRAM0_RECORDING_PC_1` reader - The second dram0's PC status when trigger DRAM busy interrupt"]
pub type CORE_0_DRAM0_RECORDING_PC_1_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - The second dram0's PC status when trigger DRAM busy interrupt"]
    #[inline(always)]
    pub fn core_0_dram0_recording_pc_1(&self) -> CORE_0_DRAM0_RECORDING_PC_1_R {
        CORE_0_DRAM0_RECORDING_PC_1_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE_0_DRAM0_EXCEPTION_MONITOR_5")
            .field(
                "core_0_dram0_recording_pc_1",
                &format_args!("{}", self.core_0_dram0_recording_pc_1().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CORE_0_DRAM0_EXCEPTION_MONITOR_5_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "core0 bus busy configuration regsiter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_0_dram0_exception_monitor_5::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CORE_0_DRAM0_EXCEPTION_MONITOR_5_SPEC;
impl crate::RegisterSpec for CORE_0_DRAM0_EXCEPTION_MONITOR_5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core_0_dram0_exception_monitor_5::R`](R) reader structure"]
impl crate::Readable for CORE_0_DRAM0_EXCEPTION_MONITOR_5_SPEC {}
#[doc = "`reset()` method sets CORE_0_DRAM0_EXCEPTION_MONITOR_5 to value 0xffff_ffff"]
impl crate::Resettable for CORE_0_DRAM0_EXCEPTION_MONITOR_5_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
