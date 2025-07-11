#[doc = "Register `CACHE_L2_CACHE_ADDR` reader"]
pub type R = crate::R<CACHE_L2_CACHE_ADDR_SPEC>;
#[doc = "Field `CACHE_L2_CACHE_ADDR` reader - Those bits stores the address which will decide where inside the specified tag memory object will be accessed."]
pub type CACHE_L2_CACHE_ADDR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Those bits stores the address which will decide where inside the specified tag memory object will be accessed."]
    #[inline(always)]
    pub fn cache_l2_cache_addr(&self) -> CACHE_L2_CACHE_ADDR_R {
        CACHE_L2_CACHE_ADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CACHE_L2_CACHE_ADDR")
            .field("cache_l2_cache_addr", &self.cache_l2_cache_addr())
            .finish()
    }
}
#[doc = "Cache address register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_l2_cache_addr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CACHE_L2_CACHE_ADDR_SPEC;
impl crate::RegisterSpec for CACHE_L2_CACHE_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cache_l2_cache_addr::R`](R) reader structure"]
impl crate::Readable for CACHE_L2_CACHE_ADDR_SPEC {}
#[doc = "`reset()` method sets CACHE_L2_CACHE_ADDR to value 0"]
impl crate::Resettable for CACHE_L2_CACHE_ADDR_SPEC {}
