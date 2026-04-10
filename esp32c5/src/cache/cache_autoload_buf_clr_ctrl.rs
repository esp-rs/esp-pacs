#[doc = "Register `CACHE_AUTOLOAD_BUF_CLR_CTRL` reader"]
pub type R = crate::R<CACHE_AUTOLOAD_BUF_CLR_CTRL_SPEC>;
#[doc = "Register `CACHE_AUTOLOAD_BUF_CLR_CTRL` writer"]
pub type W = crate::W<CACHE_AUTOLOAD_BUF_CLR_CTRL_SPEC>;
#[doc = "Field `ICACHE2_ALD_BUF_CLR` reader - Reserved"]
pub type ICACHE2_ALD_BUF_CLR_R = crate::BitReader;
#[doc = "Field `ICACHE2_ALD_BUF_CLR` writer - Reserved"]
pub type ICACHE2_ALD_BUF_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CACHE_ALD_BUF_CLR` reader - set this bit to clear autoload-buffer inside Cache. If this bit is active, autoload will not work in Cache. This bit should not be active when autoload works in Cache."]
pub type CACHE_ALD_BUF_CLR_R = crate::BitReader;
#[doc = "Field `CACHE_ALD_BUF_CLR` writer - set this bit to clear autoload-buffer inside Cache. If this bit is active, autoload will not work in Cache. This bit should not be active when autoload works in Cache."]
pub type CACHE_ALD_BUF_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 2 - Reserved"]
    #[inline(always)]
    pub fn icache2_ald_buf_clr(&self) -> ICACHE2_ALD_BUF_CLR_R {
        ICACHE2_ALD_BUF_CLR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - set this bit to clear autoload-buffer inside Cache. If this bit is active, autoload will not work in Cache. This bit should not be active when autoload works in Cache."]
    #[inline(always)]
    pub fn cache_ald_buf_clr(&self) -> CACHE_ALD_BUF_CLR_R {
        CACHE_ALD_BUF_CLR_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CACHE_AUTOLOAD_BUF_CLR_CTRL")
            .field("icache2_ald_buf_clr", &self.icache2_ald_buf_clr())
            .field("cache_ald_buf_clr", &self.cache_ald_buf_clr())
            .finish()
    }
}
impl W {
    #[doc = "Bit 2 - Reserved"]
    #[inline(always)]
    pub fn icache2_ald_buf_clr(
        &mut self,
    ) -> ICACHE2_ALD_BUF_CLR_W<'_, CACHE_AUTOLOAD_BUF_CLR_CTRL_SPEC> {
        ICACHE2_ALD_BUF_CLR_W::new(self, 2)
    }
    #[doc = "Bit 4 - set this bit to clear autoload-buffer inside Cache. If this bit is active, autoload will not work in Cache. This bit should not be active when autoload works in Cache."]
    #[inline(always)]
    pub fn cache_ald_buf_clr(
        &mut self,
    ) -> CACHE_ALD_BUF_CLR_W<'_, CACHE_AUTOLOAD_BUF_CLR_CTRL_SPEC> {
        CACHE_ALD_BUF_CLR_W::new(self, 4)
    }
}
#[doc = "Cache Autoload buffer clear control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_autoload_buf_clr_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_autoload_buf_clr_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CACHE_AUTOLOAD_BUF_CLR_CTRL_SPEC;
impl crate::RegisterSpec for CACHE_AUTOLOAD_BUF_CLR_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cache_autoload_buf_clr_ctrl::R`](R) reader structure"]
impl crate::Readable for CACHE_AUTOLOAD_BUF_CLR_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cache_autoload_buf_clr_ctrl::W`](W) writer structure"]
impl crate::Writable for CACHE_AUTOLOAD_BUF_CLR_CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CACHE_AUTOLOAD_BUF_CLR_CTRL to value 0"]
impl crate::Resettable for CACHE_AUTOLOAD_BUF_CLR_CTRL_SPEC {}
