#[doc = "Register `CORE_0_DRAM0_EXCEPTION_MONITOR_1` reader"]
pub type R = crate::R<CORE_0_DRAM0_EXCEPTION_MONITOR_1_SPEC>;
#[doc = "Field `CORE_0_DRAM0_RECORDING_ADDR_0` reader - reg_core_0_dram0_recording_addr_0"]
pub type CORE_0_DRAM0_RECORDING_ADDR_0_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:29 - reg_core_0_dram0_recording_addr_0"]
    #[inline(always)]
    pub fn core_0_dram0_recording_addr_0(&self) -> CORE_0_DRAM0_RECORDING_ADDR_0_R {
        CORE_0_DRAM0_RECORDING_ADDR_0_R::new(self.bits & 0x3fff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE_0_DRAM0_EXCEPTION_MONITOR_1")
            .field(
                "core_0_dram0_recording_addr_0",
                &self.core_0_dram0_recording_addr_0(),
            )
            .finish()
    }
}
#[doc = "exception monitor status register3\n\nYou can [`read`](crate::Reg::read) this register and get [`core_0_dram0_exception_monitor_1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CORE_0_DRAM0_EXCEPTION_MONITOR_1_SPEC;
impl crate::RegisterSpec for CORE_0_DRAM0_EXCEPTION_MONITOR_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core_0_dram0_exception_monitor_1::R`](R) reader structure"]
impl crate::Readable for CORE_0_DRAM0_EXCEPTION_MONITOR_1_SPEC {}
#[doc = "`reset()` method sets CORE_0_DRAM0_EXCEPTION_MONITOR_1 to value 0"]
impl crate::Resettable for CORE_0_DRAM0_EXCEPTION_MONITOR_1_SPEC {}
