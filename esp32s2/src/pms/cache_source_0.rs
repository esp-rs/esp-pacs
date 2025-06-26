#[doc = "Register `CACHE_SOURCE_0` reader"]
pub type R = crate::R<CACHE_SOURCE_0_SPEC>;
#[doc = "Register `CACHE_SOURCE_0` writer"]
pub type W = crate::W<CACHE_SOURCE_0_SPEC>;
#[doc = "Field `CACHE_SOURCE_LOCK` reader - Lock register. Setting to 1 locks cache access permission control registers."]
pub type CACHE_SOURCE_LOCK_R = crate::BitReader;
#[doc = "Field `CACHE_SOURCE_LOCK` writer - Lock register. Setting to 1 locks cache access permission control registers."]
pub type CACHE_SOURCE_LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Lock register. Setting to 1 locks cache access permission control registers."]
    #[inline(always)]
    pub fn cache_source_lock(&self) -> CACHE_SOURCE_LOCK_R {
        CACHE_SOURCE_LOCK_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CACHE_SOURCE_0")
            .field("cache_source_lock", &self.cache_source_lock())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Lock register. Setting to 1 locks cache access permission control registers."]
    #[inline(always)]
    pub fn cache_source_lock(&mut self) -> CACHE_SOURCE_LOCK_W<CACHE_SOURCE_0_SPEC> {
        CACHE_SOURCE_LOCK_W::new(self, 0)
    }
}
#[doc = "Cache access permission control register 0.\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_source_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_source_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CACHE_SOURCE_0_SPEC;
impl crate::RegisterSpec for CACHE_SOURCE_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cache_source_0::R`](R) reader structure"]
impl crate::Readable for CACHE_SOURCE_0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cache_source_0::W`](W) writer structure"]
impl crate::Writable for CACHE_SOURCE_0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CACHE_SOURCE_0 to value 0"]
impl crate::Resettable for CACHE_SOURCE_0_SPEC {}
