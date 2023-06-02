#[doc = "Register `CORE_0_IRAM0_EXCEPTION_MONITOR_0` reader"]
pub struct R(crate::R<CORE_0_IRAM0_EXCEPTION_MONITOR_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CORE_0_IRAM0_EXCEPTION_MONITOR_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CORE_0_IRAM0_EXCEPTION_MONITOR_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CORE_0_IRAM0_EXCEPTION_MONITOR_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CORE_0_IRAM0_RECORDING_ADDR_0` reader - reg_core_0_iram0_recording_addr_0"]
pub type CORE_0_IRAM0_RECORDING_ADDR_0_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CORE_0_IRAM0_RECORDING_WR_0` reader - reg_core_0_iram0_recording_wr_0"]
pub type CORE_0_IRAM0_RECORDING_WR_0_R = crate::BitReader;
#[doc = "Field `CORE_0_IRAM0_RECORDING_LOADSTORE_0` reader - reg_core_0_iram0_recording_loadstore_0"]
pub type CORE_0_IRAM0_RECORDING_LOADSTORE_0_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:23 - reg_core_0_iram0_recording_addr_0"]
    #[inline(always)]
    pub fn core_0_iram0_recording_addr_0(&self) -> CORE_0_IRAM0_RECORDING_ADDR_0_R {
        CORE_0_IRAM0_RECORDING_ADDR_0_R::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bit 24 - reg_core_0_iram0_recording_wr_0"]
    #[inline(always)]
    pub fn core_0_iram0_recording_wr_0(&self) -> CORE_0_IRAM0_RECORDING_WR_0_R {
        CORE_0_IRAM0_RECORDING_WR_0_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - reg_core_0_iram0_recording_loadstore_0"]
    #[inline(always)]
    pub fn core_0_iram0_recording_loadstore_0(&self) -> CORE_0_IRAM0_RECORDING_LOADSTORE_0_R {
        CORE_0_IRAM0_RECORDING_LOADSTORE_0_R::new(((self.bits >> 25) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE_0_IRAM0_EXCEPTION_MONITOR_0")
            .field(
                "core_0_iram0_recording_addr_0",
                &format_args!("{}", self.core_0_iram0_recording_addr_0().bits()),
            )
            .field(
                "core_0_iram0_recording_wr_0",
                &format_args!("{}", self.core_0_iram0_recording_wr_0().bit()),
            )
            .field(
                "core_0_iram0_recording_loadstore_0",
                &format_args!("{}", self.core_0_iram0_recording_loadstore_0().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CORE_0_IRAM0_EXCEPTION_MONITOR_0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "exception monitor status register0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [core_0_iram0_exception_monitor_0](index.html) module"]
pub struct CORE_0_IRAM0_EXCEPTION_MONITOR_0_SPEC;
impl crate::RegisterSpec for CORE_0_IRAM0_EXCEPTION_MONITOR_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [core_0_iram0_exception_monitor_0::R](R) reader structure"]
impl crate::Readable for CORE_0_IRAM0_EXCEPTION_MONITOR_0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CORE_0_IRAM0_EXCEPTION_MONITOR_0 to value 0"]
impl crate::Resettable for CORE_0_IRAM0_EXCEPTION_MONITOR_0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
