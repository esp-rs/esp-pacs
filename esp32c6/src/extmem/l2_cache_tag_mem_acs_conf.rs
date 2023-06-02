#[doc = "Register `L2_CACHE_TAG_MEM_ACS_CONF` reader"]
pub struct R(crate::R<L2_CACHE_TAG_MEM_ACS_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<L2_CACHE_TAG_MEM_ACS_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<L2_CACHE_TAG_MEM_ACS_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<L2_CACHE_TAG_MEM_ACS_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `L2_CACHE_TAG_MEM_RD_EN` reader - The bit is used to enable config-bus read L2-Cache tag memoryory. 0: disable, 1: enable."]
pub type L2_CACHE_TAG_MEM_RD_EN_R = crate::BitReader;
#[doc = "Field `L2_CACHE_TAG_MEM_WR_EN` reader - The bit is used to enable config-bus write L2-Cache tag memoryory. 0: disable, 1: enable."]
pub type L2_CACHE_TAG_MEM_WR_EN_R = crate::BitReader;
impl R {
    #[doc = "Bit 20 - The bit is used to enable config-bus read L2-Cache tag memoryory. 0: disable, 1: enable."]
    #[inline(always)]
    pub fn l2_cache_tag_mem_rd_en(&self) -> L2_CACHE_TAG_MEM_RD_EN_R {
        L2_CACHE_TAG_MEM_RD_EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - The bit is used to enable config-bus write L2-Cache tag memoryory. 0: disable, 1: enable."]
    #[inline(always)]
    pub fn l2_cache_tag_mem_wr_en(&self) -> L2_CACHE_TAG_MEM_WR_EN_R {
        L2_CACHE_TAG_MEM_WR_EN_R::new(((self.bits >> 21) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L2_CACHE_TAG_MEM_ACS_CONF")
            .field(
                "l2_cache_tag_mem_rd_en",
                &format_args!("{}", self.l2_cache_tag_mem_rd_en().bit()),
            )
            .field(
                "l2_cache_tag_mem_wr_en",
                &format_args!("{}", self.l2_cache_tag_mem_wr_en().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<L2_CACHE_TAG_MEM_ACS_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Cache tag memory access configure register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [l2_cache_tag_mem_acs_conf](index.html) module"]
pub struct L2_CACHE_TAG_MEM_ACS_CONF_SPEC;
impl crate::RegisterSpec for L2_CACHE_TAG_MEM_ACS_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [l2_cache_tag_mem_acs_conf::R](R) reader structure"]
impl crate::Readable for L2_CACHE_TAG_MEM_ACS_CONF_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets L2_CACHE_TAG_MEM_ACS_CONF to value 0"]
impl crate::Resettable for L2_CACHE_TAG_MEM_ACS_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
