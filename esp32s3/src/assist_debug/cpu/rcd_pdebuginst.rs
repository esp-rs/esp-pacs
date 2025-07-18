#[doc = "Register `RCD_PDEBUGINST` reader"]
pub type R = crate::R<RCD_PDEBUGINST_SPEC>;
#[doc = "Field `RCD_PDEBUGINST` reader - core0 pdebuginst"]
pub type RCD_PDEBUGINST_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - core0 pdebuginst"]
    #[inline(always)]
    pub fn rcd_pdebuginst(&self) -> RCD_PDEBUGINST_R {
        RCD_PDEBUGINST_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCD_PDEBUGINST")
            .field("rcd_pdebuginst", &self.rcd_pdebuginst())
            .finish()
    }
}
#[doc = "core0 pdebug status register\n\nYou can [`read`](crate::Reg::read) this register and get [`rcd_pdebuginst::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RCD_PDEBUGINST_SPEC;
impl crate::RegisterSpec for RCD_PDEBUGINST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcd_pdebuginst::R`](R) reader structure"]
impl crate::Readable for RCD_PDEBUGINST_SPEC {}
#[doc = "`reset()` method sets RCD_PDEBUGINST to value 0"]
impl crate::Resettable for RCD_PDEBUGINST_SPEC {}
