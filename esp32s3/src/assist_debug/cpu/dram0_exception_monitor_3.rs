#[doc = "Register `DRAM0_EXCEPTION_MONITOR_3` reader"]
pub type R = crate::R<DRAM0_EXCEPTION_MONITOR_3_SPEC>;
#[doc = "Field `DRAM0_RECORDING_ADDR_1` reader - The second dram0's addr\\[25:4\\] status when trigger DRAM busy interrupt"]
pub type DRAM0_RECORDING_ADDR_1_R = crate::FieldReader<u32>;
#[doc = "Field `DRAM0_RECORDING_WR_1` reader - The second dram0's wr status when trigger DRAM busy interrupt"]
pub type DRAM0_RECORDING_WR_1_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:21 - The second dram0's addr\\[25:4\\] status when trigger DRAM busy interrupt"]
    #[inline(always)]
    pub fn dram0_recording_addr_1(&self) -> DRAM0_RECORDING_ADDR_1_R {
        DRAM0_RECORDING_ADDR_1_R::new(self.bits & 0x003f_ffff)
    }
    #[doc = "Bit 22 - The second dram0's wr status when trigger DRAM busy interrupt"]
    #[inline(always)]
    pub fn dram0_recording_wr_1(&self) -> DRAM0_RECORDING_WR_1_R {
        DRAM0_RECORDING_WR_1_R::new(((self.bits >> 22) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DRAM0_EXCEPTION_MONITOR_3")
            .field("dram0_recording_addr_1", &self.dram0_recording_addr_1())
            .field("dram0_recording_wr_1", &self.dram0_recording_wr_1())
            .finish()
    }
}
#[doc = "core0 bus busy status regsiter\n\nYou can [`read`](crate::Reg::read) this register and get [`dram0_exception_monitor_3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DRAM0_EXCEPTION_MONITOR_3_SPEC;
impl crate::RegisterSpec for DRAM0_EXCEPTION_MONITOR_3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dram0_exception_monitor_3::R`](R) reader structure"]
impl crate::Readable for DRAM0_EXCEPTION_MONITOR_3_SPEC {}
#[doc = "`reset()` method sets DRAM0_EXCEPTION_MONITOR_3 to value 0"]
impl crate::Resettable for DRAM0_EXCEPTION_MONITOR_3_SPEC {}
