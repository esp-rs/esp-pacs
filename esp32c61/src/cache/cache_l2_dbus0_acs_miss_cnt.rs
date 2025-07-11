#[doc = "Register `CACHE_L2_DBUS0_ACS_MISS_CNT` reader"]
pub type R = crate::R<CACHE_L2_DBUS0_ACS_MISS_CNT_SPEC>;
#[doc = "Field `CACHE_L2_DBUS0_MISS_CNT` reader - The register records the number of missing when L1-DCache accesses L2-Cache due to bus0 accessing L1-DCache."]
pub type CACHE_L2_DBUS0_MISS_CNT_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - The register records the number of missing when L1-DCache accesses L2-Cache due to bus0 accessing L1-DCache."]
    #[inline(always)]
    pub fn cache_l2_dbus0_miss_cnt(&self) -> CACHE_L2_DBUS0_MISS_CNT_R {
        CACHE_L2_DBUS0_MISS_CNT_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CACHE_L2_DBUS0_ACS_MISS_CNT")
            .field("cache_l2_dbus0_miss_cnt", &self.cache_l2_dbus0_miss_cnt())
            .finish()
    }
}
#[doc = "L2-Cache bus0 Miss-Access Counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_l2_dbus0_acs_miss_cnt::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CACHE_L2_DBUS0_ACS_MISS_CNT_SPEC;
impl crate::RegisterSpec for CACHE_L2_DBUS0_ACS_MISS_CNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cache_l2_dbus0_acs_miss_cnt::R`](R) reader structure"]
impl crate::Readable for CACHE_L2_DBUS0_ACS_MISS_CNT_SPEC {}
#[doc = "`reset()` method sets CACHE_L2_DBUS0_ACS_MISS_CNT to value 0"]
impl crate::Resettable for CACHE_L2_DBUS0_ACS_MISS_CNT_SPEC {}
