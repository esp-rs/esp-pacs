#[doc = "Register `CACHE_AUTOLOAD_SCT2_SIZE` reader"]
pub type R = crate::R<CACHE_AUTOLOAD_SCT2_SIZE_SPEC>;
#[doc = "Field `CACHE_AUTOLOAD_SCT2_SIZE` reader - Those bits are used to configure the size of the third section for autoload operation on L1-Cache. Note that it should be used together with L1_CACHE_AUTOLOAD_SCT2_ADDR and L1_CACHE_AUTOLOAD_SCT2_ENA."]
pub type CACHE_AUTOLOAD_SCT2_SIZE_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:24 - Those bits are used to configure the size of the third section for autoload operation on L1-Cache. Note that it should be used together with L1_CACHE_AUTOLOAD_SCT2_ADDR and L1_CACHE_AUTOLOAD_SCT2_ENA."]
    #[inline(always)]
    pub fn cache_autoload_sct2_size(&self) -> CACHE_AUTOLOAD_SCT2_SIZE_R {
        CACHE_AUTOLOAD_SCT2_SIZE_R::new(self.bits & 0x01ff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CACHE_AUTOLOAD_SCT2_SIZE")
            .field("cache_autoload_sct2_size", &self.cache_autoload_sct2_size())
            .finish()
    }
}
#[doc = "L1 Cache autoload section 2 size configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_autoload_sct2_size::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CACHE_AUTOLOAD_SCT2_SIZE_SPEC;
impl crate::RegisterSpec for CACHE_AUTOLOAD_SCT2_SIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cache_autoload_sct2_size::R`](R) reader structure"]
impl crate::Readable for CACHE_AUTOLOAD_SCT2_SIZE_SPEC {}
#[doc = "`reset()` method sets CACHE_AUTOLOAD_SCT2_SIZE to value 0"]
impl crate::Resettable for CACHE_AUTOLOAD_SCT2_SIZE_SPEC {}
