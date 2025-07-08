#[doc = "Register `CACHE_L2_CACHE_PRELOAD_ADDR` reader"]
pub type R = crate::R<CACHE_L2_CACHE_PRELOAD_ADDR_SPEC>;
#[doc = "Field `CACHE_L2_CACHE_PRELOAD_ADDR` reader - Those bits are used to configure the start address of preload on L2-Cache, which should be used together with L2_CACHE_PRELOAD_SIZE_REG"]
pub type CACHE_L2_CACHE_PRELOAD_ADDR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Those bits are used to configure the start address of preload on L2-Cache, which should be used together with L2_CACHE_PRELOAD_SIZE_REG"]
    #[inline(always)]
    pub fn cache_l2_cache_preload_addr(&self) -> CACHE_L2_CACHE_PRELOAD_ADDR_R {
        CACHE_L2_CACHE_PRELOAD_ADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CACHE_L2_CACHE_PRELOAD_ADDR")
            .field(
                "cache_l2_cache_preload_addr",
                &self.cache_l2_cache_preload_addr(),
            )
            .finish()
    }
}
#[doc = "L2 Cache preload address configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_l2_cache_preload_addr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CACHE_L2_CACHE_PRELOAD_ADDR_SPEC;
impl crate::RegisterSpec for CACHE_L2_CACHE_PRELOAD_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cache_l2_cache_preload_addr::R`](R) reader structure"]
impl crate::Readable for CACHE_L2_CACHE_PRELOAD_ADDR_SPEC {}
#[doc = "`reset()` method sets CACHE_L2_CACHE_PRELOAD_ADDR to value 0"]
impl crate::Resettable for CACHE_L2_CACHE_PRELOAD_ADDR_SPEC {}
