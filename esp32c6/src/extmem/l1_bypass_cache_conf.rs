#[doc = "Register `L1_BYPASS_CACHE_CONF` reader"]
pub struct R(crate::R<L1_BYPASS_CACHE_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<L1_BYPASS_CACHE_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<L1_BYPASS_CACHE_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<L1_BYPASS_CACHE_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BYPASS_L1_ICACHE0_EN` reader - The bit is used to enable bypass L1-ICache0. 0: disable bypass, 1: enable bypass."]
pub type BYPASS_L1_ICACHE0_EN_R = crate::BitReader;
#[doc = "Field `BYPASS_L1_ICACHE1_EN` reader - The bit is used to enable bypass L1-ICache1. 0: disable bypass, 1: enable bypass."]
pub type BYPASS_L1_ICACHE1_EN_R = crate::BitReader;
#[doc = "Field `BYPASS_L1_ICACHE2_EN` reader - Reserved"]
pub type BYPASS_L1_ICACHE2_EN_R = crate::BitReader;
#[doc = "Field `BYPASS_L1_ICACHE3_EN` reader - Reserved"]
pub type BYPASS_L1_ICACHE3_EN_R = crate::BitReader;
#[doc = "Field `BYPASS_L1_DCACHE_EN` reader - The bit is used to enable bypass L1-DCache. 0: disable bypass, 1: enable bypass."]
pub type BYPASS_L1_DCACHE_EN_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - The bit is used to enable bypass L1-ICache0. 0: disable bypass, 1: enable bypass."]
    #[inline(always)]
    pub fn bypass_l1_icache0_en(&self) -> BYPASS_L1_ICACHE0_EN_R {
        BYPASS_L1_ICACHE0_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The bit is used to enable bypass L1-ICache1. 0: disable bypass, 1: enable bypass."]
    #[inline(always)]
    pub fn bypass_l1_icache1_en(&self) -> BYPASS_L1_ICACHE1_EN_R {
        BYPASS_L1_ICACHE1_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Reserved"]
    #[inline(always)]
    pub fn bypass_l1_icache2_en(&self) -> BYPASS_L1_ICACHE2_EN_R {
        BYPASS_L1_ICACHE2_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Reserved"]
    #[inline(always)]
    pub fn bypass_l1_icache3_en(&self) -> BYPASS_L1_ICACHE3_EN_R {
        BYPASS_L1_ICACHE3_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The bit is used to enable bypass L1-DCache. 0: disable bypass, 1: enable bypass."]
    #[inline(always)]
    pub fn bypass_l1_dcache_en(&self) -> BYPASS_L1_DCACHE_EN_R {
        BYPASS_L1_DCACHE_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L1_BYPASS_CACHE_CONF")
            .field(
                "bypass_l1_icache0_en",
                &format_args!("{}", self.bypass_l1_icache0_en().bit()),
            )
            .field(
                "bypass_l1_icache1_en",
                &format_args!("{}", self.bypass_l1_icache1_en().bit()),
            )
            .field(
                "bypass_l1_icache2_en",
                &format_args!("{}", self.bypass_l1_icache2_en().bit()),
            )
            .field(
                "bypass_l1_icache3_en",
                &format_args!("{}", self.bypass_l1_icache3_en().bit()),
            )
            .field(
                "bypass_l1_dcache_en",
                &format_args!("{}", self.bypass_l1_dcache_en().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<L1_BYPASS_CACHE_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Bypass Cache configure register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [l1_bypass_cache_conf](index.html) module"]
pub struct L1_BYPASS_CACHE_CONF_SPEC;
impl crate::RegisterSpec for L1_BYPASS_CACHE_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [l1_bypass_cache_conf::R](R) reader structure"]
impl crate::Readable for L1_BYPASS_CACHE_CONF_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets L1_BYPASS_CACHE_CONF to value 0"]
impl crate::Resettable for L1_BYPASS_CACHE_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
