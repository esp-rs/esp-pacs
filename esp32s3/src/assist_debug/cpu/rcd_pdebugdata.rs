#[doc = "Register `RCD_PDEBUGDATA` reader"]
pub type R = crate::R<RCD_PDEBUGDATA_SPEC>;
#[doc = "Field `RCD_PDEBUGDATA` reader - core0_pdebugdata"]
pub type RCD_PDEBUGDATA_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - core0_pdebugdata"]
    #[inline(always)]
    pub fn rcd_pdebugdata(&self) -> RCD_PDEBUGDATA_R {
        RCD_PDEBUGDATA_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCD_PDEBUGDATA")
            .field("rcd_pdebugdata", &self.rcd_pdebugdata())
            .finish()
    }
}
#[doc = "core0 pdebug status register\n\nYou can [`read`](crate::Reg::read) this register and get [`rcd_pdebugdata::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RCD_PDEBUGDATA_SPEC;
impl crate::RegisterSpec for RCD_PDEBUGDATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcd_pdebugdata::R`](R) reader structure"]
impl crate::Readable for RCD_PDEBUGDATA_SPEC {}
#[doc = "`reset()` method sets RCD_PDEBUGDATA to value 0"]
impl crate::Resettable for RCD_PDEBUGDATA_SPEC {}
