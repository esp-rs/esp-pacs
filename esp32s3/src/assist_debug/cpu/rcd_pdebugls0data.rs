#[doc = "Register `RCD_PDEBUGLS0DATA` reader"]
pub type R = crate::R<RCD_PDEBUGLS0DATA_SPEC>;
#[doc = "Field `RCD_PDEBUGLS0DATA` reader - core0_pdebug_s0data"]
pub type RCD_PDEBUGLS0DATA_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - core0_pdebug_s0data"]
    #[inline(always)]
    pub fn rcd_pdebugls0data(&self) -> RCD_PDEBUGLS0DATA_R {
        RCD_PDEBUGLS0DATA_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCD_PDEBUGLS0DATA")
            .field("rcd_pdebugls0data", &self.rcd_pdebugls0data())
            .finish()
    }
}
#[doc = "core0 pdebug status register\n\nYou can [`read`](crate::Reg::read) this register and get [`rcd_pdebugls0data::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RCD_PDEBUGLS0DATA_SPEC;
impl crate::RegisterSpec for RCD_PDEBUGLS0DATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcd_pdebugls0data::R`](R) reader structure"]
impl crate::Readable for RCD_PDEBUGLS0DATA_SPEC {}
#[doc = "`reset()` method sets RCD_PDEBUGLS0DATA to value 0"]
impl crate::Resettable for RCD_PDEBUGLS0DATA_SPEC {}
