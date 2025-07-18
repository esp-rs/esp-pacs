#[doc = "Register `CACHE_L2_CACHE_PRELOAD_RST_CTRL` reader"]
pub type R = crate::R<CACHE_L2_CACHE_PRELOAD_RST_CTRL_SPEC>;
#[doc = "Field `CACHE_L2_CACHE_PLD_RST` reader - set this bit to reset preload-logic inside L2-Cache. Recommend that this should only be used to initialize preload-logic when some fatal error of preload-logic occurs."]
pub type CACHE_L2_CACHE_PLD_RST_R = crate::BitReader;
impl R {
    #[doc = "Bit 5 - set this bit to reset preload-logic inside L2-Cache. Recommend that this should only be used to initialize preload-logic when some fatal error of preload-logic occurs."]
    #[inline(always)]
    pub fn cache_l2_cache_pld_rst(&self) -> CACHE_L2_CACHE_PLD_RST_R {
        CACHE_L2_CACHE_PLD_RST_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CACHE_L2_CACHE_PRELOAD_RST_CTRL")
            .field("cache_l2_cache_pld_rst", &self.cache_l2_cache_pld_rst())
            .finish()
    }
}
#[doc = "Cache Preload Reset control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_l2_cache_preload_rst_ctrl::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CACHE_L2_CACHE_PRELOAD_RST_CTRL_SPEC;
impl crate::RegisterSpec for CACHE_L2_CACHE_PRELOAD_RST_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cache_l2_cache_preload_rst_ctrl::R`](R) reader structure"]
impl crate::Readable for CACHE_L2_CACHE_PRELOAD_RST_CTRL_SPEC {}
#[doc = "`reset()` method sets CACHE_L2_CACHE_PRELOAD_RST_CTRL to value 0"]
impl crate::Resettable for CACHE_L2_CACHE_PRELOAD_RST_CTRL_SPEC {}
