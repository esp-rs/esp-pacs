#[doc = "Register `RCD_SP` reader"]
pub type R = crate::R<RCD_SP_SPEC>;
#[doc = "Field `CORE_0_RCD_SP` reader - core0_stack pointer"]
pub type CORE_0_RCD_SP_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - core0_stack pointer"]
    #[inline(always)]
    pub fn core_0_rcd_sp(&self) -> CORE_0_RCD_SP_R {
        CORE_0_RCD_SP_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCD_SP")
            .field("core_0_rcd_sp", &self.core_0_rcd_sp())
            .finish()
    }
}
#[doc = "core0 pdebug status register\n\nYou can [`read`](crate::Reg::read) this register and get [`rcd_sp::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RCD_SP_SPEC;
impl crate::RegisterSpec for RCD_SP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcd_sp::R`](R) reader structure"]
impl crate::Readable for RCD_SP_SPEC {}
#[doc = "`reset()` method sets RCD_SP to value 0"]
impl crate::Resettable for RCD_SP_SPEC {}
