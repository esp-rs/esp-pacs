#[doc = "Register `CACHE_LEVEL_SPLIT0` reader"]
pub type R = crate::R<CACHE_LEVEL_SPLIT0_SPEC>;
#[doc = "Field `CACHE_LEVEL_SPLIT0` reader - Reserved"]
pub type CACHE_LEVEL_SPLIT0_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Reserved"]
    #[inline(always)]
    pub fn cache_level_split0(&self) -> CACHE_LEVEL_SPLIT0_R {
        CACHE_LEVEL_SPLIT0_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CACHE_LEVEL_SPLIT0")
            .field("cache_level_split0", &self.cache_level_split0())
            .finish()
    }
}
#[doc = "USED TO SPLIT L1 CACHE AND L2 CACHE\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_level_split0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CACHE_LEVEL_SPLIT0_SPEC;
impl crate::RegisterSpec for CACHE_LEVEL_SPLIT0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cache_level_split0::R`](R) reader structure"]
impl crate::Readable for CACHE_LEVEL_SPLIT0_SPEC {}
#[doc = "`reset()` method sets CACHE_LEVEL_SPLIT0 to value 0x0268"]
impl crate::Resettable for CACHE_LEVEL_SPLIT0_SPEC {
    const RESET_VALUE: u32 = 0x0268;
}
