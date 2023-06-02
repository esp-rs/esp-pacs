#[doc = "Register `L2_CACHE_TAG_MEM_POWER_CTRL` reader"]
pub struct R(crate::R<L2_CACHE_TAG_MEM_POWER_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<L2_CACHE_TAG_MEM_POWER_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<L2_CACHE_TAG_MEM_POWER_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<L2_CACHE_TAG_MEM_POWER_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `L2_CACHE_TAG_MEM_FORCE_ON` reader - The bit is used to close clock gating of L2-Cache tag memory. 1: close gating, 0: open clock gating."]
pub type L2_CACHE_TAG_MEM_FORCE_ON_R = crate::BitReader;
#[doc = "Field `L2_CACHE_TAG_MEM_FORCE_PD` reader - The bit is used to power L2-Cache tag memory down. 0: follow rtc_lslp, 1: power down"]
pub type L2_CACHE_TAG_MEM_FORCE_PD_R = crate::BitReader;
#[doc = "Field `L2_CACHE_TAG_MEM_FORCE_PU` reader - The bit is used to power L2-Cache tag memory up. 0: follow rtc_lslp, 1: power up"]
pub type L2_CACHE_TAG_MEM_FORCE_PU_R = crate::BitReader;
impl R {
    #[doc = "Bit 20 - The bit is used to close clock gating of L2-Cache tag memory. 1: close gating, 0: open clock gating."]
    #[inline(always)]
    pub fn l2_cache_tag_mem_force_on(&self) -> L2_CACHE_TAG_MEM_FORCE_ON_R {
        L2_CACHE_TAG_MEM_FORCE_ON_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - The bit is used to power L2-Cache tag memory down. 0: follow rtc_lslp, 1: power down"]
    #[inline(always)]
    pub fn l2_cache_tag_mem_force_pd(&self) -> L2_CACHE_TAG_MEM_FORCE_PD_R {
        L2_CACHE_TAG_MEM_FORCE_PD_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - The bit is used to power L2-Cache tag memory up. 0: follow rtc_lslp, 1: power up"]
    #[inline(always)]
    pub fn l2_cache_tag_mem_force_pu(&self) -> L2_CACHE_TAG_MEM_FORCE_PU_R {
        L2_CACHE_TAG_MEM_FORCE_PU_R::new(((self.bits >> 22) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L2_CACHE_TAG_MEM_POWER_CTRL")
            .field(
                "l2_cache_tag_mem_force_on",
                &format_args!("{}", self.l2_cache_tag_mem_force_on().bit()),
            )
            .field(
                "l2_cache_tag_mem_force_pd",
                &format_args!("{}", self.l2_cache_tag_mem_force_pd().bit()),
            )
            .field(
                "l2_cache_tag_mem_force_pu",
                &format_args!("{}", self.l2_cache_tag_mem_force_pu().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<L2_CACHE_TAG_MEM_POWER_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Cache tag memory power control register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [l2_cache_tag_mem_power_ctrl](index.html) module"]
pub struct L2_CACHE_TAG_MEM_POWER_CTRL_SPEC;
impl crate::RegisterSpec for L2_CACHE_TAG_MEM_POWER_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [l2_cache_tag_mem_power_ctrl::R](R) reader structure"]
impl crate::Readable for L2_CACHE_TAG_MEM_POWER_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets L2_CACHE_TAG_MEM_POWER_CTRL to value 0"]
impl crate::Resettable for L2_CACHE_TAG_MEM_POWER_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
