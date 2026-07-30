#[doc = "Register `L1_CACHE_PRELOAD_SIZE` reader"]
pub type R = crate::R<L1_CACHE_PRELOAD_SIZE_SPEC>;
#[doc = "Register `L1_CACHE_PRELOAD_SIZE` writer"]
pub type W = crate::W<L1_CACHE_PRELOAD_SIZE_SPEC>;
#[doc = "Field `L1_CACHE_PRELOAD_SIZE` reader - Configures the size of Section 0 for the preloading operation in the L1 cache. This field should be configured together with CACHE_L1_CACHE_PRELOAD_ADDR_REG."]
pub type L1_CACHE_PRELOAD_SIZE_R = crate::FieldReader<u32>;
#[doc = "Field `L1_CACHE_PRELOAD_SIZE` writer - Configures the size of Section 0 for the preloading operation in the L1 cache. This field should be configured together with CACHE_L1_CACHE_PRELOAD_ADDR_REG."]
pub type L1_CACHE_PRELOAD_SIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - Configures the size of Section 0 for the preloading operation in the L1 cache. This field should be configured together with CACHE_L1_CACHE_PRELOAD_ADDR_REG."]
    #[inline(always)]
    pub fn l1_cache_preload_size(&self) -> L1_CACHE_PRELOAD_SIZE_R {
        L1_CACHE_PRELOAD_SIZE_R::new(self.bits & 0x00ff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L1_CACHE_PRELOAD_SIZE")
            .field("l1_cache_preload_size", &self.l1_cache_preload_size())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:23 - Configures the size of Section 0 for the preloading operation in the L1 cache. This field should be configured together with CACHE_L1_CACHE_PRELOAD_ADDR_REG."]
    #[inline(always)]
    pub fn l1_cache_preload_size(
        &mut self,
    ) -> L1_CACHE_PRELOAD_SIZE_W<'_, L1_CACHE_PRELOAD_SIZE_SPEC> {
        L1_CACHE_PRELOAD_SIZE_W::new(self, 0)
    }
}
#[doc = "L1 cache preloading size configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`l1_cache_preload_size::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l1_cache_preload_size::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L1_CACHE_PRELOAD_SIZE_SPEC;
impl crate::RegisterSpec for L1_CACHE_PRELOAD_SIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l1_cache_preload_size::R`](R) reader structure"]
impl crate::Readable for L1_CACHE_PRELOAD_SIZE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`l1_cache_preload_size::W`](W) writer structure"]
impl crate::Writable for L1_CACHE_PRELOAD_SIZE_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets L1_CACHE_PRELOAD_SIZE to value 0"]
impl crate::Resettable for L1_CACHE_PRELOAD_SIZE_SPEC {}
