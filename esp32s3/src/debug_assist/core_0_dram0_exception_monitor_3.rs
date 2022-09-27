#[doc = "Register `CORE_0_DRAM0_EXCEPTION_MONITOR_3` reader"]
pub struct R(crate::R<CORE_0_DRAM0_EXCEPTION_MONITOR_3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CORE_0_DRAM0_EXCEPTION_MONITOR_3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CORE_0_DRAM0_EXCEPTION_MONITOR_3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CORE_0_DRAM0_EXCEPTION_MONITOR_3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CORE_0_DRAM0_RECORDING_ADDR_1` reader - The second dram0's addr\\[25:4\\] status when trigger DRAM busy interrupt"]
pub type CORE_0_DRAM0_RECORDING_ADDR_1_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CORE_0_DRAM0_RECORDING_WR_1` reader - The second dram0's wr status when trigger DRAM busy interrupt"]
pub type CORE_0_DRAM0_RECORDING_WR_1_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:21 - The second dram0's addr\\[25:4\\] status when trigger DRAM busy interrupt"]
    #[inline(always)]
    pub fn core_0_dram0_recording_addr_1(&self) -> CORE_0_DRAM0_RECORDING_ADDR_1_R {
        CORE_0_DRAM0_RECORDING_ADDR_1_R::new((self.bits & 0x003f_ffff) as u32)
    }
    #[doc = "Bit 22 - The second dram0's wr status when trigger DRAM busy interrupt"]
    #[inline(always)]
    pub fn core_0_dram0_recording_wr_1(&self) -> CORE_0_DRAM0_RECORDING_WR_1_R {
        CORE_0_DRAM0_RECORDING_WR_1_R::new(((self.bits >> 22) & 1) != 0)
    }
}
#[doc = "core0 bus busy status regsiter\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [core_0_dram0_exception_monitor_3](index.html) module"]
pub struct CORE_0_DRAM0_EXCEPTION_MONITOR_3_SPEC;
impl crate::RegisterSpec for CORE_0_DRAM0_EXCEPTION_MONITOR_3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [core_0_dram0_exception_monitor_3::R](R) reader structure"]
impl crate::Readable for CORE_0_DRAM0_EXCEPTION_MONITOR_3_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CORE_0_DRAM0_EXCEPTION_MONITOR_3 to value 0"]
impl crate::Resettable for CORE_0_DRAM0_EXCEPTION_MONITOR_3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
