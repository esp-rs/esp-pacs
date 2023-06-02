#[doc = "Register `L2_CACHE_PRELOCK_CONF` reader"]
pub struct R(crate::R<L2_CACHE_PRELOCK_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<L2_CACHE_PRELOCK_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<L2_CACHE_PRELOCK_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<L2_CACHE_PRELOCK_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `L2_CACHE_PRELOCK_SCT0_EN` reader - The bit is used to enable the first section of prelock function on L2-Cache."]
pub type L2_CACHE_PRELOCK_SCT0_EN_R = crate::BitReader;
#[doc = "Field `L2_CACHE_PRELOCK_SCT1_EN` reader - The bit is used to enable the second section of prelock function on L2-Cache."]
pub type L2_CACHE_PRELOCK_SCT1_EN_R = crate::BitReader;
#[doc = "Field `L2_CACHE_PRELOCK_RGID` reader - The bit is used to set the gid of l2 cache prelock."]
pub type L2_CACHE_PRELOCK_RGID_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - The bit is used to enable the first section of prelock function on L2-Cache."]
    #[inline(always)]
    pub fn l2_cache_prelock_sct0_en(&self) -> L2_CACHE_PRELOCK_SCT0_EN_R {
        L2_CACHE_PRELOCK_SCT0_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The bit is used to enable the second section of prelock function on L2-Cache."]
    #[inline(always)]
    pub fn l2_cache_prelock_sct1_en(&self) -> L2_CACHE_PRELOCK_SCT1_EN_R {
        L2_CACHE_PRELOCK_SCT1_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:5 - The bit is used to set the gid of l2 cache prelock."]
    #[inline(always)]
    pub fn l2_cache_prelock_rgid(&self) -> L2_CACHE_PRELOCK_RGID_R {
        L2_CACHE_PRELOCK_RGID_R::new(((self.bits >> 2) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L2_CACHE_PRELOCK_CONF")
            .field(
                "l2_cache_prelock_sct0_en",
                &format_args!("{}", self.l2_cache_prelock_sct0_en().bit()),
            )
            .field(
                "l2_cache_prelock_sct1_en",
                &format_args!("{}", self.l2_cache_prelock_sct1_en().bit()),
            )
            .field(
                "l2_cache_prelock_rgid",
                &format_args!("{}", self.l2_cache_prelock_rgid().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<L2_CACHE_PRELOCK_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "L2 Cache prelock configure register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [l2_cache_prelock_conf](index.html) module"]
pub struct L2_CACHE_PRELOCK_CONF_SPEC;
impl crate::RegisterSpec for L2_CACHE_PRELOCK_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [l2_cache_prelock_conf::R](R) reader structure"]
impl crate::Readable for L2_CACHE_PRELOCK_CONF_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets L2_CACHE_PRELOCK_CONF to value 0"]
impl crate::Resettable for L2_CACHE_PRELOCK_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
