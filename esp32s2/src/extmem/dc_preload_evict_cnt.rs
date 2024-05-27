#[doc = "Register `DC_PRELOAD_EVICT_CNT` reader"]
pub type R = crate::R<DC_PRELOAD_EVICT_CNT_SPEC>;
#[doc = "Field `DC_PRELOAD_EVICT_CNT` reader - The bits are used to count the number of cache evictions by pre-load which include manual pre-load and conditional pre-load."]
pub type DC_PRELOAD_EVICT_CNT_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - The bits are used to count the number of cache evictions by pre-load which include manual pre-load and conditional pre-load."]
    #[inline(always)]
    pub fn dc_preload_evict_cnt(&self) -> DC_PRELOAD_EVICT_CNT_R {
        DC_PRELOAD_EVICT_CNT_R::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DC_PRELOAD_EVICT_CNT")
            .field("dc_preload_evict_cnt", &self.dc_preload_evict_cnt())
            .finish()
    }
}
#[doc = "register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dc_preload_evict_cnt::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DC_PRELOAD_EVICT_CNT_SPEC;
impl crate::RegisterSpec for DC_PRELOAD_EVICT_CNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dc_preload_evict_cnt::R`](R) reader structure"]
impl crate::Readable for DC_PRELOAD_EVICT_CNT_SPEC {}
#[doc = "`reset()` method sets DC_PRELOAD_EVICT_CNT to value 0"]
impl crate::Resettable for DC_PRELOAD_EVICT_CNT_SPEC {
    const RESET_VALUE: u32 = 0;
}
