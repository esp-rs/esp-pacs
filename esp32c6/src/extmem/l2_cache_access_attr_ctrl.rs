#[doc = "Register `L2_CACHE_ACCESS_ATTR_CTRL` reader"]
pub struct R(crate::R<L2_CACHE_ACCESS_ATTR_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<L2_CACHE_ACCESS_ATTR_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<L2_CACHE_ACCESS_ATTR_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<L2_CACHE_ACCESS_ATTR_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `L2_CACHE_ACCESS_FORCE_CC` reader - Set this bit to force the request to l2 cache with cacheable attribute, otherwise, the attribute is propagated from L1 cache or CPU, it could be one of cacheable and non-cacheable."]
pub type L2_CACHE_ACCESS_FORCE_CC_R = crate::BitReader;
#[doc = "Field `L2_CACHE_ACCESS_FORCE_WB` reader - Set this bit to force the request to l2 cache with write-back attribute, otherwise, the attribute is propagated from L1 cache or CPU, it could be one of write-back and write-through."]
pub type L2_CACHE_ACCESS_FORCE_WB_R = crate::BitReader;
#[doc = "Field `L2_CACHE_ACCESS_FORCE_WMA` reader - Set this bit to force the request to l2 cache with write-miss-allocate attribute, otherwise, the attribute is propagated from L1 cache or CPU, it could be one of write-miss-allocate and write-miss-no-allocate."]
pub type L2_CACHE_ACCESS_FORCE_WMA_R = crate::BitReader;
#[doc = "Field `L2_CACHE_ACCESS_FORCE_RMA` reader - Set this bit to force the request to l2 cache with read-miss-allocate attribute, otherwise, the attribute is propagated from L1 cache or CPU, it could be one of read-miss-allocate and read-miss-no-allocate."]
pub type L2_CACHE_ACCESS_FORCE_RMA_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Set this bit to force the request to l2 cache with cacheable attribute, otherwise, the attribute is propagated from L1 cache or CPU, it could be one of cacheable and non-cacheable."]
    #[inline(always)]
    pub fn l2_cache_access_force_cc(&self) -> L2_CACHE_ACCESS_FORCE_CC_R {
        L2_CACHE_ACCESS_FORCE_CC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set this bit to force the request to l2 cache with write-back attribute, otherwise, the attribute is propagated from L1 cache or CPU, it could be one of write-back and write-through."]
    #[inline(always)]
    pub fn l2_cache_access_force_wb(&self) -> L2_CACHE_ACCESS_FORCE_WB_R {
        L2_CACHE_ACCESS_FORCE_WB_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set this bit to force the request to l2 cache with write-miss-allocate attribute, otherwise, the attribute is propagated from L1 cache or CPU, it could be one of write-miss-allocate and write-miss-no-allocate."]
    #[inline(always)]
    pub fn l2_cache_access_force_wma(&self) -> L2_CACHE_ACCESS_FORCE_WMA_R {
        L2_CACHE_ACCESS_FORCE_WMA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Set this bit to force the request to l2 cache with read-miss-allocate attribute, otherwise, the attribute is propagated from L1 cache or CPU, it could be one of read-miss-allocate and read-miss-no-allocate."]
    #[inline(always)]
    pub fn l2_cache_access_force_rma(&self) -> L2_CACHE_ACCESS_FORCE_RMA_R {
        L2_CACHE_ACCESS_FORCE_RMA_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L2_CACHE_ACCESS_ATTR_CTRL")
            .field(
                "l2_cache_access_force_cc",
                &format_args!("{}", self.l2_cache_access_force_cc().bit()),
            )
            .field(
                "l2_cache_access_force_wb",
                &format_args!("{}", self.l2_cache_access_force_wb().bit()),
            )
            .field(
                "l2_cache_access_force_wma",
                &format_args!("{}", self.l2_cache_access_force_wma().bit()),
            )
            .field(
                "l2_cache_access_force_rma",
                &format_args!("{}", self.l2_cache_access_force_rma().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<L2_CACHE_ACCESS_ATTR_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "L1 Cache access Attribute propagation control register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [l2_cache_access_attr_ctrl](index.html) module"]
pub struct L2_CACHE_ACCESS_ATTR_CTRL_SPEC;
impl crate::RegisterSpec for L2_CACHE_ACCESS_ATTR_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [l2_cache_access_attr_ctrl::R](R) reader structure"]
impl crate::Readable for L2_CACHE_ACCESS_ATTR_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets L2_CACHE_ACCESS_ATTR_CTRL to value 0x0f"]
impl crate::Resettable for L2_CACHE_ACCESS_ATTR_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0f;
}
