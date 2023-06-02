#[doc = "Register `L2_BYPASS_CACHE_CONF` reader"]
pub struct R(crate::R<L2_BYPASS_CACHE_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<L2_BYPASS_CACHE_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<L2_BYPASS_CACHE_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<L2_BYPASS_CACHE_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BYPASS_L2_CACHE_EN` reader - The bit is used to enable bypass L2-Cache. 0: disable bypass, 1: enable bypass."]
pub type BYPASS_L2_CACHE_EN_R = crate::BitReader;
impl R {
    #[doc = "Bit 5 - The bit is used to enable bypass L2-Cache. 0: disable bypass, 1: enable bypass."]
    #[inline(always)]
    pub fn bypass_l2_cache_en(&self) -> BYPASS_L2_CACHE_EN_R {
        BYPASS_L2_CACHE_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L2_BYPASS_CACHE_CONF")
            .field(
                "bypass_l2_cache_en",
                &format_args!("{}", self.bypass_l2_cache_en().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<L2_BYPASS_CACHE_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Bypass Cache configure register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [l2_bypass_cache_conf](index.html) module"]
pub struct L2_BYPASS_CACHE_CONF_SPEC;
impl crate::RegisterSpec for L2_BYPASS_CACHE_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [l2_bypass_cache_conf::R](R) reader structure"]
impl crate::Readable for L2_BYPASS_CACHE_CONF_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets L2_BYPASS_CACHE_CONF to value 0"]
impl crate::Resettable for L2_BYPASS_CACHE_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
