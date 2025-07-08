#[doc = "Register `CORE_0_IRAM0_EXCEPTION_MONITOR_1` reader"]
pub type R = crate::R<CORE_0_IRAM0_EXCEPTION_MONITOR_1_SPEC>;
#[doc = "Field `CORE_0_IRAM0_RECORDING_ADDR_1` reader - reg_core_0_iram0_recording_addr_1"]
pub type CORE_0_IRAM0_RECORDING_ADDR_1_R = crate::FieldReader<u32>;
#[doc = "Field `CORE_0_IRAM0_RECORDING_WR_1` reader - reg_core_0_iram0_recording_wr_1"]
pub type CORE_0_IRAM0_RECORDING_WR_1_R = crate::BitReader;
#[doc = "Field `CORE_0_IRAM0_RECORDING_LOADSTORE_1` reader - reg_core_0_iram0_recording_loadstore_1"]
pub type CORE_0_IRAM0_RECORDING_LOADSTORE_1_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:29 - reg_core_0_iram0_recording_addr_1"]
    #[inline(always)]
    pub fn core_0_iram0_recording_addr_1(&self) -> CORE_0_IRAM0_RECORDING_ADDR_1_R {
        CORE_0_IRAM0_RECORDING_ADDR_1_R::new(self.bits & 0x3fff_ffff)
    }
    #[doc = "Bit 30 - reg_core_0_iram0_recording_wr_1"]
    #[inline(always)]
    pub fn core_0_iram0_recording_wr_1(&self) -> CORE_0_IRAM0_RECORDING_WR_1_R {
        CORE_0_IRAM0_RECORDING_WR_1_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - reg_core_0_iram0_recording_loadstore_1"]
    #[inline(always)]
    pub fn core_0_iram0_recording_loadstore_1(&self) -> CORE_0_IRAM0_RECORDING_LOADSTORE_1_R {
        CORE_0_IRAM0_RECORDING_LOADSTORE_1_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE_0_IRAM0_EXCEPTION_MONITOR_1")
            .field(
                "core_0_iram0_recording_addr_1",
                &self.core_0_iram0_recording_addr_1(),
            )
            .field(
                "core_0_iram0_recording_wr_1",
                &self.core_0_iram0_recording_wr_1(),
            )
            .field(
                "core_0_iram0_recording_loadstore_1",
                &self.core_0_iram0_recording_loadstore_1(),
            )
            .finish()
    }
}
#[doc = "exception monitor status register1\n\nYou can [`read`](crate::Reg::read) this register and get [`core_0_iram0_exception_monitor_1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CORE_0_IRAM0_EXCEPTION_MONITOR_1_SPEC;
impl crate::RegisterSpec for CORE_0_IRAM0_EXCEPTION_MONITOR_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core_0_iram0_exception_monitor_1::R`](R) reader structure"]
impl crate::Readable for CORE_0_IRAM0_EXCEPTION_MONITOR_1_SPEC {}
#[doc = "`reset()` method sets CORE_0_IRAM0_EXCEPTION_MONITOR_1 to value 0"]
impl crate::Resettable for CORE_0_IRAM0_EXCEPTION_MONITOR_1_SPEC {}
