#[doc = "Register `CACHE_L2_CACHE_AUTOLOAD_SCT3_ADDR` reader"]
pub type R = crate::R<CACHE_L2_CACHE_AUTOLOAD_SCT3_ADDR_SPEC>;
#[doc = "Field `CACHE_L2_CACHE_AUTOLOAD_SCT3_ADDR` reader - Those bits are used to configure the start address of the fourth section for autoload operation on L2-Cache. Note that it should be used together with L2_CACHE_AUTOLOAD_SCT3_SIZE and L2_CACHE_AUTOLOAD_SCT3_ENA."]
pub type CACHE_L2_CACHE_AUTOLOAD_SCT3_ADDR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Those bits are used to configure the start address of the fourth section for autoload operation on L2-Cache. Note that it should be used together with L2_CACHE_AUTOLOAD_SCT3_SIZE and L2_CACHE_AUTOLOAD_SCT3_ENA."]
    #[inline(always)]
    pub fn cache_l2_cache_autoload_sct3_addr(&self) -> CACHE_L2_CACHE_AUTOLOAD_SCT3_ADDR_R {
        CACHE_L2_CACHE_AUTOLOAD_SCT3_ADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CACHE_L2_CACHE_AUTOLOAD_SCT3_ADDR")
            .field(
                "cache_l2_cache_autoload_sct3_addr",
                &self.cache_l2_cache_autoload_sct3_addr(),
            )
            .finish()
    }
}
#[doc = "L2 Cache autoload section 3 address configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_l2_cache_autoload_sct3_addr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CACHE_L2_CACHE_AUTOLOAD_SCT3_ADDR_SPEC;
impl crate::RegisterSpec for CACHE_L2_CACHE_AUTOLOAD_SCT3_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cache_l2_cache_autoload_sct3_addr::R`](R) reader structure"]
impl crate::Readable for CACHE_L2_CACHE_AUTOLOAD_SCT3_ADDR_SPEC {}
#[doc = "`reset()` method sets CACHE_L2_CACHE_AUTOLOAD_SCT3_ADDR to value 0"]
impl crate::Resettable for CACHE_L2_CACHE_AUTOLOAD_SCT3_ADDR_SPEC {}
