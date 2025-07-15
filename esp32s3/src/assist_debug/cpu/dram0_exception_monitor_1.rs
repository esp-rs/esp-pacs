#[doc = "Register `DRAM0_EXCEPTION_MONITOR_1` reader"]
pub type R = crate::R<DRAM0_EXCEPTION_MONITOR_1_SPEC>;
#[doc = "Field `DRAM0_RECORDING_BYTEEN_0` reader - The first dram0's byteen status when trigger DRAM busy interrupt"]
pub type DRAM0_RECORDING_BYTEEN_0_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - The first dram0's byteen status when trigger DRAM busy interrupt"]
    #[inline(always)]
    pub fn dram0_recording_byteen_0(&self) -> DRAM0_RECORDING_BYTEEN_0_R {
        DRAM0_RECORDING_BYTEEN_0_R::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DRAM0_EXCEPTION_MONITOR_1")
            .field("dram0_recording_byteen_0", &self.dram0_recording_byteen_0())
            .finish()
    }
}
#[doc = "core0 bus busy status regsiter\n\nYou can [`read`](crate::Reg::read) this register and get [`dram0_exception_monitor_1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DRAM0_EXCEPTION_MONITOR_1_SPEC;
impl crate::RegisterSpec for DRAM0_EXCEPTION_MONITOR_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dram0_exception_monitor_1::R`](R) reader structure"]
impl crate::Readable for DRAM0_EXCEPTION_MONITOR_1_SPEC {}
#[doc = "`reset()` method sets DRAM0_EXCEPTION_MONITOR_1 to value 0"]
impl crate::Resettable for DRAM0_EXCEPTION_MONITOR_1_SPEC {}
