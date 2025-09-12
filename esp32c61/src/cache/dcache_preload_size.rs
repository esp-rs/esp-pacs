#[doc = "Register `DCACHE_PRELOAD_SIZE` reader"]
pub type R = crate::R<DCACHE_PRELOAD_SIZE_SPEC>;
#[doc = "Register `DCACHE_PRELOAD_SIZE` writer"]
pub type W = crate::W<DCACHE_PRELOAD_SIZE_SPEC>;
#[doc = "Field `CACHE_PRELOAD_SIZE` reader - Those bits are used to configure the size of the first section of prelock on L1-Cache, which should be used together with L1_CACHE_PRELOAD_ADDR_REG"]
pub type CACHE_PRELOAD_SIZE_R = crate::FieldReader<u16>;
#[doc = "Field `CACHE_PRELOAD_SIZE` writer - Those bits are used to configure the size of the first section of prelock on L1-Cache, which should be used together with L1_CACHE_PRELOAD_ADDR_REG"]
pub type CACHE_PRELOAD_SIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:13 - Those bits are used to configure the size of the first section of prelock on L1-Cache, which should be used together with L1_CACHE_PRELOAD_ADDR_REG"]
    #[inline(always)]
    pub fn cache_preload_size(&self) -> CACHE_PRELOAD_SIZE_R {
        CACHE_PRELOAD_SIZE_R::new((self.bits & 0x3fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DCACHE_PRELOAD_SIZE")
            .field("cache_preload_size", &self.cache_preload_size())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:13 - Those bits are used to configure the size of the first section of prelock on L1-Cache, which should be used together with L1_CACHE_PRELOAD_ADDR_REG"]
    #[inline(always)]
    pub fn cache_preload_size(&mut self) -> CACHE_PRELOAD_SIZE_W<'_, DCACHE_PRELOAD_SIZE_SPEC> {
        CACHE_PRELOAD_SIZE_W::new(self, 0)
    }
}
#[doc = "L1 Cache preload size configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`dcache_preload_size::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcache_preload_size::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DCACHE_PRELOAD_SIZE_SPEC;
impl crate::RegisterSpec for DCACHE_PRELOAD_SIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcache_preload_size::R`](R) reader structure"]
impl crate::Readable for DCACHE_PRELOAD_SIZE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dcache_preload_size::W`](W) writer structure"]
impl crate::Writable for DCACHE_PRELOAD_SIZE_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DCACHE_PRELOAD_SIZE to value 0"]
impl crate::Resettable for DCACHE_PRELOAD_SIZE_SPEC {}
