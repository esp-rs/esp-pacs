#[doc = "Register `CORE_1_DRAM0_EXCEPTION_MONITOR_0` reader"]
pub struct R(crate::R<CORE_1_DRAM0_EXCEPTION_MONITOR_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CORE_1_DRAM0_EXCEPTION_MONITOR_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CORE_1_DRAM0_EXCEPTION_MONITOR_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CORE_1_DRAM0_EXCEPTION_MONITOR_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CORE_1_DRAM0_RECORDING_ADDR_0` reader - The first dram0's addr\\[25:4\\] status when trigger DRAM busy interrupt"]
pub type CORE_1_DRAM0_RECORDING_ADDR_0_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CORE_1_DRAM0_RECORDING_WR_0` reader - The first dram0's wr status when trigger DRAM busy interrupt"]
pub type CORE_1_DRAM0_RECORDING_WR_0_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:21 - The first dram0's addr\\[25:4\\] status when trigger DRAM busy interrupt"]
    #[inline(always)]
    pub fn core_1_dram0_recording_addr_0(&self) -> CORE_1_DRAM0_RECORDING_ADDR_0_R {
        CORE_1_DRAM0_RECORDING_ADDR_0_R::new((self.bits & 0x003f_ffff) as u32)
    }
    #[doc = "Bit 22 - The first dram0's wr status when trigger DRAM busy interrupt"]
    #[inline(always)]
    pub fn core_1_dram0_recording_wr_0(&self) -> CORE_1_DRAM0_RECORDING_WR_0_R {
        CORE_1_DRAM0_RECORDING_WR_0_R::new(((self.bits >> 22) & 1) != 0)
    }
}
#[doc = "Core1 bus busy status regsiter\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [core_1_dram0_exception_monitor_0](index.html) module"]
pub struct CORE_1_DRAM0_EXCEPTION_MONITOR_0_SPEC;
impl crate::RegisterSpec for CORE_1_DRAM0_EXCEPTION_MONITOR_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [core_1_dram0_exception_monitor_0::R](R) reader structure"]
impl crate::Readable for CORE_1_DRAM0_EXCEPTION_MONITOR_0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CORE_1_DRAM0_EXCEPTION_MONITOR_0 to value 0"]
impl crate::Resettable for CORE_1_DRAM0_EXCEPTION_MONITOR_0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
