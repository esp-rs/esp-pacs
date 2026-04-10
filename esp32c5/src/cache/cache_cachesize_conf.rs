#[doc = "Register `CACHE_CACHESIZE_CONF` reader"]
pub type R = crate::R<CACHE_CACHESIZE_CONF_SPEC>;
#[doc = "Field `DCACHE_CACHESIZE_32K` reader - The field is used to configure cachesize of DCache as 32k bytes. This field and all other fields within this register is onehot."]
pub type DCACHE_CACHESIZE_32K_R = crate::BitReader;
impl R {
    #[doc = "Bit 7 - The field is used to configure cachesize of DCache as 32k bytes. This field and all other fields within this register is onehot."]
    #[inline(always)]
    pub fn dcache_cachesize_32k(&self) -> DCACHE_CACHESIZE_32K_R {
        DCACHE_CACHESIZE_32K_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CACHE_CACHESIZE_CONF")
            .field("dcache_cachesize_32k", &self.dcache_cachesize_32k())
            .finish()
    }
}
#[doc = "Data Cache CacheSize mode configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_cachesize_conf::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CACHE_CACHESIZE_CONF_SPEC;
impl crate::RegisterSpec for CACHE_CACHESIZE_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cache_cachesize_conf::R`](R) reader structure"]
impl crate::Readable for CACHE_CACHESIZE_CONF_SPEC {}
#[doc = "`reset()` method sets CACHE_CACHESIZE_CONF to value 0"]
impl crate::Resettable for CACHE_CACHESIZE_CONF_SPEC {}
