#[doc = "Register `CACHE_ADDR` reader"]
pub type R = crate::R<CACHE_ADDR_SPEC>;
#[doc = "Register `CACHE_ADDR` writer"]
pub type W = crate::W<CACHE_ADDR_SPEC>;
#[doc = "Field `CACHE_ADDR` reader - Those bits stores the address which will decide where inside the specified tag memory object will be accessed."]
pub type CACHE_ADDR_R = crate::FieldReader<u32>;
#[doc = "Field `CACHE_ADDR` writer - Those bits stores the address which will decide where inside the specified tag memory object will be accessed."]
pub type CACHE_ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Those bits stores the address which will decide where inside the specified tag memory object will be accessed."]
    #[inline(always)]
    pub fn cache_addr(&self) -> CACHE_ADDR_R {
        CACHE_ADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CACHE_ADDR")
            .field("cache_addr", &self.cache_addr())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Those bits stores the address which will decide where inside the specified tag memory object will be accessed."]
    #[inline(always)]
    pub fn cache_addr(&mut self) -> CACHE_ADDR_W<'_, CACHE_ADDR_SPEC> {
        CACHE_ADDR_W::new(self, 0)
    }
}
#[doc = "Cache address register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CACHE_ADDR_SPEC;
impl crate::RegisterSpec for CACHE_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cache_addr::R`](R) reader structure"]
impl crate::Readable for CACHE_ADDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cache_addr::W`](W) writer structure"]
impl crate::Writable for CACHE_ADDR_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CACHE_ADDR to value 0"]
impl crate::Resettable for CACHE_ADDR_SPEC {}
