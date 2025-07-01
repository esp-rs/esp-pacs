#[doc = "Register `L2_CACHE_AUTOLOAD_SCT3_SIZE` reader"]
pub type R = crate::R<L2_CACHE_AUTOLOAD_SCT3_SIZE_SPEC>;
#[doc = "Register `L2_CACHE_AUTOLOAD_SCT3_SIZE` writer"]
pub type W = crate::W<L2_CACHE_AUTOLOAD_SCT3_SIZE_SPEC>;
#[doc = "Field `L2_CACHE_AUTOLOAD_SCT3_SIZE` reader - Those bits are used to configure the size of the fourth section for autoload operation on L2-Cache. Note that it should be used together with L2_CACHE_AUTOLOAD_SCT3_ADDR and L2_CACHE_AUTOLOAD_SCT3_ENA."]
pub type L2_CACHE_AUTOLOAD_SCT3_SIZE_R = crate::FieldReader<u32>;
#[doc = "Field `L2_CACHE_AUTOLOAD_SCT3_SIZE` writer - Those bits are used to configure the size of the fourth section for autoload operation on L2-Cache. Note that it should be used together with L2_CACHE_AUTOLOAD_SCT3_ADDR and L2_CACHE_AUTOLOAD_SCT3_ENA."]
pub type L2_CACHE_AUTOLOAD_SCT3_SIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bits 0:27 - Those bits are used to configure the size of the fourth section for autoload operation on L2-Cache. Note that it should be used together with L2_CACHE_AUTOLOAD_SCT3_ADDR and L2_CACHE_AUTOLOAD_SCT3_ENA."]
    #[inline(always)]
    pub fn l2_cache_autoload_sct3_size(&self) -> L2_CACHE_AUTOLOAD_SCT3_SIZE_R {
        L2_CACHE_AUTOLOAD_SCT3_SIZE_R::new(self.bits & 0x0fff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L2_CACHE_AUTOLOAD_SCT3_SIZE")
            .field(
                "l2_cache_autoload_sct3_size",
                &self.l2_cache_autoload_sct3_size(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:27 - Those bits are used to configure the size of the fourth section for autoload operation on L2-Cache. Note that it should be used together with L2_CACHE_AUTOLOAD_SCT3_ADDR and L2_CACHE_AUTOLOAD_SCT3_ENA."]
    #[inline(always)]
    pub fn l2_cache_autoload_sct3_size(
        &mut self,
    ) -> L2_CACHE_AUTOLOAD_SCT3_SIZE_W<L2_CACHE_AUTOLOAD_SCT3_SIZE_SPEC> {
        L2_CACHE_AUTOLOAD_SCT3_SIZE_W::new(self, 0)
    }
}
#[doc = "L2 Cache autoload section 3 size configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`l2_cache_autoload_sct3_size::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l2_cache_autoload_sct3_size::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L2_CACHE_AUTOLOAD_SCT3_SIZE_SPEC;
impl crate::RegisterSpec for L2_CACHE_AUTOLOAD_SCT3_SIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l2_cache_autoload_sct3_size::R`](R) reader structure"]
impl crate::Readable for L2_CACHE_AUTOLOAD_SCT3_SIZE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`l2_cache_autoload_sct3_size::W`](W) writer structure"]
impl crate::Writable for L2_CACHE_AUTOLOAD_SCT3_SIZE_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets L2_CACHE_AUTOLOAD_SCT3_SIZE to value 0"]
impl crate::Resettable for L2_CACHE_AUTOLOAD_SCT3_SIZE_SPEC {}
