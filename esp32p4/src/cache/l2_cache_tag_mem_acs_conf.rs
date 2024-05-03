#[doc = "Register `L2_CACHE_TAG_MEM_ACS_CONF` reader"]
pub type R = crate::R<L2_CACHE_TAG_MEM_ACS_CONF_SPEC>;
#[doc = "Register `L2_CACHE_TAG_MEM_ACS_CONF` writer"]
pub type W = crate::W<L2_CACHE_TAG_MEM_ACS_CONF_SPEC>;
#[doc = "Field `L2_CACHE_TAG_MEM_RD_EN` reader - The bit is used to enable config-bus read L2-Cache tag memoryory. 0: disable, 1: enable."]
pub type L2_CACHE_TAG_MEM_RD_EN_R = crate::BitReader;
#[doc = "Field `L2_CACHE_TAG_MEM_RD_EN` writer - The bit is used to enable config-bus read L2-Cache tag memoryory. 0: disable, 1: enable."]
pub type L2_CACHE_TAG_MEM_RD_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L2_CACHE_TAG_MEM_WR_EN` reader - The bit is used to enable config-bus write L2-Cache tag memoryory. 0: disable, 1: enable."]
pub type L2_CACHE_TAG_MEM_WR_EN_R = crate::BitReader;
#[doc = "Field `L2_CACHE_TAG_MEM_WR_EN` writer - The bit is used to enable config-bus write L2-Cache tag memoryory. 0: disable, 1: enable."]
pub type L2_CACHE_TAG_MEM_WR_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
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
                &self.l2_cache_tag_mem_rd_en().bit(),
            )
            .field(
                "l2_cache_tag_mem_wr_en",
                &self.l2_cache_tag_mem_wr_en().bit(),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<L2_CACHE_TAG_MEM_ACS_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 20 - The bit is used to enable config-bus read L2-Cache tag memoryory. 0: disable, 1: enable."]
    #[inline(always)]
    #[must_use]
    pub fn l2_cache_tag_mem_rd_en(
        &mut self,
    ) -> L2_CACHE_TAG_MEM_RD_EN_W<L2_CACHE_TAG_MEM_ACS_CONF_SPEC> {
        L2_CACHE_TAG_MEM_RD_EN_W::new(self, 20)
    }
    #[doc = "Bit 21 - The bit is used to enable config-bus write L2-Cache tag memoryory. 0: disable, 1: enable."]
    #[inline(always)]
    #[must_use]
    pub fn l2_cache_tag_mem_wr_en(
        &mut self,
    ) -> L2_CACHE_TAG_MEM_WR_EN_W<L2_CACHE_TAG_MEM_ACS_CONF_SPEC> {
        L2_CACHE_TAG_MEM_WR_EN_W::new(self, 21)
    }
}
#[doc = "Cache tag memory access configure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l2_cache_tag_mem_acs_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l2_cache_tag_mem_acs_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L2_CACHE_TAG_MEM_ACS_CONF_SPEC;
impl crate::RegisterSpec for L2_CACHE_TAG_MEM_ACS_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l2_cache_tag_mem_acs_conf::R`](R) reader structure"]
impl crate::Readable for L2_CACHE_TAG_MEM_ACS_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`l2_cache_tag_mem_acs_conf::W`](W) writer structure"]
impl crate::Writable for L2_CACHE_TAG_MEM_ACS_CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets L2_CACHE_TAG_MEM_ACS_CONF to value 0x0030_0000"]
impl crate::Resettable for L2_CACHE_TAG_MEM_ACS_CONF_SPEC {
    const RESET_VALUE: u32 = 0x0030_0000;
}
