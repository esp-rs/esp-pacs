#[doc = "Register `CORE_0_DRAM0_EXCEPTION_MONITOR_0` reader"]
pub type R = crate::R<CORE_0_DRAM0_EXCEPTION_MONITOR_0_SPEC>;
#[doc = "Field `CORE_0_DRAM0_RECORDING_ADDR_0` reader - The first dram0's addr\\[25:4\\] status when trigger DRAM busy interrupt"]
pub type CORE_0_DRAM0_RECORDING_ADDR_0_R = crate::FieldReader<u32>;
#[doc = "Field `CORE_0_DRAM0_RECORDING_WR_0` reader - The first dram0's wr status when trigger DRAM busy interrupt"]
pub type CORE_0_DRAM0_RECORDING_WR_0_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:21 - The first dram0's addr\\[25:4\\] status when trigger DRAM busy interrupt"]
    #[inline(always)]
    pub fn core_0_dram0_recording_addr_0(&self) -> CORE_0_DRAM0_RECORDING_ADDR_0_R {
        CORE_0_DRAM0_RECORDING_ADDR_0_R::new(self.bits & 0x003f_ffff)
    }
    #[doc = "Bit 22 - The first dram0's wr status when trigger DRAM busy interrupt"]
    #[inline(always)]
    pub fn core_0_dram0_recording_wr_0(&self) -> CORE_0_DRAM0_RECORDING_WR_0_R {
        CORE_0_DRAM0_RECORDING_WR_0_R::new(((self.bits >> 22) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE_0_DRAM0_EXCEPTION_MONITOR_0")
            .field(
                "core_0_dram0_recording_addr_0",
                &self.core_0_dram0_recording_addr_0(),
            )
            .field(
                "core_0_dram0_recording_wr_0",
                &self.core_0_dram0_recording_wr_0(),
            )
            .finish()
    }
}
#[doc = "core0 bus busy status regsiter\n\nYou can [`read`](crate::Reg::read) this register and get [`core_0_dram0_exception_monitor_0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CORE_0_DRAM0_EXCEPTION_MONITOR_0_SPEC;
impl crate::RegisterSpec for CORE_0_DRAM0_EXCEPTION_MONITOR_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core_0_dram0_exception_monitor_0::R`](R) reader structure"]
impl crate::Readable for CORE_0_DRAM0_EXCEPTION_MONITOR_0_SPEC {}
#[doc = "`reset()` method sets CORE_0_DRAM0_EXCEPTION_MONITOR_0 to value 0"]
impl crate::Resettable for CORE_0_DRAM0_EXCEPTION_MONITOR_0_SPEC {}
