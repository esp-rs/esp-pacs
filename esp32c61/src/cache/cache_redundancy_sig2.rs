#[doc = "Register `CACHE_REDUNDANCY_SIG2` reader"]
pub type R = crate::R<CACHE_REDUNDANCY_SIG2_SPEC>;
#[doc = "Register `CACHE_REDUNDANCY_SIG2` writer"]
pub type W = crate::W<CACHE_REDUNDANCY_SIG2_SPEC>;
#[doc = "Field `CACHE_REDCY_SIG2` reader - Those bits are prepared for ECO."]
pub type CACHE_REDCY_SIG2_R = crate::FieldReader<u32>;
#[doc = "Field `CACHE_REDCY_SIG2` writer - Those bits are prepared for ECO."]
pub type CACHE_REDCY_SIG2_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Those bits are prepared for ECO."]
    #[inline(always)]
    pub fn cache_redcy_sig2(&self) -> CACHE_REDCY_SIG2_R {
        CACHE_REDCY_SIG2_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CACHE_REDUNDANCY_SIG2")
            .field("cache_redcy_sig2", &self.cache_redcy_sig2())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Those bits are prepared for ECO."]
    #[inline(always)]
    pub fn cache_redcy_sig2(&mut self) -> CACHE_REDCY_SIG2_W<CACHE_REDUNDANCY_SIG2_SPEC> {
        CACHE_REDCY_SIG2_W::new(self, 0)
    }
}
#[doc = "Cache redundancy signal 2 register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_redundancy_sig2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_redundancy_sig2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CACHE_REDUNDANCY_SIG2_SPEC;
impl crate::RegisterSpec for CACHE_REDUNDANCY_SIG2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cache_redundancy_sig2::R`](R) reader structure"]
impl crate::Readable for CACHE_REDUNDANCY_SIG2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cache_redundancy_sig2::W`](W) writer structure"]
impl crate::Writable for CACHE_REDUNDANCY_SIG2_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CACHE_REDUNDANCY_SIG2 to value 0"]
impl crate::Resettable for CACHE_REDUNDANCY_SIG2_SPEC {}
