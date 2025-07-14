#[doc = "Register `CACHE_L2_CACHE_ACCESS_ATTR_CTRL` reader"]
pub type R = crate::R<CACHE_L2_CACHE_ACCESS_ATTR_CTRL_SPEC>;
#[doc = "Field `CACHE_L2_CACHE_ACCESS_FORCE_CC` reader - Set this bit to force the request to l2 cache with cacheable attribute, otherwise, the attribute is propagated from L1 cache or CPU, it could be one of cacheable and non-cacheable."]
pub type CACHE_L2_CACHE_ACCESS_FORCE_CC_R = crate::BitReader;
#[doc = "Field `CACHE_L2_CACHE_ACCESS_FORCE_WB` reader - Set this bit to force the request to l2 cache with write-back attribute, otherwise, the attribute is propagated from L1 cache or CPU, it could be one of write-back and write-through."]
pub type CACHE_L2_CACHE_ACCESS_FORCE_WB_R = crate::BitReader;
#[doc = "Field `CACHE_L2_CACHE_ACCESS_FORCE_WMA` reader - Set this bit to force the request to l2 cache with write-miss-allocate attribute, otherwise, the attribute is propagated from L1 cache or CPU, it could be one of write-miss-allocate and write-miss-no-allocate."]
pub type CACHE_L2_CACHE_ACCESS_FORCE_WMA_R = crate::BitReader;
#[doc = "Field `CACHE_L2_CACHE_ACCESS_FORCE_RMA` reader - Set this bit to force the request to l2 cache with read-miss-allocate attribute, otherwise, the attribute is propagated from L1 cache or CPU, it could be one of read-miss-allocate and read-miss-no-allocate."]
pub type CACHE_L2_CACHE_ACCESS_FORCE_RMA_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Set this bit to force the request to l2 cache with cacheable attribute, otherwise, the attribute is propagated from L1 cache or CPU, it could be one of cacheable and non-cacheable."]
    #[inline(always)]
    pub fn cache_l2_cache_access_force_cc(&self) -> CACHE_L2_CACHE_ACCESS_FORCE_CC_R {
        CACHE_L2_CACHE_ACCESS_FORCE_CC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set this bit to force the request to l2 cache with write-back attribute, otherwise, the attribute is propagated from L1 cache or CPU, it could be one of write-back and write-through."]
    #[inline(always)]
    pub fn cache_l2_cache_access_force_wb(&self) -> CACHE_L2_CACHE_ACCESS_FORCE_WB_R {
        CACHE_L2_CACHE_ACCESS_FORCE_WB_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set this bit to force the request to l2 cache with write-miss-allocate attribute, otherwise, the attribute is propagated from L1 cache or CPU, it could be one of write-miss-allocate and write-miss-no-allocate."]
    #[inline(always)]
    pub fn cache_l2_cache_access_force_wma(&self) -> CACHE_L2_CACHE_ACCESS_FORCE_WMA_R {
        CACHE_L2_CACHE_ACCESS_FORCE_WMA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Set this bit to force the request to l2 cache with read-miss-allocate attribute, otherwise, the attribute is propagated from L1 cache or CPU, it could be one of read-miss-allocate and read-miss-no-allocate."]
    #[inline(always)]
    pub fn cache_l2_cache_access_force_rma(&self) -> CACHE_L2_CACHE_ACCESS_FORCE_RMA_R {
        CACHE_L2_CACHE_ACCESS_FORCE_RMA_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CACHE_L2_CACHE_ACCESS_ATTR_CTRL")
            .field(
                "cache_l2_cache_access_force_cc",
                &self.cache_l2_cache_access_force_cc(),
            )
            .field(
                "cache_l2_cache_access_force_wb",
                &self.cache_l2_cache_access_force_wb(),
            )
            .field(
                "cache_l2_cache_access_force_wma",
                &self.cache_l2_cache_access_force_wma(),
            )
            .field(
                "cache_l2_cache_access_force_rma",
                &self.cache_l2_cache_access_force_rma(),
            )
            .finish()
    }
}
#[doc = "L2 cache access attribute control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_l2_cache_access_attr_ctrl::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CACHE_L2_CACHE_ACCESS_ATTR_CTRL_SPEC;
impl crate::RegisterSpec for CACHE_L2_CACHE_ACCESS_ATTR_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cache_l2_cache_access_attr_ctrl::R`](R) reader structure"]
impl crate::Readable for CACHE_L2_CACHE_ACCESS_ATTR_CTRL_SPEC {}
#[doc = "`reset()` method sets CACHE_L2_CACHE_ACCESS_ATTR_CTRL to value 0x0f"]
impl crate::Resettable for CACHE_L2_CACHE_ACCESS_ATTR_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x0f;
}
