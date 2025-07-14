#[doc = "Register `CACHE_L2_CACHE_WRAP_AROUND_CTRL` reader"]
pub type R = crate::R<CACHE_L2_CACHE_WRAP_AROUND_CTRL_SPEC>;
#[doc = "Field `CACHE_L2_CACHE_WRAP` reader - Set this bit as 1 to enable L2-Cache wrap around mode."]
pub type CACHE_L2_CACHE_WRAP_R = crate::BitReader;
impl R {
    #[doc = "Bit 5 - Set this bit as 1 to enable L2-Cache wrap around mode."]
    #[inline(always)]
    pub fn cache_l2_cache_wrap(&self) -> CACHE_L2_CACHE_WRAP_R {
        CACHE_L2_CACHE_WRAP_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CACHE_L2_CACHE_WRAP_AROUND_CTRL")
            .field("cache_l2_cache_wrap", &self.cache_l2_cache_wrap())
            .finish()
    }
}
#[doc = "Cache wrap around control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_l2_cache_wrap_around_ctrl::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CACHE_L2_CACHE_WRAP_AROUND_CTRL_SPEC;
impl crate::RegisterSpec for CACHE_L2_CACHE_WRAP_AROUND_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cache_l2_cache_wrap_around_ctrl::R`](R) reader structure"]
impl crate::Readable for CACHE_L2_CACHE_WRAP_AROUND_CTRL_SPEC {}
#[doc = "`reset()` method sets CACHE_L2_CACHE_WRAP_AROUND_CTRL to value 0"]
impl crate::Resettable for CACHE_L2_CACHE_WRAP_AROUND_CTRL_SPEC {}
