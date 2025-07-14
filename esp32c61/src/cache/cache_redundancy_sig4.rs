#[doc = "Register `CACHE_REDUNDANCY_SIG4` reader"]
pub type R = crate::R<CACHE_REDUNDANCY_SIG4_SPEC>;
#[doc = "Field `CACHE_REDCY_SIG4` reader - Those bits are prepared for ECO."]
pub type CACHE_REDCY_SIG4_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Those bits are prepared for ECO."]
    #[inline(always)]
    pub fn cache_redcy_sig4(&self) -> CACHE_REDCY_SIG4_R {
        CACHE_REDCY_SIG4_R::new((self.bits & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CACHE_REDUNDANCY_SIG4")
            .field("cache_redcy_sig4", &self.cache_redcy_sig4())
            .finish()
    }
}
#[doc = "Cache redundancy signal 0 register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_redundancy_sig4::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CACHE_REDUNDANCY_SIG4_SPEC;
impl crate::RegisterSpec for CACHE_REDUNDANCY_SIG4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cache_redundancy_sig4::R`](R) reader structure"]
impl crate::Readable for CACHE_REDUNDANCY_SIG4_SPEC {}
#[doc = "`reset()` method sets CACHE_REDUNDANCY_SIG4 to value 0"]
impl crate::Resettable for CACHE_REDUNDANCY_SIG4_SPEC {}
