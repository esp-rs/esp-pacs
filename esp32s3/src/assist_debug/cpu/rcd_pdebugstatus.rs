#[doc = "Register `RCD_PDEBUGSTATUS` reader"]
pub type R = crate::R<RCD_PDEBUGSTATUS_SPEC>;
#[doc = "Field `RCD_PDEBUGSTATUS` reader - core0 pdebugstatus"]
pub type RCD_PDEBUGSTATUS_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - core0 pdebugstatus"]
    #[inline(always)]
    pub fn rcd_pdebugstatus(&self) -> RCD_PDEBUGSTATUS_R {
        RCD_PDEBUGSTATUS_R::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCD_PDEBUGSTATUS")
            .field("rcd_pdebugstatus", &self.rcd_pdebugstatus())
            .finish()
    }
}
#[doc = "core0 pdebug status register\n\nYou can [`read`](crate::Reg::read) this register and get [`rcd_pdebugstatus::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RCD_PDEBUGSTATUS_SPEC;
impl crate::RegisterSpec for RCD_PDEBUGSTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcd_pdebugstatus::R`](R) reader structure"]
impl crate::Readable for RCD_PDEBUGSTATUS_SPEC {}
#[doc = "`reset()` method sets RCD_PDEBUGSTATUS to value 0"]
impl crate::Resettable for RCD_PDEBUGSTATUS_SPEC {}
