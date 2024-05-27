#[doc = "Register `CORE_0_DRAM0_EXCEPTION_MONITOR_4` reader"]
pub type R = crate::R<CORE_0_DRAM0_EXCEPTION_MONITOR_4_SPEC>;
#[doc = "Field `CORE_0_DRAM0_RECORDING_ADDR_1` reader - reg_core_0_dram0_recording_addr_1"]
pub type CORE_0_DRAM0_RECORDING_ADDR_1_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:23 - reg_core_0_dram0_recording_addr_1"]
    #[inline(always)]
    pub fn core_0_dram0_recording_addr_1(&self) -> CORE_0_DRAM0_RECORDING_ADDR_1_R {
        CORE_0_DRAM0_RECORDING_ADDR_1_R::new(self.bits & 0x00ff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE_0_DRAM0_EXCEPTION_MONITOR_4")
            .field(
                "core_0_dram0_recording_addr_1",
                &self.core_0_dram0_recording_addr_1(),
            )
            .finish()
    }
}
#[doc = "exception monitor status register6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_0_dram0_exception_monitor_4::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CORE_0_DRAM0_EXCEPTION_MONITOR_4_SPEC;
impl crate::RegisterSpec for CORE_0_DRAM0_EXCEPTION_MONITOR_4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core_0_dram0_exception_monitor_4::R`](R) reader structure"]
impl crate::Readable for CORE_0_DRAM0_EXCEPTION_MONITOR_4_SPEC {}
#[doc = "`reset()` method sets CORE_0_DRAM0_EXCEPTION_MONITOR_4 to value 0"]
impl crate::Resettable for CORE_0_DRAM0_EXCEPTION_MONITOR_4_SPEC {
    const RESET_VALUE: u32 = 0;
}
