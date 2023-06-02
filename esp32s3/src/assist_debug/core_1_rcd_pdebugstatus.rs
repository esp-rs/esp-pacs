#[doc = "Register `CORE_1_RCD_PDEBUGSTATUS` reader"]
pub struct R(crate::R<CORE_1_RCD_PDEBUGSTATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CORE_1_RCD_PDEBUGSTATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CORE_1_RCD_PDEBUGSTATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CORE_1_RCD_PDEBUGSTATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CORE_1_RCD_PDEBUGSTATUS` reader - Core1 pdebugstatus"]
pub type CORE_1_RCD_PDEBUGSTATUS_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Core1 pdebugstatus"]
    #[inline(always)]
    pub fn core_1_rcd_pdebugstatus(&self) -> CORE_1_RCD_PDEBUGSTATUS_R {
        CORE_1_RCD_PDEBUGSTATUS_R::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE_1_RCD_PDEBUGSTATUS")
            .field(
                "core_1_rcd_pdebugstatus",
                &format_args!("{}", self.core_1_rcd_pdebugstatus().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CORE_1_RCD_PDEBUGSTATUS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Core1 pdebug status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [core_1_rcd_pdebugstatus](index.html) module"]
pub struct CORE_1_RCD_PDEBUGSTATUS_SPEC;
impl crate::RegisterSpec for CORE_1_RCD_PDEBUGSTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [core_1_rcd_pdebugstatus::R](R) reader structure"]
impl crate::Readable for CORE_1_RCD_PDEBUGSTATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CORE_1_RCD_PDEBUGSTATUS to value 0"]
impl crate::Resettable for CORE_1_RCD_PDEBUGSTATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
