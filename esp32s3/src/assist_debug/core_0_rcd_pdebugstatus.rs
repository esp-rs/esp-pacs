#[doc = "Register `CORE_0_RCD_PDEBUGSTATUS` reader"]
pub type R = crate::R<CORE_0_RCD_PDEBUGSTATUS_SPEC>;
#[doc = "Field `CORE_0_RCD_PDEBUGSTATUS` reader - core0 pdebugstatus"]
pub type CORE_0_RCD_PDEBUGSTATUS_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - core0 pdebugstatus"]
    #[inline(always)]
    pub fn core_0_rcd_pdebugstatus(&self) -> CORE_0_RCD_PDEBUGSTATUS_R {
        CORE_0_RCD_PDEBUGSTATUS_R::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE_0_RCD_PDEBUGSTATUS")
            .field(
                "core_0_rcd_pdebugstatus",
                &format_args!("{}", self.core_0_rcd_pdebugstatus().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CORE_0_RCD_PDEBUGSTATUS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "core0 pdebug status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_0_rcd_pdebugstatus::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CORE_0_RCD_PDEBUGSTATUS_SPEC;
impl crate::RegisterSpec for CORE_0_RCD_PDEBUGSTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core_0_rcd_pdebugstatus::R`](R) reader structure"]
impl crate::Readable for CORE_0_RCD_PDEBUGSTATUS_SPEC {}
#[doc = "`reset()` method sets CORE_0_RCD_PDEBUGSTATUS to value 0"]
impl crate::Resettable for CORE_0_RCD_PDEBUGSTATUS_SPEC {
    const RESET_VALUE: u32 = 0;
}
