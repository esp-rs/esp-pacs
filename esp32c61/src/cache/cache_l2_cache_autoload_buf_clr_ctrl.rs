#[doc = "Register `CACHE_L2_CACHE_AUTOLOAD_BUF_CLR_CTRL` reader"]
pub type R = crate::R<CACHE_L2_CACHE_AUTOLOAD_BUF_CLR_CTRL_SPEC>;
#[doc = "Field `CACHE_L2_CACHE_ALD_BUF_CLR` reader - set this bit to clear autoload-buffer inside L2-Cache. If this bit is active, autoload will not work in L2-Cache. This bit should not be active when autoload works in L2-Cache."]
pub type CACHE_L2_CACHE_ALD_BUF_CLR_R = crate::BitReader;
impl R {
    #[doc = "Bit 5 - set this bit to clear autoload-buffer inside L2-Cache. If this bit is active, autoload will not work in L2-Cache. This bit should not be active when autoload works in L2-Cache."]
    #[inline(always)]
    pub fn cache_l2_cache_ald_buf_clr(&self) -> CACHE_L2_CACHE_ALD_BUF_CLR_R {
        CACHE_L2_CACHE_ALD_BUF_CLR_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CACHE_L2_CACHE_AUTOLOAD_BUF_CLR_CTRL")
            .field(
                "cache_l2_cache_ald_buf_clr",
                &self.cache_l2_cache_ald_buf_clr(),
            )
            .finish()
    }
}
#[doc = "Cache Autoload buffer clear control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_l2_cache_autoload_buf_clr_ctrl::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CACHE_L2_CACHE_AUTOLOAD_BUF_CLR_CTRL_SPEC;
impl crate::RegisterSpec for CACHE_L2_CACHE_AUTOLOAD_BUF_CLR_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cache_l2_cache_autoload_buf_clr_ctrl::R`](R) reader structure"]
impl crate::Readable for CACHE_L2_CACHE_AUTOLOAD_BUF_CLR_CTRL_SPEC {}
#[doc = "`reset()` method sets CACHE_L2_CACHE_AUTOLOAD_BUF_CLR_CTRL to value 0"]
impl crate::Resettable for CACHE_L2_CACHE_AUTOLOAD_BUF_CLR_CTRL_SPEC {}
