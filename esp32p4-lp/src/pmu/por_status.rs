#[doc = "Register `POR_STATUS` reader"]
pub type R = crate::R<POR_STATUS_SPEC>;
#[doc = "Field `POR_DONE` reader - PMU_POR_DONE"]
pub type POR_DONE_R = crate::BitReader;
impl R {
    #[doc = "Bit 31 - PMU_POR_DONE"]
    #[inline(always)]
    pub fn por_done(&self) -> POR_DONE_R {
        POR_DONE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("POR_STATUS")
            .field("por_done", &self.por_done())
            .finish()
    }
}
#[doc = "PMU_POR_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`por_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct POR_STATUS_SPEC;
impl crate::RegisterSpec for POR_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`por_status::R`](R) reader structure"]
impl crate::Readable for POR_STATUS_SPEC {}
#[doc = "`reset()` method sets POR_STATUS to value 0"]
impl crate::Resettable for POR_STATUS_SPEC {}
