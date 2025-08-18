#[doc = "Register `CACHE_AUTOLOAD_SCT0_SIZE` reader"]
pub type R = crate::R<CACHE_AUTOLOAD_SCT0_SIZE_SPEC>;
#[doc = "Register `CACHE_AUTOLOAD_SCT0_SIZE` writer"]
pub type W = crate::W<CACHE_AUTOLOAD_SCT0_SIZE_SPEC>;
#[doc = "Field `CACHE_AUTOLOAD_SCT0_SIZE` reader - Those bits are used to configure the size of the first section for autoload operation on L1-Cache. Note that it should be used together with L1_CACHE_AUTOLOAD_SCT0_ADDR and L1_CACHE_AUTOLOAD_SCT0_ENA."]
pub type CACHE_AUTOLOAD_SCT0_SIZE_R = crate::FieldReader<u32>;
#[doc = "Field `CACHE_AUTOLOAD_SCT0_SIZE` writer - Those bits are used to configure the size of the first section for autoload operation on L1-Cache. Note that it should be used together with L1_CACHE_AUTOLOAD_SCT0_ADDR and L1_CACHE_AUTOLOAD_SCT0_ENA."]
pub type CACHE_AUTOLOAD_SCT0_SIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 25, u32>;
impl R {
    #[doc = "Bits 0:24 - Those bits are used to configure the size of the first section for autoload operation on L1-Cache. Note that it should be used together with L1_CACHE_AUTOLOAD_SCT0_ADDR and L1_CACHE_AUTOLOAD_SCT0_ENA."]
    #[inline(always)]
    pub fn cache_autoload_sct0_size(&self) -> CACHE_AUTOLOAD_SCT0_SIZE_R {
        CACHE_AUTOLOAD_SCT0_SIZE_R::new(self.bits & 0x01ff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CACHE_AUTOLOAD_SCT0_SIZE")
            .field("cache_autoload_sct0_size", &self.cache_autoload_sct0_size())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:24 - Those bits are used to configure the size of the first section for autoload operation on L1-Cache. Note that it should be used together with L1_CACHE_AUTOLOAD_SCT0_ADDR and L1_CACHE_AUTOLOAD_SCT0_ENA."]
    #[inline(always)]
    pub fn cache_autoload_sct0_size(
        &mut self,
    ) -> CACHE_AUTOLOAD_SCT0_SIZE_W<'_, CACHE_AUTOLOAD_SCT0_SIZE_SPEC> {
        CACHE_AUTOLOAD_SCT0_SIZE_W::new(self, 0)
    }
}
#[doc = "L1 Cache autoload section 0 size configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_autoload_sct0_size::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_autoload_sct0_size::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CACHE_AUTOLOAD_SCT0_SIZE_SPEC;
impl crate::RegisterSpec for CACHE_AUTOLOAD_SCT0_SIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cache_autoload_sct0_size::R`](R) reader structure"]
impl crate::Readable for CACHE_AUTOLOAD_SCT0_SIZE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cache_autoload_sct0_size::W`](W) writer structure"]
impl crate::Writable for CACHE_AUTOLOAD_SCT0_SIZE_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CACHE_AUTOLOAD_SCT0_SIZE to value 0"]
impl crate::Resettable for CACHE_AUTOLOAD_SCT0_SIZE_SPEC {}
