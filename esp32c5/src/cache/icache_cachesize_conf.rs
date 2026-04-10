#[doc = "Register `ICACHE_CACHESIZE_CONF` reader"]
pub type R = crate::R<ICACHE_CACHESIZE_CONF_SPEC>;
#[doc = "Field `ICACHE_CACHESIZE_4K` reader - The field is used to configure cachesize of ICache as 4k bytes. This field and all other fields within this register is onehot."]
pub type ICACHE_CACHESIZE_4K_R = crate::BitReader;
impl R {
    #[doc = "Bit 4 - The field is used to configure cachesize of ICache as 4k bytes. This field and all other fields within this register is onehot."]
    #[inline(always)]
    pub fn icache_cachesize_4k(&self) -> ICACHE_CACHESIZE_4K_R {
        ICACHE_CACHESIZE_4K_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICACHE_CACHESIZE_CONF")
            .field("icache_cachesize_4k", &self.icache_cachesize_4k())
            .finish()
    }
}
#[doc = "Instruction Cache CacheSize mode configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`icache_cachesize_conf::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ICACHE_CACHESIZE_CONF_SPEC;
impl crate::RegisterSpec for ICACHE_CACHESIZE_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icache_cachesize_conf::R`](R) reader structure"]
impl crate::Readable for ICACHE_CACHESIZE_CONF_SPEC {}
#[doc = "`reset()` method sets ICACHE_CACHESIZE_CONF to value 0"]
impl crate::Resettable for ICACHE_CACHESIZE_CONF_SPEC {}
