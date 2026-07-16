#[doc = "Register `LP_CORE_DMACTIVE_LPCORE` reader"]
pub type R = crate::R<LP_CORE_DMACTIVE_LPCORE_SPEC>;
#[doc = "Field `LP_CORE_DMACTIVE_LPCORE` reader - "]
pub type LP_CORE_DMACTIVE_LPCORE_R = crate::BitReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn lp_core_dmactive_lpcore(&self) -> LP_CORE_DMACTIVE_LPCORE_R {
        LP_CORE_DMACTIVE_LPCORE_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LP_CORE_DMACTIVE_LPCORE")
            .field("lp_core_dmactive_lpcore", &self.lp_core_dmactive_lpcore())
            .finish()
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_core_dmactive_lpcore::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LP_CORE_DMACTIVE_LPCORE_SPEC;
impl crate::RegisterSpec for LP_CORE_DMACTIVE_LPCORE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lp_core_dmactive_lpcore::R`](R) reader structure"]
impl crate::Readable for LP_CORE_DMACTIVE_LPCORE_SPEC {}
#[doc = "`reset()` method sets LP_CORE_DMACTIVE_LPCORE to value 0"]
impl crate::Resettable for LP_CORE_DMACTIVE_LPCORE_SPEC {}
