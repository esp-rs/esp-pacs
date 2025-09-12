#[doc = "Register `L2_CACHE_SYNC_RST_CTRL` reader"]
pub type R = crate::R<L2_CACHE_SYNC_RST_CTRL_SPEC>;
#[doc = "Register `L2_CACHE_SYNC_RST_CTRL` writer"]
pub type W = crate::W<L2_CACHE_SYNC_RST_CTRL_SPEC>;
#[doc = "Field `L2_CACHE_SYNC_RST` reader - set this bit to reset sync-logic inside L2-Cache. Recommend that this should only be used to initialize sync-logic when some fatal error of sync-logic occurs."]
pub type L2_CACHE_SYNC_RST_R = crate::BitReader;
#[doc = "Field `L2_CACHE_SYNC_RST` writer - set this bit to reset sync-logic inside L2-Cache. Recommend that this should only be used to initialize sync-logic when some fatal error of sync-logic occurs."]
pub type L2_CACHE_SYNC_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 5 - set this bit to reset sync-logic inside L2-Cache. Recommend that this should only be used to initialize sync-logic when some fatal error of sync-logic occurs."]
    #[inline(always)]
    pub fn l2_cache_sync_rst(&self) -> L2_CACHE_SYNC_RST_R {
        L2_CACHE_SYNC_RST_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L2_CACHE_SYNC_RST_CTRL")
            .field("l2_cache_sync_rst", &self.l2_cache_sync_rst())
            .finish()
    }
}
impl W {
    #[doc = "Bit 5 - set this bit to reset sync-logic inside L2-Cache. Recommend that this should only be used to initialize sync-logic when some fatal error of sync-logic occurs."]
    #[inline(always)]
    pub fn l2_cache_sync_rst(&mut self) -> L2_CACHE_SYNC_RST_W<'_, L2_CACHE_SYNC_RST_CTRL_SPEC> {
        L2_CACHE_SYNC_RST_W::new(self, 5)
    }
}
#[doc = "Cache Sync Reset control register\n\nYou can [`read`](crate::Reg::read) this register and get [`l2_cache_sync_rst_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l2_cache_sync_rst_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L2_CACHE_SYNC_RST_CTRL_SPEC;
impl crate::RegisterSpec for L2_CACHE_SYNC_RST_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l2_cache_sync_rst_ctrl::R`](R) reader structure"]
impl crate::Readable for L2_CACHE_SYNC_RST_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`l2_cache_sync_rst_ctrl::W`](W) writer structure"]
impl crate::Writable for L2_CACHE_SYNC_RST_CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets L2_CACHE_SYNC_RST_CTRL to value 0"]
impl crate::Resettable for L2_CACHE_SYNC_RST_CTRL_SPEC {}
