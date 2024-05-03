#[doc = "Register `PRO_CACHE_1` reader"]
pub type R = crate::R<PRO_CACHE_1_SPEC>;
#[doc = "Register `PRO_CACHE_1` writer"]
pub type W = crate::W<PRO_CACHE_1_SPEC>;
#[doc = "Field `PRO_CACHE_CONNECT` reader - Configure which SRAM Block will be occupied by Icache or Dcache."]
pub type PRO_CACHE_CONNECT_R = crate::FieldReader<u16>;
#[doc = "Field `PRO_CACHE_CONNECT` writer - Configure which SRAM Block will be occupied by Icache or Dcache."]
pub type PRO_CACHE_CONNECT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Configure which SRAM Block will be occupied by Icache or Dcache."]
    #[inline(always)]
    pub fn pro_cache_connect(&self) -> PRO_CACHE_CONNECT_R {
        PRO_CACHE_CONNECT_R::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRO_CACHE_1")
            .field("pro_cache_connect", &self.pro_cache_connect().bits())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PRO_CACHE_1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:15 - Configure which SRAM Block will be occupied by Icache or Dcache."]
    #[inline(always)]
    #[must_use]
    pub fn pro_cache_connect(&mut self) -> PRO_CACHE_CONNECT_W<PRO_CACHE_1_SPEC> {
        PRO_CACHE_CONNECT_W::new(self, 0)
    }
}
#[doc = "Cache permission control register 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_cache_1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_cache_1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRO_CACHE_1_SPEC;
impl crate::RegisterSpec for PRO_CACHE_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pro_cache_1::R`](R) reader structure"]
impl crate::Readable for PRO_CACHE_1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pro_cache_1::W`](W) writer structure"]
impl crate::Writable for PRO_CACHE_1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PRO_CACHE_1 to value 0"]
impl crate::Resettable for PRO_CACHE_1_SPEC {
    const RESET_VALUE: u32 = 0;
}
