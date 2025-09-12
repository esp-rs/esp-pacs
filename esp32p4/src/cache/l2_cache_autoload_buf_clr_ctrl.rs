#[doc = "Register `L2_CACHE_AUTOLOAD_BUF_CLR_CTRL` reader"]
pub type R = crate::R<L2_CACHE_AUTOLOAD_BUF_CLR_CTRL_SPEC>;
#[doc = "Register `L2_CACHE_AUTOLOAD_BUF_CLR_CTRL` writer"]
pub type W = crate::W<L2_CACHE_AUTOLOAD_BUF_CLR_CTRL_SPEC>;
#[doc = "Field `L2_CACHE_ALD_BUF_CLR` reader - set this bit to clear autoload-buffer inside L2-Cache. If this bit is active, autoload will not work in L2-Cache. This bit should not be active when autoload works in L2-Cache."]
pub type L2_CACHE_ALD_BUF_CLR_R = crate::BitReader;
#[doc = "Field `L2_CACHE_ALD_BUF_CLR` writer - set this bit to clear autoload-buffer inside L2-Cache. If this bit is active, autoload will not work in L2-Cache. This bit should not be active when autoload works in L2-Cache."]
pub type L2_CACHE_ALD_BUF_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 5 - set this bit to clear autoload-buffer inside L2-Cache. If this bit is active, autoload will not work in L2-Cache. This bit should not be active when autoload works in L2-Cache."]
    #[inline(always)]
    pub fn l2_cache_ald_buf_clr(&self) -> L2_CACHE_ALD_BUF_CLR_R {
        L2_CACHE_ALD_BUF_CLR_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L2_CACHE_AUTOLOAD_BUF_CLR_CTRL")
            .field("l2_cache_ald_buf_clr", &self.l2_cache_ald_buf_clr())
            .finish()
    }
}
impl W {
    #[doc = "Bit 5 - set this bit to clear autoload-buffer inside L2-Cache. If this bit is active, autoload will not work in L2-Cache. This bit should not be active when autoload works in L2-Cache."]
    #[inline(always)]
    pub fn l2_cache_ald_buf_clr(
        &mut self,
    ) -> L2_CACHE_ALD_BUF_CLR_W<'_, L2_CACHE_AUTOLOAD_BUF_CLR_CTRL_SPEC> {
        L2_CACHE_ALD_BUF_CLR_W::new(self, 5)
    }
}
#[doc = "Cache Autoload buffer clear control register\n\nYou can [`read`](crate::Reg::read) this register and get [`l2_cache_autoload_buf_clr_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l2_cache_autoload_buf_clr_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L2_CACHE_AUTOLOAD_BUF_CLR_CTRL_SPEC;
impl crate::RegisterSpec for L2_CACHE_AUTOLOAD_BUF_CLR_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l2_cache_autoload_buf_clr_ctrl::R`](R) reader structure"]
impl crate::Readable for L2_CACHE_AUTOLOAD_BUF_CLR_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`l2_cache_autoload_buf_clr_ctrl::W`](W) writer structure"]
impl crate::Writable for L2_CACHE_AUTOLOAD_BUF_CLR_CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets L2_CACHE_AUTOLOAD_BUF_CLR_CTRL to value 0"]
impl crate::Resettable for L2_CACHE_AUTOLOAD_BUF_CLR_CTRL_SPEC {}
