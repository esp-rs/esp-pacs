#[doc = "Register `CORE_1_DRAM0_EXCEPTION_MONITOR_4` reader"]
pub struct R(crate::R<CORE_1_DRAM0_EXCEPTION_MONITOR_4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CORE_1_DRAM0_EXCEPTION_MONITOR_4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CORE_1_DRAM0_EXCEPTION_MONITOR_4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CORE_1_DRAM0_EXCEPTION_MONITOR_4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CORE_1_DRAM0_RECORDING_BYTEEN_1` reader - The second dram0's byteen status when trigger DRAM busy interrupt"]
pub type CORE_1_DRAM0_RECORDING_BYTEEN_1_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - The second dram0's byteen status when trigger DRAM busy interrupt"]
    #[inline(always)]
    pub fn core_1_dram0_recording_byteen_1(&self) -> CORE_1_DRAM0_RECORDING_BYTEEN_1_R {
        CORE_1_DRAM0_RECORDING_BYTEEN_1_R::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE_1_DRAM0_EXCEPTION_MONITOR_4")
            .field(
                "core_1_dram0_recording_byteen_1",
                &format_args!("{}", self.core_1_dram0_recording_byteen_1().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CORE_1_DRAM0_EXCEPTION_MONITOR_4_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Core1 bus busy status regsiter\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [core_1_dram0_exception_monitor_4](index.html) module"]
pub struct CORE_1_DRAM0_EXCEPTION_MONITOR_4_SPEC;
impl crate::RegisterSpec for CORE_1_DRAM0_EXCEPTION_MONITOR_4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [core_1_dram0_exception_monitor_4::R](R) reader structure"]
impl crate::Readable for CORE_1_DRAM0_EXCEPTION_MONITOR_4_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CORE_1_DRAM0_EXCEPTION_MONITOR_4 to value 0"]
impl crate::Resettable for CORE_1_DRAM0_EXCEPTION_MONITOR_4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
