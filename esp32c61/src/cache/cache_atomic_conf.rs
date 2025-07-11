#[doc = "Register `CACHE_ATOMIC_CONF` reader"]
pub type R = crate::R<CACHE_ATOMIC_CONF_SPEC>;
#[doc = "Field `CACHE_ATOMIC_EN` reader - The bit is used to enable atomic feature on L1-Cache when multiple cores access L1-Cache. 1: disable, 1: enable."]
pub type CACHE_ATOMIC_EN_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - The bit is used to enable atomic feature on L1-Cache when multiple cores access L1-Cache. 1: disable, 1: enable."]
    #[inline(always)]
    pub fn cache_atomic_en(&self) -> CACHE_ATOMIC_EN_R {
        CACHE_ATOMIC_EN_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CACHE_ATOMIC_CONF")
            .field("cache_atomic_en", &self.cache_atomic_en())
            .finish()
    }
}
#[doc = "L1 Cache atomic feature configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_atomic_conf::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CACHE_ATOMIC_CONF_SPEC;
impl crate::RegisterSpec for CACHE_ATOMIC_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cache_atomic_conf::R`](R) reader structure"]
impl crate::Readable for CACHE_ATOMIC_CONF_SPEC {}
#[doc = "`reset()` method sets CACHE_ATOMIC_CONF to value 0"]
impl crate::Resettable for CACHE_ATOMIC_CONF_SPEC {}
